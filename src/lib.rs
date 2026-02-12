pub mod button;
pub mod group;
pub mod sized;
pub mod slider;

pub mod absolute;
pub mod aligned;
pub mod animated;
pub mod background;
pub mod border;
pub mod centered;
pub mod checkbox;
pub mod child;
pub mod custom_material;
pub mod events;
pub mod gapped;
pub mod grid;
pub mod hover;
pub mod image;
pub mod justified;
pub mod margin;
pub mod on_event;
pub mod padded;
pub mod placeholder;
pub mod positioned;
pub mod scaled;
pub mod sibling;
pub mod sizing;
pub mod text;
pub mod theme;

pub use button::Button;
pub use group::Column;
pub use slider::Slider;
pub use text::{Text, TextSizing};

use std::{marker::PhantomData, sync::Arc};

use bevy::{
    app::{App, Plugin, Update},
    asset::Handle,
    color::Color,
    ecs::{
        bundle::Bundle,
        change_detection::{DetectChanges, DetectChangesMut},
        component::Component,
        entity::{Entity, EntityIndexSet},
        hierarchy::ChildOf,
        message::{Message, MessageReader},
        query::Added,
        relationship::RelatedSpawnerCommands,
        resource::Resource,
        schedule::{
            IntoScheduleConfigs, SystemSet,
            common_conditions::{resource_added, resource_exists},
        },
        system::{Commands, EntityCommands, Query, Res, ResMut},
        world::{FromWorld, Ref},
    },
    log::{info, warn},
    platform::collections::HashMap,
    prelude::{Deref, DerefMut},
    text::Font,
    ui::{BorderColor, BorderRadius, Node, UiRect, px},
};

use crate::{
    centered::Centered,
    checkbox::update_checkbox_style,
    child::Child,
    events::Init,
    placeholder::{InsertPlaceholderTraget, PlaceholderTarget},
    scaled::update_computed_size,
    sized::update_node_on_size_change,
    slider::update_slider_style,
    theme::Themed,
};

pub struct MenuPlugin<M: Component> {
    _pd: PhantomData<M>,
    root: Root,
}
impl<M: Component> Default for MenuPlugin<M> {
    fn default() -> Self {
        Self {
            _pd: Default::default(),
            root: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Root {
    root_element: Arc<Box<dyn ChildElementSpawner>>,
}

impl Root {
    pub fn with_root_node<E: IntoChildElementSpawner>(mut self, element: E) -> Self {
        self.root_element = Arc::new(element.into_element_spawner());
        self
    }
    pub fn set_root_element<E: IntoChildElementSpawner>(&mut self, element: E) {
        self.root_element = Arc::new(element.into_element_spawner());
    }
}

impl Default for Root {
    #[inline]
    fn default() -> Self {
        Self {
            root_element: Arc::new(
                Centered {
                    content: sized::Sized::expanded(Column::new(())),
                }
                .into_element_spawner(),
            ),
        }
    }
}

impl<M: Component> MenuPlugin<M> {
    pub fn set_root_element<E: IntoChildElementSpawner>(&mut self, element: E) {
        self.root.set_root_element(element);
    }
}

pub trait IntoChildElementSpawner {
    fn into_element_spawner(self) -> Box<dyn ChildElementSpawner>;
}
impl<E: Element + 'static> IntoChildElementSpawner for E {
    fn into_element_spawner(self) -> Box<dyn ChildElementSpawner> {
        Box::new(ElementSpawnerImpl { e: self })
    }
}

pub trait IntoChild {
    fn into_child(self) -> Child;
}

impl<E: Element + 'static> IntoChild for E {
    #[inline]
    fn into_child(self) -> Child {
        Child {
            content: self.into_element_spawner(),
        }
    }
}

pub trait Element: Send + Sync {
    type Bundle: Bundle;

    fn modify_node(&self, node: &mut Node, context: &UiContext);
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle;
    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext);
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>);
}

#[derive(Resource, Clone)]
pub struct UiContext {
    pub font: Handle<Font>,
    pub text_size: f32,
    pub text_size_big: f32,
    pub background_color: Color,
    pub foreground_color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub border_thickness: UiRect,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
    pub highlight_color: Color,
    pub image_color: Color,
    pub current_animator: Option<Entity>,
}

#[derive(Resource)]
pub struct UiFont(pub Handle<Font>);

impl FromWorld for UiContext {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        let font = world.get_resource::<UiFont>().unwrap();
        let text_size = 12.0;
        let text_size_big = 34.0;
        let bg = Color::linear_rgba(0.0, 0.0, 0.3, 0.8);
        let fg = Color::linear_rgba(1.0, 0.8, 0.8, 1.0);
        let hg = Color::linear_rgba(0.2, 0.2, 0.3, 0.8);
        let border_thickness = px(2.0);
        let text_c = Color::linear_rgba(0.3, 0.6, 0.2, 1.0);
        let border_c = Color::BLACK;
        Self {
            font: font.0.clone(),
            text_size,
            text_size_big,
            background_color: bg,
            foreground_color: fg,
            hover_color: hg,
            text_color: text_c,
            border_thickness: UiRect::all(border_thickness),
            border_color: BorderColor::all(border_c),
            border_radius: BorderRadius::MAX,
            highlight_color: Color::linear_rgb(1.0, 0.0, 0.0),
            image_color: Color::WHITE,
            current_animator: None,
        }
    }
}

impl UiContext {
    pub fn with_animator(self: Arc<UiContext>, entity: Option<Entity>) -> Arc<UiContext> {
        let mut s = (*self).clone();
        s.current_animator = entity;
        Arc::new(s)
    }
}

pub trait ChildElementSpawner: Send + Sync {
    fn spawn(&self, commands: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>);
    fn insert_root(&self, commands: &mut EntityCommands, context: Arc<UiContext>);
}
pub struct ElementSpawnerImpl<E: Element> {
    e: E,
}

impl<E: Element> ChildElementSpawner for ElementSpawnerImpl<E> {
    fn spawn(&self, commands: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        let mut node = Node::default();
        self.e.modify_node(&mut node, &context);
        let themed = Themed::from(context.clone());
        let mut ec = commands.spawn((node, self.e.create_bundle(&context)));
        ec.insert_if_new(themed);
        self.e.register_observers(&mut ec, &context);
        ec.with_children(|rcs| {
            self.e.spawn_children(rcs, context);
        });
        ec.trigger(|e| Init { entity: e });
    }

    fn insert_root(&self, commands: &mut EntityCommands, context: Arc<UiContext>) {
        let mut node = Node::default();
        self.e.modify_node(&mut node, &context);
        let themed = Themed::from(context.clone());
        let ec = commands.insert_if_new((node, self.e.create_bundle(&context)));
        ec.insert_if_new(themed);
        self.e.register_observers(ec, &context);
        ec.with_children(|rcs| {
            self.e.spawn_children(rcs, context);
        });
        ec.trigger(|e| Init { entity: e });
    }
}

#[derive(Resource)]
pub struct Menu<M: Component> {
    root: Root,
    _pd: PhantomData<M>,
}

#[derive(Message)]
pub struct DestroyMenu<M: Component> {
    pub target: Entity,
    _pd: PhantomData<M>,
}

impl<M: Component> DestroyMenu<M> {
    pub fn new(entity: Entity) -> Self {
        Self {
            target: entity,
            _pd: Default::default(),
        }
    }
}

impl<M: Component + Default> Plugin for MenuPlugin<M> {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(JustRemovedEntities(EntityIndexSet::new()));
        let menu = Menu::<M> {
            root: self.root.clone(),
            _pd: Default::default(),
        };

        app.add_message::<DestroyMenu<M>>();
        app.add_systems(
            Update,
            show_menu_function::<M>
                .run_if(resource_exists::<UiContext>)
                .in_set(UiSystems::Add),
        );

        app.insert_resource(menu);
        app.add_systems(Update, cleanup::<M>.in_set(UiSystems::Remove));

        app.add_systems(Update, clear_just_added.in_set(UiSystems::Finish));
        if !app.is_plugin_added::<SharedMenuStatePlugin>() {
            app.add_plugins(SharedMenuStatePlugin);
        }
    }
}

fn cleanup<C: Component>(
    mut commands: Commands,
    mut messages: MessageReader<DestroyMenu<C>>,
    mut just_removed: ResMut<JustRemovedEntities>,
) {
    if messages.is_empty() {
        return;
    }
    for e in messages.read() {
        info!("cleaning up: {:?}", e.target);
        just_removed.0.insert(e.target);
        commands.entity(e.target).despawn();
    }
}

struct SharedMenuStatePlugin;

impl Plugin for SharedMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlaceHolders(HashMap::new()));
        app.add_systems(Update, update_slider_style);
        app.add_systems(
            Update,
            (
                update_checkbox_style,
                (update_placeholder, move_to_placeholder_target)
                    .chain()
                    .in_set(UiSystems::Add),
            ),
        );
        app.add_systems(
            Update,
            (update_computed_size, update_node_on_size_change)
                .chain()
                .in_set(UiSystems::Add),
        );
        app.add_systems(
            Update,
            init_resource::<UiContext>.run_if(resource_added::<UiFont>),
        );
        app.configure_sets(
            Update,
            (UiSystems::Remove, UiSystems::Add, UiSystems::Finish).chain(),
        );
    }
}

fn init_resource<R: Resource + FromWorld>(mut commands: Commands) {
    info!("init resource");
    commands.init_resource::<R>();
}

#[derive(Resource, Deref, DerefMut)]
struct PlaceHolders(HashMap<String, Entity>);

fn update_placeholder(
    mut placeholders: ResMut<PlaceHolders>,
    query: Query<(Entity, &PlaceholderTarget), Added<PlaceholderTarget>>,
) {
    for (e, pt) in query {
        placeholders.insert(pt.0.clone(), e);
    }
}

fn move_to_placeholder_target(
    inserts: Query<(Entity, Ref<InsertPlaceholderTraget>)>,
    placeholders: Res<PlaceHolders>,
    mut commands: Commands,
) {
    for (e, i) in inserts {
        if i.is_added() || placeholders.is_changed() {
            let Some(p) = placeholders.get(&i.0).cloned() else {
                warn!("placeholder target not found");
                continue;
            };
            info!("placing as child of placeholder");
            commands.entity(e).insert(ChildOf(p));
        }
    }
}

fn show_menu_function<M: Component>(
    mut commands: Commands,
    menu: Res<Menu<M>>,
    context: Res<UiContext>,
    query: Query<Entity, Added<M>>,
    just_removed: Res<JustRemovedEntities>,
) {
    for e in query {
        if just_removed.0.contains(&e) {
            info!("menu already destroyed");
            continue;
        }
        let arc = Arc::new(context.clone());

        menu.root
            .root_element
            .insert_root(&mut commands.entity(e), arc);
    }
}

fn clear_just_added(mut just_removed: ResMut<JustRemovedEntities>) {
    if just_removed.is_changed() {
        just_removed.bypass_change_detection().0.clear();
    }
}

#[derive(Resource)]
pub struct JustRemovedEntities(EntityIndexSet);

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
pub enum UiSystems {
    Remove,
    Add,
    Finish,
}

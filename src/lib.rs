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
pub mod hover;
pub mod image;
pub mod justified;
pub mod on_event;
pub mod padded;
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
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        message::Message,
        query::{Added, With},
        relationship::RelatedSpawnerCommands,
        resource::Resource,
        schedule::{
            IntoScheduleConfigs,
            common_conditions::{on_message, resource_added},
        },
        system::{Commands, EntityCommands, Query, Res},
        world::FromWorld,
    },
    log::info,
    text::Font,
    ui::{BorderColor, BorderRadius, Node, UiRect, px},
};

use crate::{
    centered::Centered, checkbox::update_checkbox_style, child::Child, events::Init,
    scaled::update_computed_size, sized::update_node_on_size_change, slider::update_slider_style,
    theme::Themed,
};

#[derive(Default)]
pub struct MenuPlugin<M: Component> {
    _pd: PhantomData<M>,
    root: Root,
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
                    content: sized::Sized::expanded(Column::new()),
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
    font: Handle<Font>,
    text_size: f32,
    text_size_big: f32,
    background_color: Color,
    foreground_color: Color,
    hover_color: Color,
    text_color: Color,
    border_thickness: UiRect,
    border_color: BorderColor,
    border_radius: BorderRadius,
    highlight_color: Color,
    image_color: Color,
    current_animator: Option<Entity>,
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
}

#[derive(Resource)]
pub struct Menu<M: Component> {
    root: Root,
    _pd: PhantomData<M>,
}

#[derive(Message)]
pub struct DestroyMenu<M: Component> {
    _pd: PhantomData<M>,
}

impl<M: Component> Default for DestroyMenu<M> {
    fn default() -> Self {
        Self {
            _pd: Default::default(),
        }
    }
}
impl<M: Component + Default> Plugin for MenuPlugin<M> {
    fn build(&self, app: &mut bevy::app::App) {
        let menu = Menu::<M> {
            root: self.root.clone(),
            _pd: Default::default(),
        };

        app.add_message::<DestroyMenu<M>>();
        app.add_systems(Update, show_menu_function::<M>);

        app.insert_resource(menu);
        app.add_systems(Update, cleanup::<M>.run_if(on_message::<DestroyMenu<M>>));
        if !app.is_plugin_added::<SharedMenuStatePlugin>() {
            app.add_plugins(SharedMenuStatePlugin);
        }
    }
}

fn cleanup<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for e in query {
        commands.entity(e).despawn();
    }
}

struct SharedMenuStatePlugin;

impl Plugin for SharedMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_slider_style);
        app.add_systems(Update, update_checkbox_style);
        app.add_systems(Update, update_computed_size);
        app.add_systems(Update, update_node_on_size_change);
        app.add_systems(
            Update,
            init_resource::<UiContext>.run_if(resource_added::<UiFont>),
        );
    }
}

fn init_resource<R: Resource + FromWorld>(mut commands: Commands) {
    info!("init resource");
    commands.init_resource::<R>();
}

fn show_menu_function<M: Component>(
    mut commands: Commands,
    menu: Res<Menu<M>>,
    context: Res<UiContext>,
    query: Query<Entity, Added<M>>,
) {
    for e in query {
        info!("showing menu");
        let arc = Arc::new(context.clone());
        commands.entity(e).with_children(|r| {
            menu.root.root_element.spawn(r, arc);
        });
    }
}

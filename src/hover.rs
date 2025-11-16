use std::sync::Arc;

use bevy::{
    ecs::{
        hierarchy::ChildOf,
        lifecycle::Insert,
        observer::On,
        relationship::RelatedSpawnerCommands,
        system::{EntityCommands, IntoObserverSystem, Query},
    },
    log::info,
    picking::hover::Hovered,
    ui::{BackgroundColor, Node},
};
use wane_observers::{EntityObserverRegistration, IntoEntityObserverRegistration};

use crate::{Element, UiContext, theme::Themed};

pub struct Hover<E: Element> {
    on_hover: Box<dyn EntityObserverRegistration>,
    content: E,
}

impl<E: Element> Hover<E> {
    #[inline]
    pub fn new<'a, F: Send + Sync, M: 'static>(on_hover: &'a F, content: E) -> Self
    where
        &'a F: IntoObserverSystem<Insert, Hovered, M>,
    {
        Self {
            on_hover: Box::new(on_hover.into_registration()),
            content: content,
        }
    }
}

impl<E: Element> Element for Hover<E> {
    type Bundle = (Hovered, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (Hovered::default(), self.content.create_bundle(context))
    }

    #[inline]
    fn register_observers(&self, entity_commands: &mut EntityCommands, context: &UiContext) {
        self.on_hover.register_observer(entity_commands);
        self.content.register_observers(entity_commands, context);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }
}

pub fn default_on_hover(
    event: On<Insert, Hovered>,
    mut query: Query<(&mut BackgroundColor, &Hovered, &Themed)>,
) {
    let Ok((mut bg, hovered, theme)) = query.get_mut(event.entity) else {
        info!("BackgroundColor not found");
        return;
    };
    if hovered.get() {
        bg.0 = theme.context.hover_color;
    } else {
        bg.0 = theme.context.background_color;
    }
}

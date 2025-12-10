use std::sync::Arc;

use bevy::ui::{BackgroundColor, Node};

use crate::{Element, UiContext};

pub struct Background<E: Element> {
    pub content: E,
}

impl<E: Element> Background<E> {
    #[inline]
    pub fn new(content: E) -> Self {
        Background { content }
    }
}

impl<E: Element> Element for Background<E> {
    type Bundle = (BackgroundColor, E::Bundle);

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (
            BackgroundColor(context.background_color),
            self.content.create_bundle(context),
        )
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &super::UiContext,
    ) {
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: Arc<UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }
}

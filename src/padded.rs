use std::sync::Arc;

use bevy::ui::{Node, UiRect};

use crate::{Element, UiContext};

pub struct Padded<E: Element> {
    pub content: E,
    pub padding: UiRect,
}

impl<E: Element> Element for Padded<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.padding = self.padding;
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &UiContext,
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

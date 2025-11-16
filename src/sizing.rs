use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::{BoxSizing, Node},
};

use crate::{Element, UiContext};

pub struct Sizing<E: Element> {
    pub content: E,
    pub sizing: BoxSizing,
}

impl<E: Element> Element for Sizing<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(&self, entity_commands: &mut EntityCommands, context: &UiContext) {
        self.content.register_observers(entity_commands, context);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.box_sizing = self.sizing;
        self.content.modify_node(node, context);
    }
}

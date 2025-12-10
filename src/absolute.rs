use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::Node,
};

use crate::{Element, UiContext};

pub struct Absolute<E: Element> {
    pub content: E,
}

impl<E: Element> Absolute<E> {
    #[inline]
    pub fn new(content: E) -> Self {
        Self { content }
    }
}

impl<E: Element> Element for Absolute<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext) {
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut RelatedSpawnerCommands<ChildOf>,
        context: std::sync::Arc<UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.position_type = bevy::ui::PositionType::Absolute;
        self.content.modify_node(node, context);
    }
}

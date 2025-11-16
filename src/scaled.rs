use std::sync::Arc;

use bevy::{ecs::component::Component, math::Vec2, ui::Node};

use crate::{Element, UiContext};

pub struct Scale<E: Element> {
    pub scale: Vec2,
    pub content: E,
}

impl<E: Element> Element for Scale<E> {
    type Bundle = (Scaled, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {
        (
            Scaled { scale: self.scale },
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

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        self.content.modify_node(node, context);
        node.width = node.width * self.scale.x;
        node.height = node.height * self.scale.y;
    }
}

#[derive(Component)]
pub struct Scaled {
    pub scale: Vec2,
}

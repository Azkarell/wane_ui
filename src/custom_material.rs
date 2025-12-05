use bevy::{
    asset::Handle,
    ui_render::prelude::{MaterialNode, UiMaterial},
};

use crate::{Element, IntoChild, child::Child};

pub struct CustomNode<M: UiMaterial> {
    material: Handle<M>,
    content: Child,
}

impl<M: UiMaterial> CustomNode<M> {
    #[inline]
    #[track_caller]
    pub fn new<E: IntoChild>(content: E) -> Self {
        CustomNode {
            material: Handle::default(),
            content: content.into_child(),
        }
    }
}

impl<M: UiMaterial> Element for CustomNode<M> {
    type Bundle = MaterialNode<M>;

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &super::UiContext) {
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, _context: &super::UiContext) -> Self::Bundle {
        MaterialNode(self.material.clone())
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
        context: std::sync::Arc<super::UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }
}

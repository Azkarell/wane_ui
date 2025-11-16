use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::Node,
};

use crate::{ChildElementSpawner, Element, UiContext};

pub struct Child {
    pub content: Box<dyn ChildElementSpawner>,
}
impl Element for Child {
    type Bundle = ();

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {}

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {}

    #[inline]
    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext) {}

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn(rcs, context);
    }
}

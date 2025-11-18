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
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}

    #[inline]
    fn create_bundle(&self, _context: &UiContext) -> Self::Bundle {}

    #[inline]
    fn register_observers(&self, _entity_command: &mut EntityCommands, _context: &UiContext) {}

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn(rcs, context);
    }
}

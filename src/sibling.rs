use crate::{Element, child::Child};

impl Element for (Child, Child) {
    type Bundle = ();

    fn modify_node(&self, node: &mut bevy::ui::Node, context: &super::UiContext) {}

    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {}

    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &super::UiContext,
    ) {
    }

    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: std::sync::Arc<super::UiContext>,
    ) {
        self.0.spawn_children(rcs, context.clone());
        self.1.spawn_children(rcs, context);
    }
}

use crate::Element;

pub struct Grid<E: Element> {
    content: E,
}

impl<E: Element> Element for Grid<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &crate::UiContext) {
        node.display = bevy::ui::Display::Grid;
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &crate::UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &crate::UiContext,
    ) {
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: std::sync::Arc<crate::UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }
}

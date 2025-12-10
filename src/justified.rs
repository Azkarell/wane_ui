use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::{JustifyContent, Node},
};

use crate::{Element, UiContext};

pub struct Justified<E: Element> {
    pub content: E,
    pub justify_content: JustifyContent,
}

impl<E: Element> Justified<E> {
    #[inline]
    pub fn new(content: E) -> Self {
        Self {
            content,
            justify_content: JustifyContent::Center,
        }
    }

    #[inline]
    pub fn centered(mut self) -> Self {
        self.justify_content = JustifyContent::Center;
        self
    }

    #[inline]
    pub fn start(mut self) -> Self {
        self.justify_content = JustifyContent::Start;
        self
    }

    #[inline]
    pub fn end(mut self) -> Self {
        self.justify_content = JustifyContent::End;
        self
    }
}

impl<E: Element> Element for Justified<E> {
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
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.justify_content = self.justify_content;
        self.content.modify_node(node, context);
    }
}

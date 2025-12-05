use std::panic::Location;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::{AlignItems, Node},
};

use crate::{Element, UiContext};

pub struct Aligned<E: Element> {
    pub content: E,

    pub align_items: AlignItems,
}

impl<E: Element> Aligned<E> {
    #[inline]
    #[track_caller]
    pub fn new(content: E) -> Self {
        Self {
            content,

            align_items: AlignItems::Center,
        }
    }
    #[inline]
    pub fn centered(mut self) -> Self {
        self.align_items = AlignItems::Center;
        self
    }

    #[inline]
    pub fn start(mut self) -> Self {
        self.align_items = AlignItems::Start;
        self
    }

    #[inline]
    pub fn end(mut self) -> Self {
        self.align_items = AlignItems::End;
        self
    }
}

impl<E: Element> Element for Aligned<E> {
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
        node.align_items = self.align_items;
        self.content.modify_node(node, context);
    }
}

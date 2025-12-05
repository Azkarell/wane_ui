use std::{panic::Location, sync::Arc};

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    ui::{Node, Val},
};

use crate::{Element, UiContext};

pub struct Gapped<E: Element> {
    pub content: E,
    pub column: Val,
    pub row: Val,
}

impl<E: Element> Gapped<E> {
    #[inline]
    #[track_caller]
    pub fn new(content: E) -> Self {
        Self {
            content,
            column: Val::DEFAULT,
            row: Val::DEFAULT,
        }
    }

    #[inline]
    pub fn with_value(mut self, val: Val) -> Self {
        self.column = val;
        self.row = val;
        self
    }
    #[inline]
    pub fn column_gap(mut self, val: Val) -> Self {
        self.column = val;
        self
    }
    #[inline]
    pub fn row_gap(mut self, val: Val) -> Self {
        self.row = val;
        self
    }
}

impl<E: Element> Element for Gapped<E> {
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
        node.column_gap = self.column;
        node.row_gap = self.row;
        self.content.modify_node(node, context);
    }
}

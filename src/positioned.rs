use std::sync::Arc;

use bevy::ui::{Node, Val};

use crate::{Element, UiContext};

pub struct Positioned<E: Element> {
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
    pub content: E,
}

impl<E: Element> Positioned<E> {
    #[inline]
    pub fn all(val: Val, content: E) -> Self {
        Self {
            left: val,
            right: val,
            top: val,
            bottom: val,
            content,
        }
    }
    #[inline]
    pub fn left(val: Val, content: E) -> Self {
        Self {
            left: val,
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
            content,
        }
    }

    #[inline]
    pub fn right(val: Val, content: E) -> Self {
        Self {
            left: Val::Auto,
            right: val,
            top: Val::Auto,
            bottom: Val::Auto,
            content,
        }
    }

    #[inline]
    pub fn top(val: Val, content: E) -> Self {
        Self {
            left: Val::Auto,
            right: Val::Auto,
            top: val,
            bottom: Val::Auto,
            content,
        }
    }

    #[inline]
    pub fn bottom(val: Val, content: E) -> Self {
        Self {
            left: Val::Auto,
            right: Val::Auto,
            top: Val::Auto,
            bottom: val,
            content,
        }
    }
}

impl<E: Element> Element for Positioned<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
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
        node.left = self.left;
        node.right = self.right;
        node.top = self.top;
        node.bottom = self.bottom;
    }
}

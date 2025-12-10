use std::sync::Arc;

use bevy::{
    ecs::{component::Component, query::Changed, system::Query},
    ui::{Node, Val, percent},
};

use crate::{Element, UiContext};

pub struct Sized<E: Element> {
    pub width: Val,
    pub height: Val,
    pub content: E,
}

impl Element for () {
    type Bundle = ();

    #[inline]
    fn create_bundle(&self, _context: &super::UiContext) -> Self::Bundle {}

    #[inline]
    fn register_observers(
        &self,
        _entity_command: &mut bevy::ecs::system::EntityCommands,
        _context: &super::UiContext,
    ) {
    }

    #[inline]
    fn spawn_children(
        &self,
        _rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        _context: Arc<UiContext>,
    ) {
    }

    #[inline]
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}
}

impl Sized<()> {
    #[inline]
    pub fn empty_sized(width: Val, height: Val) -> Self {
        Self {
            width,
            height,
            content: (),
        }
    }
    #[inline]
    pub fn empty() -> Self {
        Self {
            width: Val::Auto,
            height: Val::Auto,
            content: (),
        }
    }

    #[inline]
    pub fn expand() -> Self {
        Self {
            width: percent(100),
            height: percent(100),
            content: (),
        }
    }
}

impl<E: Element> Sized<E> {
    #[inline]
    pub fn new(width: Val, height: Val, content: E) -> Self {
        Self {
            width,
            height,
            content,
        }
    }
    #[inline]
    pub fn expanded(content: E) -> Self {
        Self {
            width: percent(100),
            height: percent(100),
            content,
        }
    }
}

impl<E: Element> Element for Sized<E> {
    type Bundle = (ComputedSize, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {
        (
            ComputedSize {
                original_height: self.height,
                original_width: self.width,
                computed_height: self.height,
                computed_width: self.width,
            },
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
        node.width = self.width;
        node.height = self.height;
        self.content.modify_node(node, context);
    }
}

#[derive(Component)]
pub struct ComputedSize {
    pub original_width: Val,
    pub original_height: Val,
    pub computed_width: Val,
    pub computed_height: Val,
}

pub(crate) fn update_node_on_size_change(
    query: Query<(&mut Node, &ComputedSize), Changed<ComputedSize>>,
) {
    for (mut n, c) in query {
        n.width = c.computed_width;
        n.height = c.computed_height;
    }
}

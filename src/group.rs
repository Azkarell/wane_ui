use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands},
    ui::{FlexDirection, Node},
};

use crate::{ChildElementSpawner, Element, IntoChildElementSpawner, UiContext};

pub struct Column<E: Element> {
    content: E,
    children: Vec<Box<dyn ChildElementSpawner>>,
}

impl<E: Element> Element for Column<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &UiContext,
    ) {
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context.clone());
        for c in &self.children {
            c.spawn(rcs, context.clone());
        }
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.flex_direction = FlexDirection::Column;
        node.grid_auto_flow = bevy::ui::GridAutoFlow::Column;
        self.content.modify_node(node, context);
    }
}

impl<E: Element> Column<E> {
    #[inline]
    pub fn new(content: E) -> Self {
        Self {
            children: vec![],
            content,
        }
    }
    #[inline]
    pub fn with_element<I: IntoChildElementSpawner>(mut self, e: I) -> Self {
        self.children.push(e.into_element_spawner());
        self
    }
    #[inline]
    pub fn add_element<I: IntoChildElementSpawner>(&mut self, e: I) {
        self.children.push(e.into_element_spawner());
    }
}
pub struct Row<E: Element> {
    children: Vec<Box<dyn ChildElementSpawner>>,
    content: E,
}

impl<E: Element> Element for Row<E> {
    type Bundle = E::Bundle;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        self.content.create_bundle(context)
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &UiContext,
    ) {
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context.clone());
        for c in &self.children {
            c.spawn(rcs, context.clone());
        }
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        node.flex_direction = FlexDirection::Row;
        node.grid_auto_flow = bevy::ui::GridAutoFlow::Row;
        self.content.modify_node(node, context);
    }
}

impl<E: Element> Row<E> {
    #[inline]
    pub fn new(content: E) -> Self {
        Self {
            children: vec![],
            content,
        }
    }
    #[inline]
    pub fn with_element<I: IntoChildElementSpawner>(mut self, e: I) -> Self {
        self.children.push(e.into_element_spawner());
        self
    }
    #[inline]
    pub fn add_element<I: IntoChildElementSpawner>(&mut self, e: I) {
        self.children.push(e.into_element_spawner());
    }
}

use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands},
    ui::{FlexDirection, Node},
};

use crate::{ChildElementSpawner, Element, IntoChildElementSpawner, UiContext};

pub struct Column {
    elements: Vec<Box<dyn ChildElementSpawner>>,
}

impl Element for Column {
    type Bundle = ();

    #[inline]
    fn create_bundle(&self, _context: &UiContext) -> Self::Bundle {}

    #[inline]
    fn register_observers(
        &self,
        _entity_command: &mut bevy::ecs::system::EntityCommands,
        _context: &UiContext,
    ) {
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        for c in &self.elements {
            c.spawn(rcs, context.clone());
        }
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, _context: &UiContext) {
        node.flex_direction = FlexDirection::Column;
    }
}

impl Column {
    #[inline]
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    #[inline]
    pub fn with_element<E: IntoChildElementSpawner>(mut self, e: E) -> Self {
        self.elements.push(e.into_element_spawner());
        self
    }
    #[inline]
    pub fn add_element<E: IntoChildElementSpawner>(&mut self, e: E) {
        self.elements.push(e.into_element_spawner());
    }
}
pub struct Row {
    elements: Vec<Box<dyn ChildElementSpawner>>,
}

impl Element for Row {
    type Bundle = ();

    #[inline]
    fn create_bundle(&self, _context: &UiContext) -> Self::Bundle {}

    #[inline]
    fn register_observers(
        &self,
        _entity_command: &mut bevy::ecs::system::EntityCommands,
        _context: &UiContext,
    ) {
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        for c in &self.elements {
            c.spawn(rcs, context.clone());
        }
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, _context: &UiContext) {
        node.flex_direction = FlexDirection::Row;
    }
}

impl Row {
    #[inline]
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    #[inline]
    pub fn with_element<E: IntoChildElementSpawner>(mut self, e: E) -> Self {
        self.elements.push(e.into_element_spawner());
        self
    }
    #[inline]
    pub fn add_element<E: IntoChildElementSpawner>(&mut self, e: E) {
        self.elements.push(e.into_element_spawner());
    }
}

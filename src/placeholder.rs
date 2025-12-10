use bevy::{
    ecs::{component::Component, entity::Entity, event::EntityEvent},
    reflect::Reflect,
};

use crate::Element;

pub struct Placeholder {
    pub name: String,
}

#[derive(Component, Reflect)]
pub struct PlaceholderTarget(pub String);

impl Placeholder {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

impl Element for Placeholder {
    type Bundle = PlaceholderTarget;

    fn modify_node(&self, _node: &mut bevy::ui::Node, _context: &crate::UiContext) {}

    fn create_bundle(&self, _context: &crate::UiContext) -> Self::Bundle {
        PlaceholderTarget(self.name.clone())
    }

    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        _context: &crate::UiContext,
    ) {
        entity_command.trigger(|e| PlaceholderCreated {
            entity: e,
            name: self.name.clone(),
        });
    }

    fn spawn_children(
        &self,
        _rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        _context: std::sync::Arc<crate::UiContext>,
    ) {
    }
}

#[derive(EntityEvent)]
pub struct PlaceholderCreated {
    pub entity: Entity,
    pub name: String,
}

use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct Init {
    pub entity: Entity,
}

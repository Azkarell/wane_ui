use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(EntityEvent)]
pub struct Init {
    pub entity: Entity,
}

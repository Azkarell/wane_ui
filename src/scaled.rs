use std::sync::Arc;

use bevy::{
    ecs::{
        component::Component,
        query::{Changed, Or},
        system::Query,
    },
    math::Vec2,
    reflect::{Reflect, TypePath},
    ui::{Node, Val},
};

use crate::{Element, UiContext, sized::ComputedSize};

pub struct Scale<E: Element> {
    pub scale: Vec2,
    pub content: E,
}

impl<E: Element> Element for Scale<E> {
    type Bundle = (Scaled, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {
        (
            Scaled {
                scale: self.scale,
                original_scale: self.scale,
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
        self.content.modify_node(node, context);
        node.width = node.width * self.scale.x;
        node.height = node.height * self.scale.y;
    }
}

#[derive(Component, Reflect, Clone)]
pub struct Scaled {
    pub scale: Vec2,
    pub original_scale: Vec2,
}
impl Scaled {
    pub fn reset(&mut self) {
        self.scale = self.original_scale;
    }
}
pub(crate) fn update_computed_size(
    query: Query<
        (&mut ComputedSize, Option<&Scaled>),
        Or<(Changed<Scaled>, Changed<ComputedSize>)>,
    >,
) {
    for (mut cs, s) in query {
        cs.computed_width = cs.original_width * s.map(|x| x.scale.x).unwrap_or(1.0);
        cs.computed_height = cs.original_height * s.map(|x| x.scale.y).unwrap_or(1.0);
    }
}

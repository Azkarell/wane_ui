use std::{marker::PhantomData, sync::Arc};

use bevy::{
    ecs::{event::EntityEvent, system::IntoObserverSystem},
    log::info,
};
use wane_observers::{EntityObserverRegistration, IntoEntityObserverRegistration};

use crate::{Element, UiContext};

pub struct OnEvent<E: Element, Ev: EntityEvent> {
    content: E,
    on_event: Box<dyn EntityObserverRegistration>,
    _pd: PhantomData<Ev>,
}

impl<E: Element, Ev: EntityEvent> OnEvent<E, Ev> {
    #[inline]
    pub fn new<F, M: 'static>(on_event: F, content: E) -> Self
    where
        F: IntoObserverSystem<Ev, (), M> + Copy + Send + Sync,
    {
        Self {
            on_event: Box::new(on_event.into_registration()),
            content,
            _pd: Default::default(),
        }
    }
}

impl<E: Element, Ev: EntityEvent> Element for OnEvent<E, Ev> {
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
        info!("registering_event");
        self.on_event.register_observer(entity_command);
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
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }
}

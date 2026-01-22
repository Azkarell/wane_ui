use std::sync::Arc;

use bevy::{
    ecs::{
        hierarchy::ChildOf,
        relationship::RelatedSpawnerCommands,
        system::{EntityCommands, IntoObserverSystem},
    },
    ui::{Node, Val},
    ui_widgets::{Activate, Button as UiButton},
};

use crate::{
    Element, IntoChild, Text, TextSizing, UiContext, centered::Centered, child::Child, sized::Sized,
};
use wane_observers::{EntityObserverRegistration, IntoEntityObserverRegistration};

pub struct Button {
    on_click: Box<dyn EntityObserverRegistration>,
    content: Child,
}

impl Button {
    #[inline]
    pub fn new<F, M: 'static, C: IntoChild>(on_click: F, content: C) -> Self
    where
        F: IntoObserverSystem<Activate, (), M> + Copy + Send + Sync,
    {
        Self {
            on_click: Box::new(on_click.into_registration()),
            content: content.into_child(),
        }
    }
}

impl Element for Button {
    type Bundle = UiButton;

    #[inline]
    fn create_bundle(&self, _context: &UiContext) -> Self::Bundle {
        UiButton
    }

    #[inline]
    fn register_observers(&self, entity_command: &mut EntityCommands, _context: &UiContext) {
        self.on_click.register_observer(entity_command);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}
}

pub fn button_with_text<F, M: 'static>(
    text: String,
    width: Val,
    height: Val,
    on_click: F,
) -> impl Element
where
    F: IntoObserverSystem<Activate, (), M> + Copy + Send + Sync,
{
    Sized {
        width,
        height,
        content: Centered {
            content: Button::new(
                on_click,
                Text {
                    text,
                    sizing: TextSizing::Big,
                }
                .into_child(),
            ),
        },
    }
}

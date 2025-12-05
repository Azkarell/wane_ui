use std::sync::Arc;

use bevy::{
    ecs::{
        hierarchy::ChildOf,
        lifecycle::Insert,
        observer::On,
        relationship::RelatedSpawnerCommands,
        system::{EntityCommands, IntoObserverSystem, Query},
    },
    log::info,
    picking::hover::Hovered,
    ui::{BackgroundColor, Node, Val},
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
    #[track_caller]
    pub fn new<F: Send + Sync, M: 'static, C: IntoChild>(on_click: F, content: C) -> Self
    where
        F: IntoObserverSystem<Activate, (), M> + Copy,
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
    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext) {
        let bg_c = context.background_color;
        let hover_c = context.hover_color;
        entity_command.observe(
            move |event: On<Insert, Hovered>,
                  mut query: Query<(&mut BackgroundColor, &Hovered)>| {
                let Ok((mut bg, hovered)) = query.get_mut(event.entity) else {
                    info!("BackgroundColor not found");
                    return;
                };
                if hovered.get() {
                    bg.0 = hover_c;
                } else {
                    bg.0 = bg_c;
                }
            },
        );
        self.on_click.register_observer(entity_command);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}
}

#[track_caller]
pub fn button_with_text<F: Send + Sync, M: 'static>(
    text: String,
    width: Val,
    height: Val,
    on_click: F,
) -> impl Element
where
    F: IntoObserverSystem<Activate, (), M> + Copy,
{
    Sized {
        width,
        height,
        content: Centered {
            content: Button::new(
                on_click,
                Text {
                    text: text,
                    sizing: TextSizing::Big,
                }
                .into_child(),
            ),
        },
    }
}

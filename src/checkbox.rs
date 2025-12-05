use std::{panic::Location, sync::Arc};

use bevy::{
    ecs::{
        hierarchy::Children,
        query::{Has, With, Without},
        system::{If, IntoObserverSystem, Query, Res},
    },
    log::warn,
    ui::{BackgroundColor, BorderColor, Checked, px},
    ui_widgets::{Checkbox as UiCheckbox, ValueChange, checkbox_self_update},
};
use wane_observers::{EntityObserverRegistration, IntoEntityObserverRegistration};

use crate::{
    Element, IntoChild, UiContext, background::Background, border::Border, centered::Centered,
    sized::Sized,
};

pub struct Checkbox<E: Element> {
    on_change: Box<dyn EntityObserverRegistration>,
    content: E,
}
impl Checkbox<()> {
    #[track_caller]
    pub fn new_default<'a, F: Send + Sync, M: 'static>(on_change: &'a F) -> impl Element
    where
        &'a F: IntoObserverSystem<ValueChange<bool>, (), M>,
    {
        Checkbox {
            on_change: Box::new(on_change.into_registration()),
            content: Centered {
                content: Border::all(Sized {
                    content: Background {
                        content: Background {
                            content: Centered {
                                content: Sized {
                                    width: px(8),
                                    height: px(8),
                                    content: (),
                                },
                            },
                        }
                        .into_child(),
                    },
                    width: px(16),
                    height: px(16),
                }),
            }
            .into_child(),
        }
    }
}

impl<E: Element> Checkbox<E> {
    #[inline]
    #[track_caller]
    pub fn new<'a, F: Send + Sync, M: 'static>(on_change: &'a F, content: E) -> Self
    where
        &'a F: IntoObserverSystem<ValueChange<bool>, (), M>,
    {
        Self {
            on_change: Box::new(on_change.into_registration()),
            content,
        }
    }
}

impl<E: Element> Element for Checkbox<E> {
    type Bundle = (UiCheckbox, E::Bundle);

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &super::UiContext) {
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &super::UiContext) -> Self::Bundle {
        (UiCheckbox, self.content.create_bundle(context))
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &super::UiContext,
    ) {
        self.on_change.register_observer(entity_command);
        entity_command.observe(checkbox_self_update);
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
}

pub fn update_checkbox_style(
    mut q_checkbox: Query<(Has<Checked>, &Children), With<UiCheckbox>>,
    mut q_border: Query<&Children, With<BorderColor>>,
    mut q_color: Query<&mut BackgroundColor, Without<Children>>,
    context: If<Res<UiContext>>,
) {
    for (checked, children) in q_checkbox.iter_mut() {
        let Some(border_id) = children.first() else {
            continue;
        };

        let Ok(border_children) = q_border.get_mut(*border_id) else {
            continue;
        };

        let Some(mark_id) = border_children.first() else {
            warn!("Checkbox does not have a mark entity.");
            continue;
        };

        let Ok(mut mark_bg) = q_color.get_mut(*mark_id) else {
            warn!("Checkbox mark entity lacking a background color.");
            continue;
        };
        if checked {
            mark_bg.0 = context.highlight_color;
        } else {
            mark_bg.0 = context.foreground_color;
        }
    }
}

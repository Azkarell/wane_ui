use std::sync::Arc;

use bevy::{
    color::Color,
    ecs::{
        entity::Entity,
        event::EntityEvent,
        hierarchy::{ChildOf, Children},
        observer::On,
        query::{Added, Changed, Has, Or, With, Without},
        relationship::RelatedSpawnerCommands,
        system::{Commands, EntityCommands, IntoObserverSystem, Query},
    },
    log::info,
    math::Vec2,
    ui::{BackgroundColor, InteractionDisabled, Node, Val, percent, px},
    ui_widgets::{
        CoreSliderDragState, SetSliderValue, Slider as UiSlider, SliderRange, SliderThumb,
        SliderValue, SliderValueChange, TrackClick, ValueChange, slider_self_update,
    },
};

use crate::{
    Element, IntoChild, UiContext, absolute::Absolute, background::Background, border::Border,
    centered::Centered, positioned::Positioned, sized::Sized, theme::Theme,
};
use wane_observers::{EntityObserverRegistration, IntoEntityObserverRegistration};

pub struct Slider<E: Element> {
    min: f32,
    max: f32,
    value: f32,
    on_value_changed: Box<dyn EntityObserverRegistration>,
    content: E,
}

impl Slider<()> {
    #[inline]
    pub fn new_default<'a, F: Send + Sync, M: 'static>(
        on_value_changed: &'a F,
        min: f32,
        max: f32,
        value: f32,
    ) -> impl Element
    where
        &'a F: IntoObserverSystem<ValueChange<f32>, (), M>,
    {
        Slider {
            on_value_changed: Box::new(on_value_changed.into_registration()),
            min,
            max,
            value,
            content: Centered {
                content: (
                    Border::all(Background { content: () }).into_child(),
                    Absolute {
                        content: Positioned {
                            left: px(0),
                            right: px(12),
                            top: Val::Auto,
                            bottom: Val::Auto,
                            content: Centered {
                                content: Thumb::new_default().into_child(),
                            },
                        },
                    }
                    .into_child(),
                ),
            },
        }
    }
}

impl<E: Element> Slider<E> {
    #[inline]
    pub fn new<'a, F: Send + Sync, M: 'static>(
        registration: &'a F,
        min: f32,
        max: f32,
        value: f32,
        content: E,
    ) -> Self
    where
        &'a F: IntoObserverSystem<ValueChange<f32>, (), M>,
    {
        Self {
            on_value_changed: Box::new(registration.into_registration()),
            min,
            max,
            value,
            content,
        }
    }
}

impl<E: Element> Element for Slider<E> {
    type Bundle = (UiSlider, SliderValue, SliderRange, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (
            UiSlider {
                track_click: TrackClick::Snap,
            },
            SliderValue(self.value),
            SliderRange::new(self.min, self.max),
            self.content.create_bundle(context),
        )
    }

    #[inline]
    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext) {
        self.on_value_changed.register_observer(entity_command);
        entity_command.observe(slider_self_update);
        self.content.register_observers(entity_command, context);
    }

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }
}

#[derive(EntityEvent)]
pub struct SliderScroll {
    entity: Entity,
    delta: Vec2,
}

pub fn slider_on_mouse_wheel(on: On<SliderScroll>, mut commands: Commands) {
    commands.trigger(SetSliderValue {
        entity: on.entity,
        change: SliderValueChange::Relative(-on.delta.y),
    });
}

pub(crate) fn update_slider_style(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &CoreSliderDragState,
            Has<InteractionDisabled>,
        ),
        (
            Or<(
                Changed<SliderValue>,
                Changed<CoreSliderDragState>,
                Added<InteractionDisabled>,
            )>,
            With<UiSlider>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, Has<SliderThumb>), Without<UiSlider>>,
) {
    for (slider_ent, value, range, _drag_state, _disabled) in sliders.iter() {
        info!("slider changed");
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                let new_value = percent(range.thumb_position(value.0) * 100.0);

                info!("update thumb: {:?}", new_value);
                thumb_node.left = new_value;
            }
        }
    }
}

pub struct Thumb<E: Element> {
    pub content: E,
}

impl Thumb<()> {
    pub fn new_default() -> impl Element {
        Thumb {
            content: Theme::new(Absolute {
                content: Positioned::left(
                    percent(0),
                    Border::all(Background {
                        content: Sized {
                            width: px(12),
                            height: px(12),
                            content: (),
                        },
                    }),
                ),
            })
            .with_background_color(Some(Color::WHITE))
            .should_propagate(false),
        }
    }
}

impl<E: Element> Element for Thumb<E> {
    type Bundle = (SliderThumb, E::Bundle);

    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }

    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (SliderThumb, self.content.create_bundle(context))
    }

    fn register_observers(&self, entity_command: &mut EntityCommands, context: &UiContext) {
        self.content.register_observers(entity_command, context);
    }

    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        self.content.spawn_children(rcs, context);
    }
}

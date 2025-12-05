use std::{panic::Location, sync::Arc};

use bevy::{
    asset::Handle,
    color::Color,
    ecs::component::Component,
    text::Font,
    ui::{BorderColor, BorderRadius, UiRect},
};

use crate::{Element, UiContext};

#[derive(Component)]
pub struct Themed {
    pub context: Arc<UiContext>,
}

impl From<Arc<UiContext>> for Themed {
    #[inline]
    fn from(value: Arc<UiContext>) -> Self {
        Self { context: value }
    }
}

pub struct Theme<E: Element> {
    font: Option<Handle<Font>>,
    text_size: Option<f32>,
    text_size_big: Option<f32>,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
    hover_color: Option<Color>,
    text_color: Option<Color>,
    border_thickness: Option<UiRect>,
    border_color: Option<BorderColor>,
    border_radius: Option<BorderRadius>,
    content: E,
    propagate_to_children: bool,
    image_color: Option<Color>,
    highlight_color: Option<Color>,
}

impl<E: Element> Theme<E> {
    #[inline]
    #[track_caller]
    pub fn new(content: E) -> Self {
        Self {
            image_color: None,
            highlight_color: None,
            font: None,
            text_size: None,
            text_size_big: None,
            background_color: None,
            foreground_color: None,
            hover_color: None,
            text_color: None,
            border_thickness: None,
            border_color: None,
            border_radius: None,
            content,
            propagate_to_children: true,
        }
    }

    #[inline]
    pub fn should_propagate(mut self, value: bool) -> Self {
        self.propagate_to_children = value;
        self
    }
    #[inline]
    pub fn with_font(mut self, font: Option<Handle<Font>>) -> Self {
        self.font = font;
        self
    }
    #[inline]
    pub fn with_text_size(mut self, size: Option<f32>) -> Self {
        self.text_size = size;
        self
    }
    #[inline]
    pub fn with_text_size_big(mut self, size: Option<f32>) -> Self {
        self.text_size_big = size;
        self
    }

    #[inline]
    pub fn with_background_color(mut self, color: Option<Color>) -> Self {
        self.background_color = color;
        self
    }

    #[inline]
    pub fn with_border_radius(mut self, val: Option<BorderRadius>) -> Self {
        self.border_radius = val;
        self
    }

    #[inline]
    pub fn create_context(&self, other: &UiContext) -> UiContext {
        UiContext {
            current_animator: other.current_animator.clone(),
            image_color: self.image_color.unwrap_or(other.image_color),
            highlight_color: self.highlight_color.unwrap_or(other.highlight_color),
            font: self.font.clone().unwrap_or(other.font.clone()),
            text_size: self.text_size.unwrap_or(other.text_size),
            text_size_big: self.text_size_big.unwrap_or(other.text_size_big),
            background_color: self.background_color.unwrap_or(other.background_color),
            foreground_color: self.foreground_color.unwrap_or(other.foreground_color),
            hover_color: self.hover_color.unwrap_or(other.hover_color),
            text_color: self.text_color.unwrap_or(other.text_color),
            border_thickness: self.border_thickness.unwrap_or(other.border_thickness),
            border_color: self.border_color.unwrap_or(other.border_color),
            border_radius: self.border_radius.unwrap_or(other.border_radius),
        }
    }
}

impl<E: Element> Element for Theme<E> {
    type Bundle = (Themed, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        let new_context = Arc::new(self.create_context(context));
        (
            Themed::from(new_context.clone()),
            self.content.create_bundle(&new_context),
        )
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &UiContext,
    ) {
        let new_context = self.create_context(context);
        self.content
            .register_observers(entity_command, &new_context);
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: Arc<UiContext>,
    ) {
        if self.propagate_to_children {
            let new_context = Arc::new(self.create_context(&context));
            self.content.spawn_children(rcs, new_context);
        } else {
            self.content.spawn_children(rcs, context);
        }
    }

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &UiContext) {
        self.content.modify_node(node, context);
    }
}

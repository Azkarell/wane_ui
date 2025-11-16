use std::sync::Arc;

use bevy::{
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    text::{Justify, TextColor, TextFont, TextLayout},
    ui::{Node, widget::Text as UiText},
};

use crate::{Element, UiContext};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextSizing {
    Huge,
    Big,
    Small,
}
#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub sizing: TextSizing,
}

impl Element for Text {
    type Bundle = (UiText, TextFont, TextColor, TextLayout);

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        let size = match self.sizing {
            TextSizing::Huge => context.text_size_big * 2.0,
            TextSizing::Big => context.text_size_big,
            TextSizing::Small => context.text_size,
        };
        (
            UiText::new(&self.text),
            TextFont::from_font_size(size).with_font(context.font.clone()),
            TextColor(context.text_color.clone()),
            TextLayout::new_with_justify(Justify::Center),
        )
    }

    #[inline]
    fn register_observers(&self, _entity_command: &mut EntityCommands, _context: &UiContext) {}

    #[inline]
    fn spawn_children(&self, _rcs: &mut RelatedSpawnerCommands<ChildOf>, _context: Arc<UiContext>) {
    }

    #[inline]
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}
}

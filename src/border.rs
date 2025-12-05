use std::sync::Arc;

use bevy::ui::{BorderColor, BorderRadius, Node, UiRect, px};

use crate::{Element, UiContext};

pub struct Border<E: Element> {
    pub content: E,
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for Border<()> {
    #[track_caller]
    fn default() -> Self {
        Self {
            content: (),
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }
}

impl<E: Element> Border<E> {
    #[inline]
    #[track_caller]
    pub fn all(content: E) -> Self {
        Self {
            content: content,
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }

    #[inline]
    pub fn with_content<C: Element>(self, content: C) -> Border<C> {
        Border {
            content,
            top: self.top,
            bottom: self.bottom,
            left: self.left,
            right: self.right,
        }
    }

    #[inline]
    #[track_caller]
    pub fn bottom(content: E) -> Self {
        Self {
            content,
            top: false,
            bottom: true,
            left: false,
            right: false,
        }
    }

    #[inline]
    #[track_caller]
    pub fn top(content: E) -> Self {
        Self {
            content,
            top: true,
            bottom: false,
            left: false,
            right: false,
        }
    }
    #[inline]
    #[track_caller]
    pub fn right(content: E) -> Self {
        Self {
            content,
            top: false,
            bottom: false,
            left: false,
            right: true,
        }
    }
    #[inline]
    #[track_caller]
    pub fn left(content: E) -> Self {
        Self {
            content,
            top: false,
            bottom: false,
            left: true,
            right: false,
        }
    }
}

impl<E: Element> Element for Border<E> {
    type Bundle = (BorderRadius, BorderColor, E::Bundle);

    #[inline]
    fn modify_node(&self, node: &mut Node, context: &UiContext) {
        let ui_rect = UiRect::new(
            if self.left {
                context.border_thickness.left
            } else {
                px(0)
            },
            if self.right {
                context.border_thickness.right
            } else {
                px(0)
            },
            if self.top {
                context.border_thickness.top
            } else {
                px(0)
            },
            if self.bottom {
                context.border_thickness.bottom
            } else {
                px(0)
            },
        );
        node.border = ui_rect;

        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (
            context.border_radius,
            context.border_color,
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
        context: Arc<super::UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }
}

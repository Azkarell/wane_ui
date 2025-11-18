use std::sync::Arc;

use bevy::{
    asset::Handle,
    ecs::{hierarchy::ChildOf, relationship::RelatedSpawnerCommands, system::EntityCommands},
    math::Rect,
    prelude::Image as UiImage,
    ui::{
        Node,
        widget::{ImageNode, NodeImageMode},
    },
};

use crate::{Element, IntoChild, UiContext, child::Child};

pub struct Image {
    pub child: Option<Child>,
    pub handle: Option<Handle<UiImage>>,
    pub rect: Option<Rect>,
    pub image_mode: NodeImageMode,
}

impl Image {
    #[inline]
    pub fn new() -> Self {
        Self {
            child: None,
            handle: None,
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }

    #[inline]
    pub fn with_child<I: IntoChild>(mut self, child: I) -> Self {
        self.child = Some(child.into_child());
        self
    }

    #[inline]
    pub fn set_image(&mut self, handle: Handle<UiImage>) {
        self.handle = Some(handle);
    }
}

impl Element for Image {
    type Bundle = ImageNode;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        ImageNode {
            color: context.image_color,
            image: self.handle.clone().unwrap_or(Handle::default()),
            texture_atlas: None,
            flip_x: false,
            flip_y: false,
            rect: self.rect,
            image_mode: self.image_mode.clone(),
        }
    }

    #[inline]
    fn register_observers(&self, _entity_commands: &mut EntityCommands, _context: &UiContext) {}

    #[inline]
    fn spawn_children(&self, rcs: &mut RelatedSpawnerCommands<ChildOf>, context: Arc<UiContext>) {
        let Some(ref c) = self.child else {
            return;
        };
        c.spawn_children(rcs, context);
    }

    #[inline]
    fn modify_node(&self, _node: &mut Node, _context: &UiContext) {}
}

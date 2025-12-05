use std::{ops::Deref, panic::Location, sync::Arc};

use bevy::{
    asset::{Handle, uuid::Uuid, uuid_handle},
    ecs::{
        hierarchy::ChildOf,
        observer::On,
        relationship::RelatedSpawnerCommands,
        resource::Resource,
        system::{EntityCommands, Query, Res},
    },
    log::error,
    math::Rect,
    prelude::Image as UiImage,
    ui::{
        Node,
        widget::{ImageNode, NodeImageMode},
    },
};

use crate::{Element, IntoChild, UiContext, child::Child, events::Init};

pub struct Image {
    pub child: Option<Child>,
    pub handle: Handle<UiImage>,
    pub rect: Option<Rect>,
    pub image_mode: NodeImageMode,
}

impl Image {
    #[inline]
    #[track_caller]
    pub fn new() -> Self {
        Self {
            child: None,
            handle: Handle::default(),
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }
    #[inline]
    #[track_caller]
    pub fn new_with_handle(handle: Handle<UiImage>) -> Self {
        Self {
            child: None,
            handle: handle,
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
        self.handle = handle;
    }
}

impl Element for Image {
    type Bundle = ImageNode;

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        ImageNode {
            color: context.image_color,
            image: self.handle.clone(),
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

pub fn init_image_from_resource<R: Resource + Deref<Target = Handle<bevy::prelude::Image>>>(
    on: On<Init>,
    mut query: Query<&mut ImageNode>,
    image: Res<R>,
) {
    let Ok(mut image_node) = query.get_mut(on.entity) else {
        error!("image node not found");
        return;
    };
    image_node.image = (**image).clone()
}

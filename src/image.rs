use std::{ops::Deref, sync::Arc};

use bevy::{
    asset::Handle,
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

pub struct Image<E: Element> {
    pub content: E,
    pub handle: Handle<UiImage>,
    pub rect: Option<Rect>,
    pub image_mode: NodeImageMode,
}
impl Image<()> {
    #[inline]
    pub fn new() -> Self {
        Self {
            content: (),
            handle: Handle::default(),
            rect: None,
            image_mode: NodeImageMode::default(),
        }
    }
    #[inline]
    pub fn new_with_handle(handle: Handle<UiImage>) -> Self {
        Self {
            content: (),
            handle: handle,
            rect: None,
            image_mode: NodeImageMode::Auto,
        }
    }
}

impl<E: Element> Image<E> {
    #[inline]
    pub fn with_content<O: Element>(self, child: O) -> Image<O> {
        Image {
            content: child,
            handle: self.handle,
            rect: self.rect,
            image_mode: self.image_mode,
        }
    }

    #[inline]
    pub fn with_child<I: IntoChild>(self, child: I) -> Image<Child> {
        Image {
            content: child.into_child(),
            handle: self.handle,
            rect: self.rect,
            image_mode: self.image_mode,
        }
    }

    #[inline]
    pub fn set_image(&mut self, handle: Handle<UiImage>) {
        self.handle = handle;
    }
}

impl<E: Element> Element for Image<E> {
    type Bundle = (ImageNode, E::Bundle);

    #[inline]
    fn create_bundle(&self, context: &UiContext) -> Self::Bundle {
        (
            ImageNode {
                color: context.image_color,
                image: self.handle.clone(),
                texture_atlas: None,
                flip_x: false,
                flip_y: false,
                rect: self.rect,
                image_mode: self.image_mode.clone(),
            },
            self.content.create_bundle(context),
        )
    }

    #[inline]
    fn register_observers(&self, entity_commands: &mut EntityCommands, context: &UiContext) {
        self.content.register_observers(entity_commands, context);
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

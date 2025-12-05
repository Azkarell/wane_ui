use std::{collections::HashMap, iter::once, panic::Location};

use bevy::{
    animation::{
        AnimationClip, AnimationPlayer, AnimationTarget, AnimationTargetId, VariableCurve,
        animation_curves::AnimationCurve,
        graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex},
    },
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::EntityEvent,
        hierarchy::{ChildOf, Children},
        name::Name,
        observer::On,
        system::{Commands, EntityCommands, Query, ResMut},
    },
    log::{error, info},
};

use crate::{Element, events::Init};

pub trait AnimationRegistration {
    fn register(&self, clip: &mut AnimationClip);
    fn clone_dyn(&self) -> Box<dyn AnimationRegistration + Send + Sync>;
}

pub struct Transition {
    registration: Box<dyn AnimationRegistration + Send + Sync>,
}

impl Clone for Transition {
    fn clone(&self) -> Self {
        Self {
            registration: self.registration.clone_dyn(),
        }
    }
}

impl<AR: AnimationRegistration + Send + Sync + 'static> From<AR> for Transition {
    fn from(value: AR) -> Self {
        Self {
            registration: Box::new(value),
        }
    }
}

impl AnimationRegistration for (AnimationTargetId, VariableCurve) {
    fn register(&self, clip: &mut AnimationClip) {
        clip.add_variable_curve_to_target(self.0, self.1.clone());
    }

    fn clone_dyn(&self) -> Box<dyn AnimationRegistration + Send + Sync> {
        Box::new((self.0, self.1.clone()))
    }
}

pub struct Animated<E: Element> {
    pub animation_id: AnimationTargetId,
    pub name: Name,
    pub transitions: Vec<Transition>,
    pub content: E,
}

impl<E: Element> Animated<E> {
    #[track_caller]
    pub fn new(name: String, content: E) -> Self {
        let name = Name::new(name);
        let id = AnimationTargetId::from_name(&name);

        Self {
            animation_id: id,
            name,
            content,
            transitions: vec![],
        }
    }

    #[inline]
    pub fn with_curve(mut self, curve: impl AnimationCurve) -> Self {
        self.transitions
            .push((self.animation_id, VariableCurve::new(curve)).into());
        self
    }
}

#[derive(Component)]
pub struct AnimationNodes {
    pub player: Entity,
    pub animations: HashMap<String, AnimationNodeIndex>,
}

impl AnimationNodes {
    pub fn new(
        player: Entity,
        animations: impl Iterator<Item = (String, AnimationNodeIndex)>,
    ) -> Self {
        Self {
            player,
            animations: animations.collect(),
        }
    }
}

#[derive(EntityEvent)]
struct AfterAnimationGraphInit {
    pub entity: Entity,
}

impl<E: Element> Element for Animated<E> {
    type Bundle = (Name, AnimationTarget, E::Bundle);

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &crate::UiContext) {
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &crate::UiContext) -> Self::Bundle {
        (
            self.name.clone(),
            AnimationTarget {
                id: self.animation_id.clone(),
                player: context.current_animator.expect("No Animator set"),
            },
            self.content.create_bundle(context),
        )
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &crate::UiContext,
    ) {
        self.content.register_observers(entity_command, context);
        let animator = context.current_animator.clone().expect("animator exists");
        let transitions = self.transitions.clone();
        let name = self.name.clone();
        entity_command.observe(
            move |on: On<AfterAnimationGraphInit>,
                  query: Query<&AnimationGraphHandle>,
                  mut commands: Commands,
                  mut animation_nodes_query: Query<&mut AnimationNodes>,
                  mut graphs: ResMut<Assets<AnimationGraph>>,
                  mut clips: ResMut<Assets<AnimationClip>>| {
                info!("animator: {}", animator);
                let Ok(graph_handle) = query.get(animator) else {
                    error!("graph_handle not found");
                    return;
                };
                let graph = graphs.get_mut(&graph_handle.0).expect("Graph should exist");
                let mut clip = AnimationClip::default();
                transitions
                    .iter()
                    .for_each(|t| t.registration.register(&mut clip));
                let clip_handle = clips.add(clip);
                info!("graph id: {:?}", graph_handle.0);
                let index = graph.add_clip(clip_handle, 1.0, graph.root);
                if let Ok(mut animation_node) = animation_nodes_query.get_mut(on.entity) {
                    info!("inserting animation: {:?}", index);
                    animation_node.animations.insert(name.to_string(), index);
                } else {
                    info!("adding animation: {:?}", index);
                    let animations = AnimationNodes::new(animator, once((name.to_string(), index)));
                    commands.entity(on.entity).insert(animations);
                };
            },
        );
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: std::sync::Arc<crate::UiContext>,
    ) {
        self.content.spawn_children(rcs, context);
    }
}

pub struct Animator<E: Element> {
    pub graph: Handle<AnimationGraph>,
    pub content: E,
}
impl<E: Element> Animator<E> {
    #[track_caller]
    pub fn new(content: E) -> Self {
        Self {
            graph: Handle::default(),

            content,
        }
    }
}

impl<E: Element> Element for Animator<E> {
    type Bundle = (AnimationPlayer, AnimationGraphHandle, E::Bundle);

    #[inline]
    fn modify_node(&self, node: &mut bevy::ui::Node, context: &crate::UiContext) {
        self.content.modify_node(node, context);
    }

    #[inline]
    fn create_bundle(&self, context: &crate::UiContext) -> Self::Bundle {
        let player = AnimationPlayer::default();
        let handle = AnimationGraphHandle(self.graph.clone());
        (player, handle, self.content.create_bundle(context))
    }

    #[inline]
    fn register_observers(
        &self,
        entity_command: &mut bevy::ecs::system::EntityCommands,
        context: &crate::UiContext,
    ) {
        self.content.register_observers(entity_command, context);
        entity_command.observe(init_graph);
    }

    #[inline]
    fn spawn_children(
        &self,
        rcs: &mut bevy::ecs::relationship::RelatedSpawnerCommands<bevy::ecs::hierarchy::ChildOf>,
        context: std::sync::Arc<crate::UiContext>,
    ) {
        let context = context.with_animator(Some(rcs.target_entity()));
        self.content.spawn_children(rcs, context);
    }
}

fn init_graph(
    on: On<Init>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut query: Query<&mut AnimationGraphHandle>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    let Ok(mut graph_handle) = query.get_mut(on.entity) else {
        error!("not animation graph handle found");
        return;
    };
    let graph = AnimationGraph::new();
    let handle = graphs.add(graph);
    graph_handle.0 = handle;
    for c in children.iter_descendants(on.entity) {
        commands.trigger(AfterAnimationGraphInit { entity: c });
    }
}

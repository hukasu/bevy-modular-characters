use bevy::{
    ecs::{entity::Entity, event::Event},
    prelude::Deref,
};

#[derive(Debug, Event, Deref)]
pub struct ResetChanged(pub Entity);

use bevy::ecs::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Snapshot {
    pub entities:Vec<EntitySnapshot>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThingSnapshot {
    pub x:Option<f32>,
    pub y:Option<f32>,
    pub vx:Option<f32>,
    pub vy:Option<f32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerSnapshot {
    pub name:Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntitySnapshot {
    pub id:Entity,
    pub thing:Option<ThingSnapshot>,
    pub player:Option<PlayerSnapshot>
}

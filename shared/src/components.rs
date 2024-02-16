use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name:String
}

#[derive(Component)]
pub struct Thing {
    pub pos:Vec3,
    pub vel:Vec3
}

#[derive(Component)]
pub struct Replicate;
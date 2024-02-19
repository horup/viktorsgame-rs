use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player {
    pub name:String
}

#[derive(Component, Default, Clone)]
pub struct Thing {
    pub pos:Vec3,
    pub vel:Vec3
}

#[derive(Component)]
pub struct Replicate;


#[derive(Component)]
pub struct Prev<T>(pub T);

impl<T:Default> Default for Prev<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for Prev<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Prev<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
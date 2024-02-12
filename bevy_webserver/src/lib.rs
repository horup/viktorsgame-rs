use bevy::prelude::*;
use std::marker::PhantomData;

pub struct BevyWebserver<T> {
    pub phantom:PhantomData<T>
}

impl<T : Send + Sync + 'static> Plugin for BevyWebserver<T> {
    fn build(&self, app: &mut App) {
        
    }
}
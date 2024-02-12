use bevy::{prelude::*, reflect::erased_serde::Serialize};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub trait Message : Send + Sync + Serialize + DeserializeOwned + 'static {}
impl<T> Message for T where T : Send + Sync + Serialize + DeserializeOwned + 'static {}

pub struct BevyWebserver<T> {
    pub phantom:PhantomData<T>
}

impl<T : Message> Plugin for BevyWebserver<T> {
    fn build(&self, app: &mut App) {
        
    }
}
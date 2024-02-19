use bevy::prelude::*;
use shared::*;
use shared::{Prev, Thing};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub prev_player: Prev<Player>,
    pub thing: Thing,
    pub prev_thing: Prev<Thing>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Default::default(),
            prev_player: Default::default(),
            thing: Default::default(),
            prev_thing: Default::default(),
        }
    }
}

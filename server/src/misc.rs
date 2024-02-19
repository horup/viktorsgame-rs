use bevy::prelude::*;
use shared::*;

pub type AllReplicatesQuery<'a, 'b, 'c> = Query<'a, 'b, (Entity, Option<&'c Thing>, Option<&'c Player>), With<Replicate>>;
pub fn new_complete_snapshot(replicates:&mut AllReplicatesQuery) -> Snapshot {
    let mut snapshot = Snapshot::default();
    for (id, thing, player) in replicates.iter() {
        snapshot.entities.push(EntitySnapshot {
            id,
            thing: thing.map(|thing| ThingSnapshot {
                x: Some(thing.pos.x),
                y: Some(thing.pos.y),
                vx: Some(thing.vel.x),
                vy: Some(thing.vel.y),
            }),
            player: player.map(|player| PlayerSnapshot {
                name: Some(player.name.clone()),
            }),
        })
    };
    snapshot
}

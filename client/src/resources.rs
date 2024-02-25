use std::time::{Duration, Instant};

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct EntityMapper {
    server_to_client:HashMap<Entity, Entity>,
    client_to_server:HashMap<Entity, Entity>
}

impl EntityMapper {
    pub fn server_to_client(&self, server_entity:&Entity) -> Option<&Entity> {
        self.server_to_client.get(server_entity)
    }

    pub fn client_to_server(&self, client_entity:&Entity) -> Option<&Entity> {
        self.client_to_server.get(client_entity)
    }

    pub fn map(&mut self, server_entity:Entity, client_entity:Entity) {
        self.server_to_client.insert(server_entity.clone(), client_entity.clone());
        self.client_to_server.insert(client_entity.clone(), server_entity.clone());
    }
}

#[derive(Resource)]
pub struct ServerInfo {
    pub timestep_sec:f32,
    pub last_update:Duration
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self { timestep_sec: 0.05, last_update:Duration::default() }
    }
}

impl ServerInfo {
    pub fn next_update_alpha(&self, elapsed:&Duration) -> f32 {
        let delta = *elapsed - self.last_update;
        let delta_sec = delta.as_secs_f32();
        delta_sec / self.timestep_sec
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn server_info() {
        let mut server_info = ServerInfo::default();
        let mut time = Time::new(Instant::now());
        server_info.timestep_sec = 1.0;
        server_info.last_update = time.elapsed();
        time.advance_by(Duration::from_secs(1));
        server_info.last_update = time.elapsed();
        assert_eq!(server_info.next_update_alpha(&time.elapsed()), 0.0);
        assert_eq!(server_info.last_update, time.elapsed());
        time.advance_by(Duration::from_millis(500));
        assert_eq!(server_info.next_update_alpha(&time.elapsed()), 0.5);
        time.advance_by(Duration::from_millis(500));
        assert_eq!(server_info.next_update_alpha(&time.elapsed()), 1.0);
    }
}
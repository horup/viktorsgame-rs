use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct EntityMapper {
    server_to_client:HashMap<Entity, Entity>,
    client_to_Server:HashMap<Entity, Entity>
}

impl EntityMapper {
    pub fn server_to_client(&self, server_entity:&Entity) -> Option<&Entity> {
        self.server_to_client.get(server_entity)
    }

    pub fn client_to_server(&self, client_entity:&Entity) -> Option<&Entity> {
        self.client_to_Server.get(client_entity)
    }

    pub fn map(&mut self, server_entity:Entity, client_entity:Entity) {
        self.server_to_client.insert(server_entity.clone(), client_entity.clone());
        self.client_to_Server.insert(client_entity.clone(), server_entity.clone());
    }
}
use bevy::ecs::{entity::Entity, system::Resource};

pub struct Connection(Entity, Entity);

impl PartialEq for Connection {
    /// Connections are unordered, so (a, b) equals (b, a).
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
    }
}

#[derive(Default, Resource)]
pub struct ConnectionManager {
    connections: Vec<Connection>,
}

impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager {
            connections: vec![],
        }
    }

    pub fn create_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn find_connections(&self, entity: Entity) -> Vec<Entity> {
        self.connections
            .iter()
            .filter_map(|pair| {
                if pair.0 == entity {
                    Some(pair.1)
                } else if pair.1 == entity {
                    Some(pair.0)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn remove_connection(&mut self, connection: Connection) {
        let con_idx = self.connections.iter().position(|pair| *pair == connection);
        if let Some(con_idx) = con_idx {
            self.connections.swap_remove(con_idx);
        }
    }

    pub fn remove_all_connections(&mut self, entity: Entity) {
        self.connections
            .retain(|pair| pair.0 != entity && pair.1 != entity);
    }

    pub fn has_connection(&self, connection: Connection) -> bool {
        self.connections.iter().any(|pair| *pair == connection)
    }
}

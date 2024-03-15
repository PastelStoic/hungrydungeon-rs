pub mod ai;
pub mod organs;
pub mod player;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Actor {
    pub health_current: i32,
    pub health_max: i32,
    pub attack: i32,
    pub defense: i32,
    pub size: i32,
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            health_current: 1000,
            health_max: 1000,
            attack: 100,
            defense: 100,
            size: 100,
        }
    }
}

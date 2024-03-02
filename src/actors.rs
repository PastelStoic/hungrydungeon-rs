pub mod organs;
pub mod ai;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Actor {
    pub health_current: i32,
    pub health_max: i32,
    pub attack: i32,
    pub defense: i32,
}
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>();
    }
}

#[derive(Event)]
pub enum PlayerActionEvent {
    Attack { target: String },
    Devour { target: String, organ: String },
    MoveRoom { room: String },
    Struggle,
}

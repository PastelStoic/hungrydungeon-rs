use bevy::prelude::*;

use super::Actor;



/// Shows that the player intends to run an action.
/// Whether this action is possible is determined later.
#[derive(Event)]
pub struct PlayerActionEvent {
    pub player: Entity,
    pub event_type: PlayerActionEventType,
}

pub enum PlayerActionEventType {
    Attack { target: String },
    Devour { target: String, organ: String },
    MoveRoom { room: String },
    Struggle,
}

// using the names given in the actionevent above, finds the entities corrosponding to said action
pub fn find_action_targets(
    q_actors: Query<(Entity, &Name), With<Actor>>,
    mut reader: EventReader<PlayerActionEvent>,
) {
    for ev in reader.read() {
        if let PlayerActionEventType::Attack { target } = &ev.event_type {
            // find a target by that name
            let target = q_actors.iter_mut().find(|a| a.1.as_str() == target);
            match target {
                Some(mut target) => {
                    target.2.health_current -= 5; // find player actor, use their attack
                }
                None => {
                    // error message, can't find target
                }
            }
        }
    }
}

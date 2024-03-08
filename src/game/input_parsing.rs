use super::actors::player::PlayerActionEvent;

pub fn parse_player_input(input: &String) -> Result<PlayerActionEvent, &'static str> {
    let mut split = input.split_whitespace();
    match split.next() {
        Some(w1) => match w1 {
            "attack" => match split.next() {
                Some(target) => Ok(PlayerActionEvent::Attack {
                    target: target.to_string(),
                }),
                None => Err("Missing target for attack"),
            },
            "devour" => match split.next() {
                // the third word is ignored - "devour x with y", "devour x using y", etc are all valid
                Some(target) => match split.nth(1) {
                    Some(organ) => Ok(PlayerActionEvent::Devour {
                        target: target.to_string(),
                        organ: organ.to_string(),
                    }),
                    None => Err("Missing organ for devour"),
                },
                None => Err("Missing target for devour"),
            },
            "moveto" => match split.next() {
                Some(room) => Ok(PlayerActionEvent::MoveRoom {
                    room: room.to_string(),
                }),
                None => Err("Missing room name"),
            },
            "struggle" => Ok(PlayerActionEvent::Struggle),
            _ => Err("Unknown action"),
        },
        None => Err("Unknown action"),
    }
}

/*
Possible outcomes:
player attack an enemy: "attack (enemy name)"
player devour an enemy: "devour (enemy name) with (organ name)"
move to another room: "move to (room name)"
struggle: "struggle"
escape: "escape" to go outside, "escape to (connected organ)" to move organs
remove prey from organ: "eject (prey name) to  put them outside, "eject (prey name) to (connected organ) to move them
*/

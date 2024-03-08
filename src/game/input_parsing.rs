pub enum PlayerInputParseResult<'a> {
    Attack { target: &'a str },
    Devour { target: &'a str, organ: &'a str },
    MoveRoom { room: &'a str },
    Struggle,
    Error(&'static str),
    Unknown,
}

pub fn parse_player_input(input: &String) -> PlayerInputParseResult {
    let mut split = input.split_whitespace();
    match split.next() {
        Some(w1) => match w1 {
            "attack" => match split.next() {
                Some(target) => PlayerInputParseResult::Attack { target },
                None => PlayerInputParseResult::Error("Missing target for attack"),
            },
            "devour" => match split.next() {
                // the third word is ignored - "devour x with y", "devour x using y", etc are all valid
                Some(target) => match split.nth(1) {
                    Some(organ) => PlayerInputParseResult::Devour { target, organ },
                    None => PlayerInputParseResult::Error("Missing organ for devour"),
                },
                None => PlayerInputParseResult::Error("Missing target for devour"),
            },
            "moveto" => match split.next() {
                Some(room) => PlayerInputParseResult::MoveRoom { room },
                None => PlayerInputParseResult::Error("Missing room name"),
            },
            "struggle" => PlayerInputParseResult::Struggle,
            _ => PlayerInputParseResult::Unknown,
        },
        None => PlayerInputParseResult::Unknown,
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

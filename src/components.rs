use std::fmt::Display;

use bevy::prelude::Component;

#[derive(Component)]
pub struct Name(String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

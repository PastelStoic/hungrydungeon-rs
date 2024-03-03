pub mod actors;
pub mod components;

use actors::ai::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_event::<AttackActionEvent>()
        .add_systems(Update, run_ai)
        .add_systems(Update, respond_to_ai)
        .run();
}

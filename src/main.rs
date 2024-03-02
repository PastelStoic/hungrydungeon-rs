pub mod actors;
pub mod components;

use actors::{ai::SlimeAi, Actor};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Update, run_ai)
        .run();
}

fn run_ai(mut query: Query<(Entity, &Name, &mut Actor), With<SlimeAi>>, mut query_enemies: Query<(Entity, &Name, &mut Actor)>) {
    // has a list of possible actions based on AI type
    // each of these actions is calculated, given a weight
    // for now, skip all this, just find the nearest actor and attack
    for (entity, name, mut actor) in &mut query {
        for (enemy_entity, enemy_name, mut enemy_actor) in &mut query_enemies {
            if enemy_entity != entity {
                println!("AI {name} detected enemy {enemy_name}!");
                // here, the ai calculates possible actions for this enemy
            }
        }
    }
    todo!()
}
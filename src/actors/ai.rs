use bevy::prelude::*;
use rand::prelude::*;

use super::Actor;

#[derive(Component)]
pub struct Ai;

#[derive(Event)]
pub struct AttackActionEvent(Entity, Entity);

pub fn run_ai(query: Query<(Entity, &Name, &Actor, Option<&Ai>)>) {
    // has a list of possible actions based on AI type
    // each of these actions is calculated, given a weight
    // for now, skip all this, just find the nearest actor and attack
    for l1 in &query {
        if let Some(_) = l1.3 {
            let mut possible_targets = vec![];
            for l2 in &query {
                if l1.0 != l2.0 {
                    possible_targets.push((1, l2.0));
                }
            }

            // picks target, creates attack event. Unique events for every possible action?
            
        }
    }
}
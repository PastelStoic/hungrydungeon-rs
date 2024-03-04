use bevy::prelude::*;
use rand::prelude::*;

use crate::AiTimer;

use super::Actor;

#[derive(Component)]
pub struct Ai;

#[derive(Event)]
pub struct AttackActionEvent(Entity, Entity);

pub fn run_ai(
    query: Query<(Entity, &Name, &Actor, Option<&Ai>)>,
    mut writer: EventWriter<AttackActionEvent>,
    aitimer: Res<AiTimer>,
) {
    // early return if it's not time yet
    if !aitimer.0.finished() {
        return;
    }

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
            if let Some(target) = possible_targets.first() {
                writer.send(AttackActionEvent(l1.0, target.1));
            }
        }
    }
}

pub fn respond_to_ai(
    mut reader: EventReader<AttackActionEvent>,
    mut query: Query<(Entity, &Name, &mut Actor)>,
) {
    for ev in reader.read() {
        let [attacker, mut target] = query.many_mut([ev.0, ev.1]);
        // this works!
        if target.2.health_current > 0 {
            target.2.health_current -= attacker.2.attack;
            println!(
                "AI {} dealt {} damage to {}, leaving them with {} hp.",
                attacker.1, attacker.2.attack, target.1, target.2.health_current
            );
        } else {
            println!(
                "AI {} sees {}, but their hp isn't high enough to attack.",
                attacker.1, target.1,
            );
        }
    }
}

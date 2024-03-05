use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::*};

use crate::{actors::Actor, AiTimer};

#[derive(Component)]
pub struct SlimeAi;

pub struct SlimeAiPlugin;

impl Plugin for SlimeAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SlimeAiShouldRunEvent>()
            .add_systems(Update, (check_ai_should_run, ai_choose_action, run_ai));
    }
}

#[derive(Event)]
struct SlimeAiShouldRunEvent(Entity);

fn check_ai_should_run(
    mut ev: EventWriter<SlimeAiShouldRunEvent>,
    query: Query<(Entity, &Actor, &SlimeAi)>,
) {
    for thing in &query {
        if thing.1.health_current > 0 {
            ev.send(SlimeAiShouldRunEvent(thing.0));
        }
    }
}

fn ai_choose_action(
    query: Query<(Entity, &Name, &Actor, Option<&SlimeAi>)>,
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
            if let Ok(windex) = WeightedIndex::new(possible_targets.iter().map(|item| item.0)) {
                let mut rng = thread_rng();
                let target = possible_targets[windex.sample(&mut rng)].1;
                writer.send(AttackActionEvent {
                    attacker: l1.0,
                    defender: target,
                    message_template: "Slime attacks!",
                });
            }
        }
    }
}

#[derive(Event)]
pub struct AttackActionEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub message_template: &'static str,
}

pub fn run_ai(
    mut reader: EventReader<AttackActionEvent>,
    mut query: Query<(Entity, &Name, &mut Actor)>,
) {
    for ev in reader.read() {
        let [attacker, mut target] = query.many_mut([ev.attacker, ev.defender]);
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

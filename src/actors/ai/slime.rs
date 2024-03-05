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
    query: Query<(Entity, &Actor), With<SlimeAi>>,
    aitimer: Res<AiTimer>,
) {
    // early return if it's not time yet
    if !aitimer.0.finished() {
        return;
    }
    for thing in &query {
        if thing.1.health_current > 0 {
            ev.send(SlimeAiShouldRunEvent(thing.0));
        }
    }
}

fn ai_choose_action(
    query: Query<(Entity, &Name, &Actor, Option<&SlimeAi>)>,
    mut writer: EventWriter<AttackActionEvent>,
) {
    // has a list of possible actions based on AI type
    // each of these actions is calculated, given a weight
    // for now, skip all this, just find the nearest actor and attack
    for slime in &query {
        if slime.3.is_some() {
            let mut possible_targets = vec![];
            for target in &query {
                if slime.0 != target.0 && target.2.health_current > 0 {
                    possible_targets.push((1, target.0));
                }
            }

            // picks target, creates attack event. Unique events for every possible action?
            if let Ok(windex) = WeightedIndex::new(possible_targets.iter().map(|item| item.0)) {
                let mut rng = thread_rng();
                let target = possible_targets[windex.sample(&mut rng)].1;
                writer.send(AttackActionEvent {
                    attacker: slime.0,
                    defender: target,
                });
            }
        }
    }
}

#[derive(Event)]
pub struct AttackActionEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

pub fn run_ai(
    mut reader: EventReader<AttackActionEvent>,
    mut query: Query<(Entity, &Name, &mut Actor)>,
) {
    for ev in reader.read() {
        let actors = query.get_many_mut([ev.attacker, ev.defender]);
        if let Ok([attacker, mut target]) = actors {
            // check if the slime is still active, if the target is still in reach, if its still alive
            // the "is this target valid" check should be the same code both here and above
            target.2.health_current -= attacker.2.attack;
            println!(
                "{} attacks {}, dealing {} damage!",
                attacker.1, attacker.2.attack, target.1,
            );
        }
    }
}

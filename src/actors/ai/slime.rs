use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::*};

use crate::{actors::Actor, AiTimer};

#[derive(Component)]
pub struct SlimeAi;

pub struct SlimeAiPlugin;

impl Plugin for SlimeAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AiShouldRunEvent>()
            .add_event::<ActionEvent>()
            .add_systems(Update, (check_ai_should_run, ai_choose_action, run_ai));
    }
}

#[derive(Event)]
struct AiShouldRunEvent(Entity);

#[derive(Event)]
pub enum ActionEvent {
    Attack { attacker: Entity, defender: Entity },
}

fn check_ai_should_run(
    mut ev: EventWriter<AiShouldRunEvent>,
    query: Query<(Entity, &Actor), With<SlimeAi>>,
    aitimer: Res<AiTimer>,
) {
    // early return if it's not time yet
    if !aitimer.0.finished() {
        return;
    }
    for thing in &query {
        if thing.1.health_current > 0 {
            ev.send(AiShouldRunEvent(thing.0));
        }
    }
}

fn ai_choose_action(
    query: Query<(Entity, &Name, &Actor, Option<&SlimeAi>)>,
    mut ev: EventReader<AiShouldRunEvent>,
    mut writer: EventWriter<ActionEvent>,
) {
    // has a list of possible actions based on AI type
    // each of these actions is calculated, given a weight
    // for now, skip all this, just find the nearest actor and attack
    for ev in ev.read() {
        let slime = query.get(ev.0).unwrap();
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
                writer.send(ActionEvent::Attack {
                    attacker: slime.0,
                    defender: target,
                });
            }
        }
    }
}

pub fn run_ai(mut reader: EventReader<ActionEvent>, mut query: Query<(Entity, &Name, &mut Actor)>) {
    for ev in reader.read() {
        match ev {
            ActionEvent::Attack { attacker, defender } => {
                let actors = query.get_many_mut([*attacker, *defender]);
                if let Ok([attacker, mut target]) = actors {
                    // check if the slime is still active, if the target is still in reach, if its still alive
                    // the "is this target valid" check should be the same code both here and above
                    target.2.health_current -= attacker.2.attack;
                    println!(
                        "{} attacks {}, dealing {} damage!",
                        attacker.1, target.1, attacker.2.attack
                    );
                }
            }
        }
    }
}

use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::*};

use crate::game::{
    actors::{
        organs::{Organ, OrganType},
        Actor,
    },
    AiTimer,
};

#[derive(Component)]
pub struct SlimeGirlAi;

pub struct SlimeGirlAiPlugin;

impl Plugin for SlimeGirlAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AiShouldRunEvent>()
            .add_event::<ActionEvent>()
            .add_systems(
                Update,
                (
                    check_ai_should_run,
                    ai_choose_action,
                    run_attack,
                    run_devour,
                ),
            );
    }
}

pub fn spawn(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("Slimegirl"),
            Actor {
                health_current: 100,
                health_max: 100,
                attack: 10,
                defense: 10,
            },
            SlimeGirlAi,
        ))
        .with_children(|owner| {
            owner.spawn((
                Name::new("Slimegirl stomach"),
                Organ {
                    health_current: 100,
                    health_max: 100,
                    attack: 10,
                    defense: 10,
                    capacity: 100,
                    fullness_current: 0,
                    organ_type: OrganType::Generic,
                },
            ));
        });
}

#[derive(Event)]
struct AiShouldRunEvent(Entity);

#[derive(Event)]
pub enum ActionEvent {
    Attack {
        attacker: Entity,
        defender: Entity,
    },
    Devour {
        attacker: Entity,
        defender: Entity,
        organ: Entity,
    },
}

fn check_ai_should_run(
    mut ev: EventWriter<AiShouldRunEvent>,
    query: Query<(Entity, &Actor), With<SlimeGirlAi>>,
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

#[allow(clippy::type_complexity)]
fn ai_choose_action(
    query: Query<(
        Entity,
        &Name,
        &Actor,
        &Parent,
        Option<&SlimeGirlAi>,
        Option<&Children>,
    )>,
    q_organs: Query<(Entity, &Organ)>,
    mut ev: EventReader<AiShouldRunEvent>,
    mut writer: EventWriter<ActionEvent>,
) {
    // has a list of possible actions based on AI type
    // each of these actions is calculated, given a weight
    // for now, skip all this, just find the nearest actor and attack
    for ev in ev.read() {
        let slime = query.get(ev.0).unwrap();
        if slime.4.is_some() {
            let mut possible_targets = vec![];
            for target in &query {
                if slime.0 != target.0
                    && slime.3.get() == target.3.get()
                    && target.2.health_current > 0
                {
                    possible_targets.push((
                        50,
                        ActionEvent::Attack {
                            attacker: slime.0,
                            defender: target.0,
                        },
                    ));
                    // make sure the organ exists, and we'll probably want to add a check for vore with each organ
                    let slime_organ = q_organs
                        .get(*slime.5.expect("missing organ!").iter().next().unwrap())
                        .unwrap();
                    if slime_organ.1.fullness_current + 3 /* size of prey */ <= slime_organ.1.capacity
                    {
                        possible_targets.push((
                            10,
                            ActionEvent::Devour {
                                attacker: slime.0,
                                defender: target.0,
                                organ: slime_organ.0,
                            },
                        ));
                    }
                }
            }

            // picks target, creates attack event. Unique events for every possible action?
            if let Ok(windex) = WeightedIndex::new(possible_targets.iter().map(|item| item.0)) {
                let mut rng = thread_rng();
                let chosen_action = possible_targets.swap_remove(windex.sample(&mut rng)).1;
                writer.send(chosen_action);
            }
        }
    }
}

pub fn run_attack(
    mut reader: EventReader<ActionEvent>,
    mut q_actors: Query<(Entity, &Name, &mut Actor)>,
) {
    for ev in reader.read() {
        if let ActionEvent::Attack { attacker, defender } = ev {
            let actors = q_actors.get_many_mut([*attacker, *defender]);
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

// having just one system handle every possible action will make the query params
// too complicated to reason with, and too hard to expand.
// since any number of systems can respond to an event, I should instead have one
// system for each action type.
fn run_devour(
    mut reader: EventReader<ActionEvent>,
    q_organs: Query<(Entity, &Organ)>,
    q_names: Query<&Name>,
    mut commands: Commands,
) {
    for ev in reader.read() {
        if let ActionEvent::Devour {
            attacker,
            defender,
            organ,
        } = ev
        {
            // do some calculation based on the stats of the parent to determine if success

            // sets parent of target to the organ
            let organ = q_organs
                .get(*organ)
                .expect("Organ has vanished before using!");
            commands.entity(*defender).set_parent(organ.0);
            let pred = q_names.get(*attacker).expect("Actor should have name!");
            let prey = q_names.get(*defender).expect("Actor should have name!");
            let organ_name = q_names.get(organ.0).expect("Organ should have name!");
            println!("{} devours {} with their {}!", pred, prey, organ_name);
        }
    }
}

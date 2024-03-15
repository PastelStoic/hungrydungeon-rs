use bevy::prelude::*;

use crate::game::{
    actors::{organs::Organ, Actor},
    decision_table::DecisionTable,
};

use super::{AiBehavior, Monster};

pub struct Slimegirl;

impl Monster for Slimegirl {
    fn run_ai(entity: In<Entity>, world: &mut World) {
        let mut q_actors = world.query::<(Entity, &Name, &Actor, &Parent, Option<&Children>)>();
        let mut q_organs = world.query::<(Entity, &Organ)>();
    
        let slime = q_actors.get(world, *entity).unwrap();
        // make sure the organ exists, and we'll probably want to add a check for vore with each organ
        let slime_organ = q_organs
            .get(
                world,
                *slime.4.expect("missing organ!").iter().next().unwrap(),
            )
            .unwrap();
    
        let mut possible_actions = DecisionTable::new();
    
        for target in q_actors.iter(world) {
            if slime.0 != target.0 && slime.3.get() == target.3.get() && target.2.health_current > 0 {
                possible_actions.push((
                    50,
                    ActionEvent::Attack {
                        attacker: slime.0,
                        defender: target.0,
                    },
                ));
                if slime_organ.1.fullness_current + 3 /* size of prey */ <= slime_organ.1.capacity {
                    possible_actions.push((
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
        if let Some(chosen_action) = possible_actions.decide() {
            match chosen_action {
                ActionEvent::Attack { attacker, defender } => {
                    let sys = world.register_system(run_attack);
                    world
                        .run_system_with_input(sys, (attacker, defender))
                        .unwrap();
                }
                ActionEvent::Devour {
                    attacker,
                    defender,
                    organ,
                } => {
                    let sys = world.register_system(run_devour);
                    world
                        .run_system_with_input(sys, (attacker, defender, organ))
                        .unwrap();
                }
            }
        }
    }

    fn create_actor(world: &mut World) -> Entity {
        world
            .spawn((
                Name::new("Slimegirl"),
                Actor {
                    health_current: 100,
                    health_max: 100,
                    attack: 10,
                    defense: 10,
                },
            ))
            .with_children(|owner| {
                owner.spawn((Name::new("Slimegirl stomach"), Organ::default()));
            })
            .id()
    }
}

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

pub fn run_attack(
    In((attacker, defender)): In<(Entity, Entity)>,
    mut q_actors: Query<(Entity, &Name, &mut Actor)>,
) {
    let actors = q_actors.get_many_mut([attacker, defender]);
    if let Ok([attacker, mut target]) = actors {
        target.2.health_current -= attacker.2.attack;
        println!(
            "{} attacks {}, dealing {} damage!",
            attacker.1, target.1, attacker.2.attack
        );
    }
}

// having just one system handle every possible action will make the query params
// too complicated to reason with, and too hard to expand.
// since any number of systems can respond to an event, I should instead have one
// system for each action type.
fn run_devour(
    In((attacker, defender, organ)): In<(Entity, Entity, Entity)>,
    q_organs: Query<(Entity, &Organ)>,
    q_names: Query<&Name>,
    mut commands: Commands,
) {
    // do some calculation based on the stats of the parent to determine if success

    // sets parent of target to the organ
    let organ = q_organs
        .get(organ)
        .expect("Organ has vanished before using!");
    commands.entity(defender).set_parent(organ.0);
    let pred = q_names.get(attacker).expect("Actor should have name!");
    let prey = q_names.get(defender).expect("Actor should have name!");
    let organ_name = q_names.get(organ.0).expect("Organ should have name!");
    println!("{} devours {} with their {}!", pred, prey, organ_name);
}

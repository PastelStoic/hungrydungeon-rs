use bevy::prelude::*;

use crate::game::{actors::Actor, decision_table};

use super::AiBehavior;

pub fn spawn(world: &mut World) -> Entity {
    let ai = world.register_system(slime_ai);
    world
        .spawn((
            Name::new("Slime"),
            AiBehavior(ai),
            Actor {
                health_current: 100,
                health_max: 100,
                attack: 10,
                defense: 10,
            },
        ))
        .id()
}

// this runs once per slime, on the slime
fn slime_ai(In(entity): In<Entity>, mut query: Query<(Entity, &Name, &mut Actor)>) {
    let slime = query.get(entity).unwrap();
    let mut possible_targets = decision_table::DecisionTable::new();
    for target in &query {
        if slime.0 != target.0 && target.2.health_current > 0 {
            possible_targets.push((1, target.0));
        }
    }

    // picks target, creates attack event. Unique events for every possible action?
    if let Some(target) = possible_targets.decide() {
        run_attack(slime.0, target, &mut query);
    }
}

pub fn run_attack(
    attacker: Entity,
    defender: Entity,
    query: &mut Query<(Entity, &Name, &mut Actor)>,
) {
    let actors = query.get_many_mut([attacker, defender]);
    if let Ok([attacker, mut target]) = actors {
        target.2.health_current -= attacker.2.attack;
        println!(
            "{} attacks {}, dealing {} damage!",
            attacker.1, target.1, attacker.2.attack
        );
    }
}

use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::*};

use crate::game::actors::Actor;

use super::AiBehavior;

pub fn spawn(parent: &mut ChildBuilder, world: &mut World) {
    let ai = world.register_system(slime_ai);
    parent.spawn((
        Name::new("Slime"),
        AiBehavior(ai),
        Actor {
            health_current: 100,
            health_max: 100,
            attack: 10,
            defense: 10,
        },
    ));
}


// this runs once per slime, on the slime
fn slime_ai(In(entity): In<Entity>, mut query: Query<(Entity, &Name, &mut Actor)>) {
    let slime = query.get(entity).unwrap();
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
        run_attack(slime.0, target, &mut query)
    }
}

pub fn run_attack(
    attacker: Entity,
    defender: Entity,
    mut query: &mut Query<(Entity, &Name, &mut Actor)>,
) {
    let actors = query.get_many_mut([attacker, defender]);
    if let Ok([attacker, mut target]) = actors {
        // check if the slime is still active, if the target is still in reach, if its still alive
        // the "is this target valid" check should be the same code both here and above
        target.1.health_current -= attacker.1.attack;
        println!(
            "{} attacks {}, dealing {} damage!",
            attacker.0, target.0, attacker.1.attack
        );
    }
}
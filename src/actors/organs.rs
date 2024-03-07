use bevy::prelude::*;

use super::Actor;

pub struct OrganPlugin;

impl Plugin for OrganPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrganTimer(Timer::from_seconds(10., TimerMode::Repeating)))
            // Ai decision trees go here
            .add_systems(Update, (tick_timer, digest));
    }
}

#[derive(Component)]
pub struct Organ {
    pub health_current: i32,
    pub health_max: i32,
    pub attack: i32,
    pub defense: i32,
    pub capacity: i32,
    pub fullness_current: i32,
    pub organ_type: OrganType,
}

/// Determines specific behavior of an organ
pub enum OrganType {
    Generic,
    Womb,
    Breast,
    // slime organ, allowing control of which specific actors inside it are digested.
}

#[derive(Resource)]
pub struct OrganTimer(Timer);

fn tick_timer(mut timer: ResMut<OrganTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn digest(
    q_organs: Query<(&Organ, &Name, &Children)>,
    mut q_prey: Query<(&Name, &mut Actor)>,
    aitimer: Res<OrganTimer>,
) {
    if !aitimer.0.finished() {
        return;
    }

    for organ in &q_organs {
        for prey in organ.2.iter() {
            if let Ok(mut prey) = q_prey.get_mut(*prey) {
                prey.1.health_current -= organ.0.attack;
                println!(
                    "{} digests {} for {} damage!",
                    organ.1, prey.0, organ.0.attack
                );
            }
        }
    }
}

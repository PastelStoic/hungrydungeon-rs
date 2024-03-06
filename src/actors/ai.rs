pub mod slime;
pub mod slimegirl;

use bevy::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiTimer(Timer::from_seconds(10., TimerMode::Repeating)))
            // Ai decision trees go here
            .add_systems(Update, tick_ai_timer)
            .add_plugins(slime::SlimeAiPlugin);
    }
}

#[derive(Resource)]
pub struct AiTimer(Timer);

fn tick_ai_timer(mut aitimer: ResMut<AiTimer>, time: Res<Time>) {
    aitimer.0.tick(time.delta());
}

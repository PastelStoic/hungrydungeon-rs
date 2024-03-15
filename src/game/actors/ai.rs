pub mod slime;
pub mod slimegirl;

use bevy::{ecs::system::SystemId, prelude::*};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiTimer(Timer::from_seconds(10., TimerMode::Repeating)))
            .add_systems(Update, tick_ai_timer);
    }
}

#[derive(Resource)]
pub struct AiTimer(Timer);

#[derive(Component)]
pub struct AiBehavior(SystemId<Entity>);

fn tick_ai_timer(mut aitimer: ResMut<AiTimer>, time: Res<Time>, query: Query<(Entity, &AiBehavior)>, mut commands: Commands) {
    aitimer.0.tick(time.delta());
    if aitimer.0.finished() {
        for ai in &query {
            commands.run_system_with_input(ai.1.0, ai.0);
        }
    }
}

// experimenting with making AI creation easier
pub trait Monster where Self: 'static {
    fn run_ai(entity: In<Entity>, world: &mut World);
    fn create_actor(world: &mut World) -> Entity;
    fn spawn(world: &mut World) -> Entity {
        let ai = world.register_system(Self::run_ai);
        let spawned = Self::create_actor(world);
        world.entity_mut(spawned).insert(AiBehavior(ai));
        spawned
    }
}

fn example_simplified() {

}
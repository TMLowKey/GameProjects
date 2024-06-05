use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::game::SimulationState;
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 5;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>();
        app.add_systems(OnEnter(AppState::Game), spawn_enemies);
        app.add_systems(
            Update,
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemy_over_time,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        // Exit State Systems
        app.add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}

use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use super::AppState;
use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player);
        // Configure System sets
        app.configure_sets(
            Update,
            MovementSystemSet
                .before(ConfinementSystemSet)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(
            Update,
            (
                // Same can be done with .after, .before
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet),
                player_hit_star,
                enemy_hit_player,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
        app.add_systems(OnExit(AppState::Game), despawn_player);
    }
}

use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(mut simulations_state_next_state: ResMut<NextState<SimulationState>>) {
    simulations_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulations_state_next_state: ResMut<NextState<SimulationState>>) {
    simulations_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simlation Paused.");
        }
        if **simulation_state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simlation Running.");
        }
    }
}

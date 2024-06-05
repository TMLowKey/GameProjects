pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

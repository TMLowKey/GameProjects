use bevy::prelude::Event;

pub struct GameOver {
    pub score: u32,
}

impl Event for GameOver {}

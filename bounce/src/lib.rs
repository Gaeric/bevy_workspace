use bevy::prelude::*;

mod config;
mod background;
mod game;
mod score;
mod menu;
mod utils;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum AppState {
    Loading,
    Menu,
    Setting,
    Battle,
    Practice,
    Win,
}

#[derive(Component)]
pub struct ColorText {
    timer: Timer,
    colors: Vec<Color>,
    index: usize,
}

impl ColorText {
    pub fn new(colors: Vec<Color>, duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, true),
            colors,
            index: 0,
        }
    }
}


pub struct TimeScale(pub f32);

impl Default for TimeScale {
    fn default() -> Self {
        Self(1.0)
    }
}

impl TimeScale {
    pub fn reset(&mut self) {
        self.0 = 1.0;
    }
}


pub fn run() {
    let mut app = App::new();
    app.init_resource::<TimeScale>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Win)
        .add_startup_system(setup)
        .add_plugin(score::ScorePlugin)
        .add_plugin(background::BackgroundPlugin);
    app.run();
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

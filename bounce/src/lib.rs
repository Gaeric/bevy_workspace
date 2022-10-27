use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_kira_audio::AudioPlugin;

mod background;
mod config;
mod game;
mod menu;
mod score;
mod utils;
mod loading;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum AppState {
    Loading,
    Menu,
    Settings,
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

#[derive(Component)]
pub struct HintText {
    index: usize,
    timer: Timer,
}

impl HintText {
    const HINT_TEXTS: [&'static str; 3] = [
        "Control your ball speed!",
        "Can your paddle catch the ball on its own?",
        "Try to bounce; not to push!",
    ];

    pub fn new(duration: f32) -> Self {
        Self {
            index: 0,
            timer: Timer::from_seconds(duration, true),
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

pub struct AudioVolume {
    pub music: f32,
    pub effects: f32,
}

pub struct MusicTrack(&'static str);

pub fn run() {
    let mut app = App::new();
    app
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Bounce UP!".into(),
            width: config::ARENA_WIDTH,
            height: config::ARENA_HEIGHT,
            resizable: false,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .init_resource::<TimeScale>()
        .insert_resource(AudioVolume {
            music: 0.3,
            effects: 1.0,
        })
        .insert_resource(MusicTrack(""));

    app
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_state(AppState::Loading)
        .add_startup_system(setup)
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(score::ScorePlugin)
        .add_plugin(background::BackgroundPlugin);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

use bevy::prelude::*;

mod config;
mod background;


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
        .add_startup_system(setup)
        .add_plugin(background::BackgroundPlugin);
    app.run();
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

use crate::{
    config::*,
    AppState, ColorText, TimeScale,
};
use bevy::prelude::*;

pub struct MenuPlugin;

struct ButtonAudio;

const NORMAL_BUTTON: Color = Color::NONE;
const HOVERED_BUTOON: Color = Color::WHITE;
const PRESSED_BUTTON: Color = Color::WHITE;

const NORMAL_SETTING_BUTTON: Color = Color::BLACK;


impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonStyle>()
            // .add_audio_channel::<ButtonAudio>()
            .add_system_set(
                SystemSet::new()
                    .label(CuttonSystems)
                    .with_system(button_system)
                    .with_system(button_action)
                    .with_system(value_system)
                    .with_system(value_action),
            )
            // .add_system(button_audio.after(ButtonSystems))
            .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    .with_system(enter_menu)
                    .with_system(make_menu),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Menu).with_system(cleanup_system::<Cleanup>),
            )
            .add_system_set(SystemSet::on_enter(AppState::Settings).with_system(make_settings))
            .add_system_set(SystemSet::on_update(AppState::Settings).with_system(escape_system))
            .add_system_set(
                SystemSet::on_exit(AppState::Settings).with_system(cleanup_system::<Cleanup>),
            );
    }
}

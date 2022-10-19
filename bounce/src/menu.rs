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
        
    }
}

use crate::{
    config::*,
    utils::{cleanup_system, escape_system},
    AppState, ColorText, TimeScale, HintText, AudioVolume, MusicTrack
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioControl};

pub struct MenuPlugin;

struct ButtonAudio;


impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonStyle>()
            // .add_audio_channel::<ButtonAudio>()
            .add_system_set(
                SystemSet::new()
                    .label(ButtonSystems)
                    // .with_system(button_system)
                    // .with_system(button_action)
                    // .with_system(value_system)
                    // .with_system(value_action),
            )
            // .add_system(button_audio.after(ButtonSystems))
            .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    // .with_system(enter_menu)
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

const NORMAL_BUTTON: Color = Color::NONE;
const HOVERED_BUTOON: Color = Color::WHITE;
const PRESSED_BUTTON: Color = Color::WHITE;

const NORMAL_SETTING_BUTTON: Color = Color::BLACK;
const ACTIVE_SETTING_BUTTON: Color = Color::WHITE;
const HOVERED_SETTING_BUTTON: Color = Color::GRAY;

const NORMAL_BUTTON_TEXT: Color = Color::WHITE;
const HOVERED_BUTTON_TEXT: Color = Color::BLACK;
const PRESSED_BUTTON_TEXT: Color = Color::BLACK;

#[derive(Component)]
struct Cleanup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct ButtonSystems;

#[derive(Clone, Copy, Component)]
enum ButtonAction {
    Play,
    Tutorial,
    Settings,
    Back,
}

#[derive(Clone, Copy, Component)]
enum ValueAction {
    AudioVolume(f32),
    MusicVolume(f32),
}

struct ButtonStyle {
    button: Style,
    icon: Style,
    text: TextStyle,
}

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ButtonStyle {
            button: Style {
                size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                position: UiRect {
                    left: Val::Percent(10.0),
                    ..default()
                },
                margin: UiRect {
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            icon: Style {
                size: Size::new(Val::Px(20.0), Val::Auto),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                ..default()
            },
            text: TextStyle {
                font: asset_server.load(FONT_KARMATIC),
                font_size: 20.0,
                color: NORMAL_BUTTON_TEXT,
            },
        }
    }
}

fn enter_menu(
    mut time_scale: ResMut<TimeScale>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    volume: Res<AudioVolume>,
    mut music_track: ResMut<MusicTrack>,
) {
    // time_scale.reset();
    // if music_track.0 != MENU_MUSIC {
    //     audio.stop();
    //     audio.set_playback_rate(1.0);
    //     audio.set_volume(volume.music.into());
    //     audio.play(asset_server.load(MENU_MUSIC)).looped();

    //     music_track.0 = MENU_MUSIC;
    // }
}

fn make_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_style: Res<ButtonStyle>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Cleanup)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Percent(10.0),
                        ..default()
                    },
                    margin: UiRect {
                        bottom: Val::Percent(20.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Bounce Up!",
                    TextStyle {
                        font: asset_server.load(FONT_ARCADE),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..default()
                    }),
                ..default()
            })
                .insert(ColorText::new(
                    FLIP_TEXT_COLORS.into(),
                    30.0 / MENU_MUSIC_BPM,
            ));

            parent.spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Percent(10.0),
                        top: Val::Percent(40.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load(FONT_INVASION),
                        font_size: 15.0,
                        color: HEALTH_BAR_COLOR,
                    },
                )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        ..default()
                    }),
                ..default()
            })
            .insert(HintText::new(480.0 / MENU_MUSIC_BPM));

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.button.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonAction::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: button_style.icon.clone(),
                        image: UiImage(asset_server.load(RIGHT_ICON)),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section("Play", button_style.text.clone()),
                        ..default()
                    });
                });

            // parent.spawn_bundle(ButtonBundle {
            //     style: button_style.button.clone(),
            //     color: NORMAL_BUTTON.into(),
            //     ..default()
            // })
            //     .insert(ButtonAction::Tutorial)
            //     .with_children(|parent| {
            //         parent.spawn_bundle(ImageBundle {
            //             style: button_style.icon.clone(),
            //             image: UiImage(asset_server.load(RETICLE_ICON)),
            //             ..default()
            //         });
            //         parent.spawn_bundle(TextBundle {
            //             text: Text::from_section("Practice", button_style.text.clone()),
            //             ..default()
            //         });
            //     });

            parent.spawn_bundle(ButtonBundle {
                style: button_style.button.clone(),
                color: NORMAL_BUTTON.into(),
                ..default()
            })
                .insert(ButtonAction::Settings)
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: button_style.icon.clone(),
                        image: UiImage(asset_server.load(WRENCH_ICON)),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section("Settings", button_style.text.clone()),
                        ..default()
                    });
                });
        });
}


fn make_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_style: Res<ButtonStyle>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Cleanup)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Percent(10.0),
                        ..default()
                    },
                    margin: UiRect {
                        bottom: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "Settings",
                    TextStyle {
                        font: asset_server.load(FONT_KARMATIC),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ).with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                }),
                ..default()
            });
            
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.button.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonAction::Back)
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: button_style.icon.clone(),
                        image: UiImage(asset_server.load(EXIT_ICON)),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section("Back", button_style.text.clone()),
                        ..default()
                    });
                });

        });
}

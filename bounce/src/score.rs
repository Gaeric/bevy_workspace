use crate::{config::*, AppState, ColorText, TimeScale};
use bevy::prelude::*;

pub struct Score {
    pub timestamp: f64,
    pub hits: i32,
    pub miss: i32,
}

impl FromWorld for Score {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        Self {
            timestamp: time.seconds_since_startup(),
            hits: 0,
            miss: 0,
        }
    }
}

fn enter_score(mut time_scale: ResMut<TimeScale>) {
    time_scale.reset();
}

fn make_ui(
    mut commands: Commands,
    time: Res<Time>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
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
                        "You Win!",
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

            let term_style = Style {
                size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                position: UiRect {
                    left: Val::Percent(10.0),
                    ..default()
                },
                margin: UiRect {
                    top: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            };

            let time_passed = time.seconds_since_startup() - score.timestamp;
            parent.spawn_bundle(TextBundle {
                style: term_style.clone(),
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Time: ".into(),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("{:2}", time_passed),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::GOLD,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                style: term_style.clone(),
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Hits: ".into(),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: score.hits.to_string(),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::GOLD
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });

            parent.spawn_bundle(TextBundle {
                style: term_style,
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Miss: ".into(),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: score.miss.to_string(),
                            style: TextStyle {
                                font: asset_server.load(FONT_KARMATIC),
                                font_size: 20.0,
                                color: Color::GOLD,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        });
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_system_set(
            SystemSet::on_enter(AppState::Win)
                .with_system(enter_score)
                .with_system(make_ui)
        );
    }
}

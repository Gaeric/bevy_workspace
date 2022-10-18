use self::{
    ball::*, base::*, battle::*, effects::*, enemy::*, hint::*, physics::*, player::*, slits::*,
};
use crate::{config::*, score::Score};

mod ball;
mod base;
mod battle;
mod effects;
mod enemy;
mod hint;
mod physics;
mod player;
mod slits;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_event::<PlayerHitEvent>()
            .add_event::<PlayerMissEvent>()
            .add_event::<BounceEvent>()
            .add_event::<HealEvent>()
            .insert_resource(Debounce {
                audio_bounce_long: Timer::from_seconds(0.5, false),
                audio_bounce_short: Timer::from_seconds(0.1, false),
                audio_hit: Timer::from_seconds(0.1, false),
                bounce: Timer::from_seconds(0.1, false),
                effects: Timer::from_seconds(0.1, false),
                hit: Timer::from_seconds(0.1, false),
                miss: Timer::from_seconds(0.5, false),
            })
            .init_resource::<Slits>()
            .add_audio_channel::<BounceAudioChannel>()
            .add_audio_channel::<ScoreAudioChannel>()
            .add_startup_system(setup_game)
            .add_system_set(
                SystemSet::new()
                    .with_system(move_player)
                    .with_system(assist_player)
                    .with_system(move_enemy)
                    .with_system(move_ball)
                    .with_system(activate_ball)
                    .with_system(update_ball)
                    .with_system(ball_bounce)
                    .with_system(heal_enemy_base)
                    .with_system(move_slit_block)
                    .with_system(slits_system)
                    .with_system(bounce_audio)
                    .with_system(score_audio)
                    .with_system(score_effects)
                    .with_system(bounce_effects)
                    .with_system(count_ball)
                    .with_system(score_system)
                    .with_system(health_bar)
                    .with_system(health_bar_tracker)
                    .with_system(make_player_hint)
                    .with_system(make_ball_hint)
                    .with_system(hint_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(AI_TIME_STEP))
                    .with_system(predict_ball)
                    .with_system(control_enemy),
            )
            .add_plugin(PhysicsPlugin)
            .add_plugin(EffectsPlugin)
            .add_plugin(BattlePlugin);
    }
}

struct BounceAudioChannel;

struct ScoreAudioChannel;

#[derive(Clone, Copy)]
enum GameOverEvent {
    Win,
    Lose,
}

struct PlayerHitEvent {
    ball: Entity,
    location: Vec2,
    win: bool,
}

struct PlayerMissEvent {
    ball: Entity,
    location: Vec2,
    lose: bool,
}

struct BounceEvent {
    ball: Entity,
    other: Entity,
    location: Vec2,
}

struct Debounce {
    audio_bounce_long: Timer,
    audio_bounce_short: Timer,
    audio_hit: Timer,

    bounce: Timer,
    effects: Timer,
    hit: Timer,
    miss: Timer,
}

struct GameOver {
    slow_motion_timer: Timer,
    state_change_timer: Timer,
    event: Option<GameOverEvent>,
}

impl Default for GameOver {
    fn default() -> Self {
        Self {
            slow_motion_timer: Timer::from_seconds(GAME_OVER_SLOW_MOTION_DURATION, false),
            state_change_timer: Timer::from_seconds(GAME_OVER_STATE_CHANGE_DURATION, false),
            event: None,
        }
    }
}

#[derive(Component)]
struct Cleanup;

#[derive(Clone, Copy, PartialEq, Eq, Component)]
enum BounceAudio {
    Bounce,
    Hit,
}

struct Materials {
    player: Handle<Image>,
    enemy: Handle<Image>,
    ball: Handle<Image>,
    hint: Handle<Image>,
    death: Handle<Image>,
    hit: Handle<TextureAtlas>,
}

struct Audios {
    hit_audio: Handle<AudioSource>,
    miss_audio: Handle<AudioSource>,
    explosion_audio: Handle<AudioSource>,
    lose_audio: Handle<AudioSource>,
    impact_audios: Vec<Handle<AudioSource>>,
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(Materials {
        player: asset_server.load(PLAYER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        ball: asset_server.load(BALL_SPRITE),
        hint: asset_server.load(HINT_SPRITE),
        death: asset_server.load(DEATH_SPRITE),
        hit: texture_atlases.add(TextureAtlas::from_grid(
            asset_server.load(HIT_SPRITE),
            Vec2::new(1024.0, 1024.0),
            4,
            4,
        )),
    });

    commands.insert_resource(Audios {
        hit_audio: asset_server.load(HIT_AUDIO),
        miss_audio: asset_server.load(MISS_AUDIO),
        explosion_audio: asset_server.load(EXPLOSION_AUDIO),
        lose_audio: asset_server.load(LOSE_AUDIO),
        impact_audios: IMPACT_AUDIOS
            .iter()
            .map(|path| asset_server.load(*path))
            .collect_vec(),
    });

    commands.innit_resource::<Score>();
}

fn make_arena(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(ARENA_WIDTH, 32.0)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(ARENA_WIDTH, 32.0), 0.0, 0.9, 0.5),
            PhysicsLayers::SPEARATE,
            Cleanup,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0.0, -8.0, 0.0),
                sprite: Sprite {
                    color: SEPARATE_COLOR,
                    custom_size: Some(Vec2::new(ARENA_WIDTH, 16.0)),
                    ..default()
                },
                ..default()
            });
        });

    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, ARENA_HEIGHT * 0.5 + 16.0, 0.0),
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(ARENA_WIDTH, 32.0)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(ARENA_WIDTH, 32.0), 0.0, 0.9, 0.0),
            PhysicsLayers::BOUNDARY,
            BounceAudio::Hit,
            EnemyBase::default(),
            Cleanup,
        ));

    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, -ARENA_HEIGHT * 0.5 - 16.0, 0.0),
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(ARENA_WIDTH, 32.0)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(ARENA_WIDTH, 32.0), 0.0, 0.0, 0.5),
            PhysicsLayers::BOUNDARY,
            PlayerBase::default(),
            Cleanup,
        ));

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-ARENA_WIDTH * 0.5 - 16.0, 0.0, 0.0),
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(32.0, ARENA_HEIGHT + 64.0)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(32.0, ARENA_HEIGHT + 64.0), 0.0, 1.0, 0.0),
            PhysicsLayers::BOUNDARY,
            BounceAudio::Bounce,
            Cleanup,
        ));

    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(ARENA_WIDTH * 0.5 + 16.0, 0.0, 0.0),
            sprite: Sprite {
                color: BOUNDARY_COLOR,
                custom_size: Some(Vec2::new(32.0, ARENA_HEIGHT + 64.0)),
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(32.0, ARENA_HEIGHT + 64.0), 0.0, 1.0, 0.0),
            PhysicsLayers::BOUNDARY,
            BounceAudio::Bounce,
            Cleanup,
        ));
}

fn make_ui(mut commands: Commands, materials: Res<Materials>, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(4.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Cleanup)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: HEALTH_BAR_COLOR.into(),
                    ..default()
                })
                .insert(HealthBar);
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: HEALTH_BAR_TRACKER_COLOR.into(),
                    ..default()
                })
                .insert(HealthBarTracker::default());
        });
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(16.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Cleanup)
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                    ..default()
                },
                image: materials.ball.clone().into(),
                ..default()
            });

            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: " x ".into(),
                                style: TextStyle {
                                    font: asset_server.load(FONT_FIRA_MONO),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".into(),
                                style: TextStyle {
                                    font: asset_server.load(FONT_FIRA_MONO),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(BallCounter);
        });
}

fn make_player(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, -160.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert_bundle((
            RigidBody::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT), 3.0, 2.0, 1.0),
            Motion::default(),
            PhysicsLayers::PLAYER,
            BounceAudio::Bounce,
            Controller::default(),
            MotionOverride::default(),
            Player::default(),
            PlayerAssist::default(),
            Cleanup,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(-PADDLE_WIDTH / 2.0 + 8.0, 0.0, 0.1),
                texture: materials.player.clone(),
                ..default()
            });
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(PADDLE_WIDTH / 2.0 - 8.0, 0.0, 0.1),
                texture: materials.player.clone(),
                ..default()
            })
        });
}

use bevy::{prelude::*, sprite::collide_aabb::*};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum GameState {
    MainMenu,
    Playing,
}

#[derive(Resource)]
struct Scoreboard {
    score: i32,
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Block;

const WINDOW_HEIGHT: f32 = 640.0;
const WINDOW_WIDTH: f32 = 380.0;

const PADDLE_SPEED: f32 = 190.0;
const PADDLE_WIDTH: f32 = 38.0;

const BLOCK_WIDTH: f32 = 20.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(100.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play!",
                TextStyle {
                    font: asset_server.load("pixel_font.ttf"),
                    ..default()
                },
            ));
        });
}

fn start_game(interaction_query: Query<&Interaction>, mut state: ResMut<State<GameState>>) {
    for interaction in &interaction_query {
        if let Interaction::Clicked = interaction {
            state.set(GameState::Playing).ok();
        }
    }
}

fn cleanup_menu(mut commands: Commands, node_query: Query<Entity, With<Node>>) {
    for id in node_query.iter() {
        commands.entity(id).despawn();
    }
}

fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("pixel_font.ttf"),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("pixel_font.ttf"),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, 10.0, 10.0),
                translation: Vec3::new(0.0, -(WINDOW_HEIGHT / 2.0) + 25.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn setup_ball(mut commands: Commands) {
    commands.spawn((
        Ball,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        },
        Velocity(Vec2::new(PADDLE_SPEED, PADDLE_SPEED)),
    ));
}

fn setup_blocks(mut commands: Commands) {
    let gap = 1.0;
    let rows = ((WINDOW_HEIGHT / 4.0 / 10.0) - gap * 2.0) as i32;
    let columns = ((WINDOW_WIDTH / BLOCK_WIDTH) - gap * 2.0) as i32;
    for row in 0..rows {
        for column in 0..columns {
            commands.spawn((
                Block,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(18.0, 8.0, 10.0),
                        translation: Vec3::new(
                            (WINDOW_WIDTH / 2.0)
                                - ((column as f32 + gap) * BLOCK_WIDTH)
                                - (BLOCK_WIDTH / 2.0),
                            (WINDOW_HEIGHT / 2.0) - 5.0 - ((row as f32 + gap) * 10.0),
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let Some(mut player_transform) = player_transform_query.iter_mut().next() else { return; };
    if keyboard_input.pressed(KeyCode::Left) {
        player_transform.translation.x -= time.delta_seconds() * PADDLE_SPEED;
    } 
    if keyboard_input.pressed(KeyCode::Right) {
        player_transform.translation.x += time.delta_seconds() * PADDLE_SPEED;
    }
    let bounds = WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    player_transform.translation.x = player_transform.translation.x.clamp(-bounds, bounds);
}

fn ball_movement(
    time: Res<Time>,
    mut ball_transform_query: Query<&mut Transform, With<Ball>>,
    ball_velocity_query: Query<&Velocity, With<Ball>>,
) {
    let Some(mut ball_transform) = ball_transform_query.iter_mut().next() else { return; };
    let Some(ball_velocity) = ball_velocity_query.iter().next() else { return; };
    ball_transform.translation.x += time.delta_seconds() * ball_velocity.0.x;
    ball_transform.translation.y += time.delta_seconds() * ball_velocity.0.y;
}

fn ball_bounds_collision(
    ball_transform_query: Query<&Transform, With<Ball>>,
    mut ball_velocity_query: Query<&mut Velocity, With<Ball>>,
) {
    let Some(ball_transform) = ball_transform_query.iter().next() else { return; };
    let Some(mut ball_velocity) = ball_velocity_query.iter_mut().next() else { return; };

    let x_bounds = WINDOW_WIDTH / 2.0 - 5.0;
    let y_bounds = WINDOW_HEIGHT / 2.0 - 5.0;

    if ball_transform.translation.x.abs() > x_bounds {
        ball_velocity.0.x *= -1.0;
    }
    if ball_transform.translation.y.abs() > y_bounds {
        ball_velocity.0.y *= -1.0;
    }
}

fn ball_blocks_collision(
    mut commands: Commands,
    ball_transform_query: Query<&Transform, With<Ball>>,
    mut ball_velocity_query: Query<&mut Velocity, With<Ball>>,
    block_transforms_query: Query<(Entity, &Transform), With<Block>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let Some(ball_transform) = ball_transform_query.iter().next() else { return; };
    let Some(mut ball_velocity) = ball_velocity_query.iter_mut().next() else { return; };

    let ball_size = ball_transform.scale.truncate();
    for (block_id, block_transform) in block_transforms_query.iter() {
        if let Some(collision) = collide(
            ball_transform.translation,
            ball_size,
            block_transform.translation,
            block_transform.scale.truncate(),
        ) {
            scoreboard.score += 1;
            commands.entity(block_id).despawn();

            match collision {
                Collision::Bottom | Collision::Top => {
                    ball_velocity.0.y *= -1.0;
                    return;
                }
                Collision::Left | Collision::Right => {
                    ball_velocity.0.x *= -1.0;
                    return;
                }
                _ => {}
            };
        }
    }
}

fn ball_player_collision(
    ball_transform_query: Query<&Transform, With<Ball>>,
    mut ball_velocity_query: Query<&mut Velocity, With<Ball>>,
    player_transform_query: Query<&Transform, With<Player>>,
) {
    let Some(ball_transform) = ball_transform_query.iter().next() else { return; };
    let Some(mut ball_velocity) = ball_velocity_query.iter_mut().next() else { return; };
    let Some(player_transform) = player_transform_query.iter().next() else { return; };

    if ball_velocity.0.y < 0.0 {
        if collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            player_transform.translation,
            player_transform.scale.truncate(),
        )
        .is_some()
        {
            ball_velocity.0.y *= -1.0;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut text_query: Query<&mut Text>) {
    let mut text = text_query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn main() {
    App::new()
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Breakout!".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.0, 0.0)))
        .insert_resource(Scoreboard { score: 0 })
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(start_game))
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup_menu))
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_scoreboard)
                .with_system(setup_player)
                .with_system(setup_ball)
                .with_system(setup_blocks),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_movement)
                .with_system(ball_movement)
                .with_system(ball_bounds_collision.after(ball_movement))
                .with_system(ball_player_collision.after(ball_bounds_collision))
                .with_system(ball_blocks_collision.after(ball_player_collision))
                .with_system(update_scoreboard),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

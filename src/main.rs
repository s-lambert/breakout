use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 640.0;
const WINDOW_WIDTH: f32 = 380.0;

const PADDLE_SPEED: f32 = 190.0;
const PADDLE_WIDTH: f32 = 38.0;

const BLOCK_WIDTH: f32 = 20.0;

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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let Some(mut transform) = player_transform.iter_mut().next() else { return; };
    let is_left_pressed = keyboard_input.pressed(KeyCode::Left);
    let is_right_pressed = keyboard_input.pressed(KeyCode::Right);
    if is_left_pressed && !is_right_pressed {
        transform.translation.x -= time.delta_seconds() * PADDLE_SPEED;
    } else if is_right_pressed && !is_left_pressed {
        transform.translation.x += time.delta_seconds() * PADDLE_SPEED;
    }
    let bounds = WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    transform.translation.x = transform.translation.x.clamp(-bounds, bounds);
}

fn ball_movement(
    time: Res<Time>,
    mut ball_transform: Query<&mut Transform, With<Ball>>,
    ball_velocity: Query<&Velocity, With<Ball>>,
) {
    let Some(mut transform) = ball_transform.iter_mut().next() else { return; };
    let Some(velocity) = ball_velocity.iter().next() else { return; };
    transform.translation.x += time.delta_seconds() * velocity.0.x;
    transform.translation.y += time.delta_seconds() * velocity.0.y;
}

fn ball_bounds_collision(
    ball_transform: Query<&Transform, With<Ball>>,
    mut ball_velocity: Query<&mut Velocity, With<Ball>>,
) {
    let Some(transform) = ball_transform.iter().next() else { return; };
    let Some(mut velocity) = ball_velocity.iter_mut().next() else { return; };

    let x_bounds = WINDOW_WIDTH / 2.0 - 5.0;
    let y_bounds = WINDOW_HEIGHT / 2.0 - 5.0;

    if transform.translation.x.abs() > x_bounds {
        velocity.0.x *= -1.0;
    }
    if transform.translation.y.abs() > y_bounds {
        velocity.0.y *= -1.0;
    }
}

fn ball_blocks_collision(
    mut commands: Commands,
    ball_transform: Query<&Transform, With<Ball>>,
    mut ball_velocity: Query<&mut Velocity, With<Ball>>,
    block_transforms: Query<(Entity, &Transform), With<Block>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let Some(transform) = ball_transform.iter().next() else { return; };
    let Some(mut velocity) = ball_velocity.iter_mut().next() else { return; };

    for (block, block_transform) in block_transforms.iter() {
        if aabb(transform, block_transform) {
            scoreboard.score += 1;
            commands.entity(block).despawn();

            let x_diff = (transform.translation.x - block_transform.translation.x).abs() / 15.0;
            let y_diff = (transform.translation.y - block_transform.translation.y).abs() / 10.0;
            if x_diff > y_diff {
                velocity.0.x *= -1.0;
                return;
            } else {
                velocity.0.y *= -1.0;
                return;
            }
        }
    }
}

fn ball_player_collision(
    ball_transform: Query<&Transform, With<Ball>>,
    mut ball_velocity: Query<&mut Velocity, With<Ball>>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let Some(transform) = ball_transform.iter().next() else { return; };
    let Some(mut velocity) = ball_velocity.iter_mut().next() else { return; };
    let Some(player) = player_transform.iter().next() else { return; };

    if aabb(transform, player) {
        velocity.0.y *= -1.0;
    }
}

fn aabb(a: &Transform, b: &Transform) -> bool {
    let collision_x = a.translation.x + a.scale.x / 2.0 >= b.translation.x - b.scale.x / 2.0
        && b.translation.x + b.scale.x / 2.0 >= a.translation.x - a.scale.x / 2.0;
    let collision_y = a.translation.y + a.scale.y / 2.0 >= b.translation.y - b.scale.y / 2.0
        && b.translation.y + b.scale.y / 2.0 >= a.translation.y - a.scale.y / 2.0;
    return collision_x && collision_y;
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn main() {
    App::new()
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
        .add_startup_system(setup_scoreboard)
        .add_startup_system(setup_player)
        .add_startup_system(setup_ball)
        .add_startup_system(setup_blocks)
        .add_system(bevy::window::close_on_esc)
        .add_system(player_movement)
        .add_system(ball_movement)
        .add_system(ball_bounds_collision.after(ball_movement))
        .add_system(ball_player_collision.after(ball_bounds_collision))
        .add_system(ball_blocks_collision.after(ball_player_collision))
        .add_system(update_scoreboard)
        .run();
}

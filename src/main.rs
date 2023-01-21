use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 640.0;
const WINDOW_WIDTH: f32 = 380.0;

const PADDLE_SPEED: f32 = 190.0;
const PADDLE_WIDTH: f32 = 38.0;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn ball_collision(
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
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_startup_system(setup_ball)
        .add_system(bevy::window::close_on_esc)
        .add_system(player_movement)
        .add_system(ball_movement)
        .add_system(ball_collision.after(ball_movement))
        .run();
}

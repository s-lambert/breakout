use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 640.0;
const WINDOW_WIDTH: f32 = 380.0;

const PADDLE_SPEED: f32 = 190.0;
const PADDLE_WIDTH: f32 = 38.0;

#[derive(Component)]
struct Player;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(PADDLE_WIDTH, 10.0, 10.0),
                // translation: Vec3::new(20.0, 20.0, 20.0),
                translation: Vec3::new(0.0, -(WINDOW_HEIGHT / 2.0) + 25.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
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
        .add_system(bevy::window::close_on_esc)
        .add_system(player_movement)
        .run();
}

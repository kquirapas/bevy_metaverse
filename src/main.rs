use bevy::prelude::*;

const GRAVITY: f32 = 100.0;
const SPRITE_WIDTH: f32 = 50.0;
const SPRITE_HEIGHT: f32 = 50.0;
const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPEED: f32 = 300.0;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
enum Direction {
    Stay,
    Up,
    Down,
    Left,
    Right
}

// #[derive(Component)]
// enum State {
//     Moving,
//     Stationary
// }

fn keyboard_input(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut position: Query<(&mut Direction, &mut Transform)>) {
    for (mut direction, mut position) in &mut position {
        // apply movement changes based on time delta
        match *direction {
            Direction::Up => position.translation.y += SPEED * time.delta_seconds(),
            Direction::Down => position.translation.y -= SPEED * time.delta_seconds(),
            Direction::Left => position.translation.x -= SPEED * time.delta_seconds(),
            Direction::Right => position.translation.x += SPEED * time.delta_seconds(),
            Direction::Stay => ()
        }

        // make Stay a default direction
        *direction = Direction::Stay;

        // make Transform adjustments
        if keyboard_input.pressed(KeyCode::W) {
            info!("Sprite moving up!");
            *direction = Direction::Up;
        } else if keyboard_input.pressed(KeyCode::A) {
            info!("Sprite moving left!");
            *direction = Direction::Left;
        } else if keyboard_input.pressed(KeyCode::S) {
            info!("Sprite moving down!");
            *direction = Direction::Down;
        } else if keyboard_input.pressed(KeyCode::D) {
            info!("Sprite moving right!");
            *direction = Direction::Right;
        }
    }
}

/*
 * @dev prevent the sprite from going out of bounds
 */
fn screen_bound(mut position: Query<(&mut Direction, &mut Transform)>) {
    let half_screen_width = SCREEN_WIDTH / 2.0;
    let half_screen_height = SCREEN_HEIGHT / 2.0;
    let half_sprite_width = SPRITE_WIDTH / 2.0;
    let half_sprite_height = SPRITE_HEIGHT / 2.0;

    for (_, mut position) in &mut position {
        if position.translation.x - half_sprite_width < -half_screen_width {
            info!("Left Screen Collision!");
            position.translation.x = -half_screen_width + half_sprite_width;
        } else if position.translation.x + half_sprite_width > half_screen_width {
            info!("Right Screen Collision!");
            position.translation.x = half_screen_width - half_sprite_width;
        }

        if position.translation.y - half_sprite_height < -half_screen_height {
            info!("Top Screen Collision!");
            position.translation.y = -half_screen_height + half_sprite_height;
        } else if position.translation.y + half_sprite_height > half_screen_height {
            info!("Bottom Screen Collision!");
            position.translation.y = half_screen_height - half_sprite_height;
        }
    }
}

/*
 * @dev apply gravity
 */
fn apply_gravity(time: Res<Time>, mut position: Query<(&mut Direction, &mut Transform)>) {
    for (_, mut position) in &mut position {
        position.translation.y -= GRAVITY * time.delta_seconds();
    }
}

fn draw(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Player Entity
    // Spawn entity with (SpriteBundle, Direction) tuple
    commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            Direction::Stay
        )
    );
}

fn main() {
    App::new()
        // manage window size
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(draw)
        .add_system(keyboard_input)
        .add_system(apply_gravity)
        .add_system(screen_bound)
        .run();
}

use bevy::prelude::*;

const JUMP_IMPULSE: f32 = 300.0;
const GRAVITY: f32 = 500.0;
const SPRITE_WIDTH: f32 = 50.0;
const SPRITE_HEIGHT: f32 = 50.0;
const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPEED: f32 = 300.0;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
enum State {
    Jumping,
    OnGround
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn keyboard_input(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut position: Query<(&mut State, &mut Velocity, &mut Transform)>) {
    for (mut state, mut velocity, mut position) in &mut position {
        // reset horizontal velocity
        velocity.x = 0.0;
        
        // make Transform adjustments
        if keyboard_input.pressed(KeyCode::W) {
            if let State::OnGround = *state {
                info!("Sprite jumping up!");
                velocity.y = JUMP_IMPULSE;

                *state = State::Jumping;
            }
        }

        if keyboard_input.pressed(KeyCode::A) {
            info!("Sprite moving left!");
            velocity.x = -SPEED;
        } else if keyboard_input.pressed(KeyCode::D) {
            info!("Sprite moving right!");
            velocity.x = SPEED;
        }

        // apply movement changes based on time delta
        position.translation.x += velocity.x * time.delta_seconds();
        position.translation.y += velocity.y * time.delta_seconds();
    }
}

/*
 * @dev prevent the sprite from going out of bounds
 */
fn screen_bound(mut position: Query<(&mut State, &mut Transform)>) {
    let half_screen_width = SCREEN_WIDTH / 2.0;
    let half_screen_height = SCREEN_HEIGHT / 2.0;
    let half_sprite_width = SPRITE_WIDTH / 2.0;
    let half_sprite_height = SPRITE_HEIGHT / 2.0;

    for (mut state, mut position) in &mut position {
        if position.translation.x - half_sprite_width < -half_screen_width {
            info!("Left Screen Collision!");
            position.translation.x = -half_screen_width + half_sprite_width;
        } else if position.translation.x + half_sprite_width > half_screen_width {
            info!("Right Screen Collision!");
            position.translation.x = half_screen_width - half_sprite_width;
        }

        if position.translation.y - half_sprite_height < -half_screen_height {
            info!("Bottom Screen Collision!");
            position.translation.y = -half_screen_height + half_sprite_height;

            // change state to no longer jumping
            *state = State::OnGround;
        } else if position.translation.y + half_sprite_height > half_screen_height {
            info!("Top Screen Collision!");
            position.translation.y = half_screen_height - half_sprite_height;
        }
    }
}

/*
 * @dev apply gravity
 * 
 * remember that gravity is a form of acceleration (m^2)
 */
fn apply_gravity(time: Res<Time>, mut position: Query<(&mut State, &mut Velocity)>) {
    for (state, mut velocity) in &mut position {
        // gravity only applies when player is in Jumping State
        match *state {
            State::Jumping => velocity.y -= GRAVITY * time.delta_seconds(),
            State::OnGround => velocity.y = 0.0
        }
    }
}

fn draw(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Player Entity
    // Spawn entity with (SpriteBundle, State, Velocity) tuple
    commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            State::Jumping,
            Velocity {
                x: 0.0,
                y: 0.0,
            }
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

use bevy::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;
const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;
const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Tetris".to_string(),
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        ..default()
    });
}

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Tetris".to_string(),
                width: 500.,
                height: 500.,
                ..default()
            },
            ..default()
        }))
        .run();
}

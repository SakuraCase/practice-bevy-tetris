use bevy::prelude::*;
use rand::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;
const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;
const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Resource)]
struct Materials {
    colors: Vec<Color>,
}

#[derive(Resource)]
struct BlockPatterns(Vec<Vec<(i32, i32)>>);

#[derive(Default)]
struct NewBlockEvent;

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
        .insert_resource(BlockPatterns(vec![
            vec![(0, 0), (0, -1), (0, 1), (0, 2)],  // I
            vec![(0, 0), (0, -1), (0, 1), (-1, 1)], // L
            vec![(0, 0), (0, -1), (0, 1), (1, 1)],  // 逆L
            vec![(0, 0), (0, -1), (1, 0), (1, 1)],  // Z
            vec![(0, 0), (1, 0), (0, 1), (1, -1)],  // 逆Z
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],   // 四角
            vec![(0, 0), (-1, 0), (1, 0), (0, 1)],  // T
        ]))
        .add_startup_system(setup)
        .add_system(position_transform)
        .add_system(spawn_block)
        .add_event::<NewBlockEvent>()
        .run();
}

fn setup(mut commands: Commands, mut new_block_events: EventWriter<NewBlockEvent>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Materials {
        colors: vec![
            Color::rgb_u8(64, 230, 100),
            Color::rgb_u8(220, 64, 90),
            Color::rgb_u8(70, 150, 210),
            Color::rgb_u8(220, 230, 70),
            Color::rgb_u8(35, 220, 241),
            Color::rgb_u8(240, 140, 70),
        ],
    });
    new_block_events.send(NewBlockEvent);
}

fn block_element(color: Color, position: Position) -> (SpriteBundle, Position) {
    (
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            ..default()
        },
        position,
    )
}

fn next_color(colors: &Vec<Color>) -> Color {
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= colors.len();

    colors[color_index]
}

fn next_block(block_patterns: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut block_index: usize = rng.gen();
    block_index %= block_patterns.len();

    block_patterns[block_index].clone()
}

fn spawn_block(
    mut commands: Commands,
    materials: Res<Materials>,
    block_patterns: Res<BlockPatterns>,
    mut new_block_events_reader: EventReader<NewBlockEvent>,
) {
    if new_block_events_reader.iter().next().is_none() {
        return;
    }

    let new_block = next_block(&block_patterns.0);
    let new_color = next_color(&materials.colors);

    // ブロックの初期位置
    let initial_x = X_LENGTH / 2;
    let initial_y = Y_LENGTH - 4;

    new_block.iter().for_each(|(r_x, r_y)| {
        commands.spawn(block_element(
            new_color,
            Position {
                x: (initial_x as i32 + r_x),
                y: (initial_y as i32 + r_y),
            },
        ));
    });
}

fn position_transform(mut position_query: Query<(&Position, &mut Transform, &mut Sprite)>) {
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;

    position_query
        .iter_mut()
        .for_each(|(pos, mut transform, mut sprite)| {
            transform.translation = Vec3::new(
                (origin_x + pos.x * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
            sprite.custom_size = Some(Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32))
        });
}

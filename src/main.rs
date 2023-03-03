use std::{collections::HashSet, mem::swap};

use bevy::{prelude::*, time::FixedTimestep};
use rand::{thread_rng, Rng};

const SIZE: i32 = 10;

const SNAKE_HEAD_COLOR: Color = Color::WHITE;
const SNAKE_BODY_COLOR: Color = Color::GRAY;
const FOOD_COLOR: Color = Color::BISQUE;

const FOOD_SPAWNER_TIMESTEP: f64 = 2.5;
const SNAKE_MOVEMENT_TIMESTEP: f64 = 0.2;

const BACKGROUND_COLOR: Color = Color::rgb(0.18, 0.18, 0.18);
const DEAD_ZONE_COLOR: Color = Color::rgb(0.6, 0.47, 0.47);

const START_SNAKE_POSITION: I32Position = I32Position { x: 4, y: 3 };

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Resource)]
struct SnakeDirection(Direction);

#[derive(Resource, Default)]
struct SnakeParts(Vec<Entity>);

#[derive(Resource, Default)]
struct LastSnakeTailPosition(I32Position);

#[derive(Resource, Default)]
struct OccupiedTiles(HashSet<I32Position>);

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct I32Position {
    x: i32,
    y: i32,
}

#[derive(Component, Clone, Copy)]
struct F32Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct SnakeBody;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct DespawnOnRestart;

#[derive(Component)]
struct Size(f32);

struct FoodEatenEvent;

struct GameOverEvent;

impl I32Position {
    fn move_in_direction(&self, direction: Direction) -> Self {
        Self {
            x: match direction {
                Direction::Left => self.x - 1,
                Direction::Right => self.x + 1,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y + 1,
                Direction::Down => self.y - 1,
                _ => self.y,
            },
        }
    }
}

impl F32Position {
    fn screen_coordinate(coordinate: f32, screen_size: f32) -> f32 {
        let tile_size = screen_size / SIZE as f32;

        tile_size * coordinate as f32 - screen_size / 2.0 + tile_size / 2.0
    }

    fn to_screen_coordinates(&self, width: f32, height: f32) -> (f32, f32) {
        (
            Self::screen_coordinate(self.x, width),
            Self::screen_coordinate(self.y, height),
        )
    }
}

impl From<I32Position> for F32Position {
    fn from(value: I32Position) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

fn spawn_field(mut commands: Commands) {
    let position = (SIZE - 1) as f32 / 2.0;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BACKGROUND_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    z: -0.01,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(F32Position {
            x: position,
            y: position,
        })
        .insert(Size(SIZE as f32));
}

fn spawn_snake(
    mut occupied_tiles: ResMut<OccupiedTiles>,
    mut last_snake_tail_position: ResMut<LastSnakeTailPosition>,
    mut snake_parts: ResMut<SnakeParts>,
    mut commands: Commands,
) {
    let head_id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(START_SNAKE_POSITION)
        .insert(Size(0.9))
        .insert(SnakeBody)
        .insert(DespawnOnRestart)
        .id();

    snake_parts.0.push(head_id);

    occupied_tiles.0.insert(START_SNAKE_POSITION);

    last_snake_tail_position.0 = I32Position {
        y: START_SNAKE_POSITION.y - 1,
        ..START_SNAKE_POSITION
    };
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn transform_translation(
    windows: Res<Windows>,
    mut q: Query<(
        Option<&I32Position>,
        Option<&F32Position>,
        &Size,
        &mut Transform,
    )>,
) {
    let window = match windows.get_primary() {
        Some(window) => window,
        None => return,
    };
    let min_window_dimension = window.width().min(window.height());

    let cell_size = min_window_dimension / SIZE as f32;

    for (i32_position, f32_position, size, mut transform) in q.iter_mut() {
        let position = if let Some(i32_position) = i32_position {
            (*i32_position).into()
        } else {
            *f32_position.unwrap()
        };

        let screen_position =
            position.to_screen_coordinates(min_window_dimension, min_window_dimension);

        transform.translation.x = screen_position.0;
        transform.translation.y = screen_position.1;

        transform.scale.x = cell_size * size.0;
        transform.scale.y = transform.scale.x;
    }
}

fn snake_movement(
    snake_parts: Res<SnakeParts>,
    direction: Res<SnakeDirection>,
    mut last_snake_tail_position: ResMut<LastSnakeTailPosition>,
    mut snake_body: Query<&mut I32Position, With<SnakeBody>>,
    mut occupied_tiles: ResMut<OccupiedTiles>,
) {
    let mut snake_parts = snake_parts.0.iter();

    let snake_head_id = snake_parts.next().unwrap();

    let mut new_position = *snake_body
        .get_component::<I32Position>(*snake_head_id)
        .unwrap();

    occupied_tiles.0.insert(new_position);

    *snake_body.get_mut(*snake_head_id).unwrap() = new_position.move_in_direction(direction.0);

    for snake_part_id in snake_parts {
        let mut position = snake_body.get_mut(*snake_part_id).unwrap();
        swap(&mut new_position, &mut position);
    }

    occupied_tiles.0.remove(&new_position);

    last_snake_tail_position.0 = new_position;
}

fn snake_direction_input(keys: Res<Input<KeyCode>>, mut direction: ResMut<SnakeDirection>) {
    if keys.pressed(KeyCode::W) {
        direction.0 = Direction::Up;
    } else if keys.pressed(KeyCode::S) {
        direction.0 = Direction::Down;
    } else if keys.pressed(KeyCode::D) {
        direction.0 = Direction::Right;
    } else if keys.pressed(KeyCode::A) {
        direction.0 = Direction::Left;
    }
}

fn spawn_snake_body(
    occupied_tiles: &mut ResMut<OccupiedTiles>,
    snake_parts: &mut ResMut<SnakeParts>,
    commands: &mut Commands,
    position: I32Position,
) {
    let id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_BODY_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(Size(0.8))
        .insert(SnakeBody)
        .insert(DespawnOnRestart)
        .id();

    snake_parts.0.push(id);

    occupied_tiles.0.insert(position);
}

fn food_spawner(mut commands: Commands, mut occupied_tiles: ResMut<OccupiedTiles>) {
    let mut rng = thread_rng();

    let x: i32 = rng.gen_range(0..SIZE);
    let y: i32 = rng.gen_range(0..SIZE);

    let position = I32Position { x, y };

    if occupied_tiles.0.contains(&position) {
        return;
    }

    occupied_tiles.0.insert(position);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(Size(0.9))
        .insert(Food)
        .insert(DespawnOnRestart);
}

fn food_eater(
    mut commands: Commands,
    mut food_eaten_writer: EventWriter<FoodEatenEvent>,
    mut occupied_tiles: ResMut<OccupiedTiles>,
    snake_parts: Res<SnakeParts>,
    food_query: Query<(Entity, &I32Position), With<Food>>,
    snake_body: Query<&I32Position, With<SnakeBody>>,
) {
    let snake_head_position = snake_body.get(snake_parts.0[0]).unwrap();

    for (entity_id, food_position) in food_query.iter() {
        if food_position == snake_head_position {
            occupied_tiles.0.remove(&food_position);
            commands.entity(entity_id).despawn();

            food_eaten_writer.send(FoodEatenEvent);

            break;
        }
    }
}

fn snake_grower(
    mut commands: Commands,
    mut food_eaten_reader: EventReader<FoodEatenEvent>,
    mut last_snake_tail_position: ResMut<LastSnakeTailPosition>,
    mut snake_parts: ResMut<SnakeParts>,
    mut occupied_tiles: ResMut<OccupiedTiles>,
) {
    for _ in food_eaten_reader.iter() {
        spawn_snake_body(
            &mut occupied_tiles,
            &mut snake_parts,
            &mut commands,
            last_snake_tail_position.0,
        );

        last_snake_tail_position.0 = I32Position {
            x: last_snake_tail_position.0.x - 1,
            y: last_snake_tail_position.0.y,
        };
    }
}

fn collision_detection(
    snake_parts: Res<SnakeParts>,
    snake_body: Query<&I32Position, With<SnakeBody>>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
) {
    let snake_head_position = snake_body.get(snake_parts.0[0]).unwrap();

    if snake_head_position.x < 0
        || snake_head_position.y < 0
        || snake_head_position.x >= SIZE
        || snake_head_position.y >= SIZE
    {
        game_over_event_writer.send(GameOverEvent);

        return;
    }

    let mut collisions_count = 0;

    for snake_part_position in snake_body.iter() {
        if snake_head_position == snake_part_position {
            collisions_count += 1;

            if collisions_count == 2 {
                game_over_event_writer.send(GameOverEvent);

                return;
            }
        }
    }
}

fn game_over_event_handler(game_over_event_reader: EventReader<GameOverEvent>) {
    if !game_over_event_reader.is_empty() {
        info!("game over");

        game_over_event_reader.clear();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SnakeParts>()
        .init_resource::<LastSnakeTailPosition>()
        .init_resource::<OccupiedTiles>()
        .insert_resource(SnakeDirection(Direction::Up))
        .insert_resource(ClearColor(DEAD_ZONE_COLOR))
        .add_event::<FoodEatenEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_field)
        .add_startup_system(spawn_snake)
        .add_system(snake_direction_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SNAKE_MOVEMENT_TIMESTEP))
                .with_system(snake_movement)
                .with_system(food_eater.after(snake_movement))
                .with_system(collision_detection.after(snake_movement))
                .with_system(game_over_event_handler.after(collision_detection))
                .with_system(snake_grower.after(food_eater)),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FOOD_SPAWNER_TIMESTEP))
                .with_system(food_spawner),
        )
        .add_system_to_stage(CoreStage::PostUpdate, transform_translation)
        .run();
}

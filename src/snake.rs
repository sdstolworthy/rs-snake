use std::time::Duration;

use crate::{
    dimensions::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH},
    feed::Food,
};
use bevy::{prelude::*, time::common_conditions::on_timer};

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
pub fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
            || segment_positions.contains(&head_pos)
        {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

#[derive(Component)]
struct SnakeSegment;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameOverEvent>()
            .add_event::<GrowthEvent>()
            .add_systems(
                Update,
                (
                    snake_movement_input,
                    (
                        snake_movement.run_if(on_timer(Duration::from_secs(1))),
                        game_over,
                        snake_eating,
                        snake_growth,
                    )
                        .chain(),
                ),
            )
            .add_systems(Startup, spawn_snake);
    }
}

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub struct SnakePlugin;

#[derive(Event)]
pub struct GameOverEvent;
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 3 }),
    ]);
}

#[derive(Event)]
pub struct GrowthEvent;

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

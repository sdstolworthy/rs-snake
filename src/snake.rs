use crate::{dimensions::{Position, Size}, movement};
use bevy::prelude::*;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct SnakeHead {
    pub direction: movement::Direction,
}

pub fn spawn_snake(mut commands: Commands) {
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
            direction: movement::Direction::Up,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

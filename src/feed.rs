use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::prelude::random;

use crate::dimensions::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
pub struct FeederPlugin;
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

impl Plugin for FeederPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            food_spawner.run_if(on_timer(Duration::from_secs(2))),
        );
    }
}
fn food_spawner(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

#[derive(Component)]
pub struct Food;

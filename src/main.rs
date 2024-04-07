mod dimensions;
mod feed;
mod movement;
mod snake;
use crate::dimensions::{position_translation, size_scaling};
use bevy::{prelude::*, window::WindowResolution};
use movement::SnakeMovementPlugin;
use snake::spawn_snake;

fn main() {
    App::new()
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(PostUpdate, (position_translation, size_scaling).chain())
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(feed::FeederPlugin)
        .add_plugins(SnakeMovementPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snek".into(),
                name: Some("Snek".into()),
                resolution: WindowResolution::new(500., 500.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

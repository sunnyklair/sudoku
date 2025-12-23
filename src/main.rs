use bevy::prelude::*;

fn main() {
    App::new() 
    .add_systems(Startup, setup)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text2d::new("Hello World"),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
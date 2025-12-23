mod plugins;
use plugins::MenuPlugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    SplashIntro,
    MainMenu,
    //Game,
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((DefaultPlugins, MenuPlugin))
        .init_state::<GameState>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

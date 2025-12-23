mod plugins;

use bevy::prelude::*;
use plugins::MenuPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MenuPlugin,
        ))
        .run();
}
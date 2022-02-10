use bevy::prelude::*;
use rustiant::RustiantPlugin;

mod rustiant;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RustiantPlugin)
        .run();
}

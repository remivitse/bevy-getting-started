pub mod component;
pub mod entity;
pub mod plugin;
pub mod system;

use crate::plugin::hello::HelloPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

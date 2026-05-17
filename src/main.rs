pub mod component;
pub mod entity;
pub mod system;

use crate::system::add_people::add_people;
use crate::system::greet_people::greet_people;
use crate::system::hello_world::hello_world;
use crate::system::update_people::update_people;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

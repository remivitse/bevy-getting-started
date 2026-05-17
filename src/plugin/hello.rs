use crate::system::add_people::add_people;
use crate::system::greet_people::greet_people;
use crate::system::update_people::update_people;
use bevy::prelude::IntoScheduleConfigs;
use bevy::prelude::{App, Plugin, Startup, Update};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

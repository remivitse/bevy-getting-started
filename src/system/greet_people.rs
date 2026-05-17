use crate::component::name::Name;
use crate::component::person::Person;
use bevy::prelude::{Query, With};

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

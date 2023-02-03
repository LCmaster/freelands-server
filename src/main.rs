mod lib;
use std::{
    cmp::{max, min},
    thread, time,
};

use lib::world::{Component, World};
use rand::prelude::*;

// use serde::Serialize;
// use warp::Filter;

struct Health {
    points: u8,
}
impl Component for Health {}

fn main() {
    let mut world = World::new();
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        world.spawn(vec![Health {
            points: max(0, min(100, rng.gen())),
        }]);
    }

    world.add_system(|entity, components| {
        if let Some(health) = components.get::<Health>() {
            println!("Entity {} has {} life points", entity, health.points);
        }
    });

    for _ in 0..3 {
        println!("UPDATING!");
        world.update();
        thread::sleep(time::Duration::from_millis(1000));
    }
}

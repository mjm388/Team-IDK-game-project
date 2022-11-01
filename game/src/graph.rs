use::std::collections::LinkedList;
use bevy::prelude::*;

use crate::{
    room_generator::NUM_OF_ROOMS,
    room_generator::Room,
    GameState,
};

pub struct Graph{
    pub vert: Vec<Vec<i32>>,
}

impl Graph {

    fn new() -> Graph {
        let mut v = Vec::new();

        for i in 0..NUM_OF_ROOMS{
            v.push(Vec::new());
        }
        Graph{
            vert: v,
        }


    }


}

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_startup_system()
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_exit(GameState::Overworld)
        );
    }
}

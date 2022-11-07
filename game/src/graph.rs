use bevy::prelude::*;

use crate::{
    room_generator::NUM_OF_ROOMS,
    room_generator::Room,
    GameState,
};

#[derive(Component)]
struct Coordinate{
    coord: Vec2,
}

#[derive(Component)]
struct Length(Vec2);

#[derive(Component,Clone)]
pub struct WeightedEdge{
    room: Room,
    length: Vec2,
}

pub struct Graph{
    pub vert: [Vec<WeightedEdge>; NUM_OF_ROOMS as usize],
}


pub struct Edge {
    a: Vec2,
    b: Vec2,
    length: Vec2,
}

enum List<WeightedEdge> {
    Node(WeightedEdge, Box<List<WeightedEdge>>),
    Nil,
}



impl Graph {

    /*fn create(
        &mut self,
        polygon: Vec<Edge>,
        room: Query<(&mut Room, &Coordinate), With<Room>>,
        room2: Query<(&mut Room, &Coordinate), With<Room>>,
    ) -> &mut Graph {

        for (r,c) in room.iter(){
            let mut cur_vec: &mut Vec<WeightedEdge> = &mut self.vert.get(r.id).unwrap().to_vec();
            for e in polygon.iter(){
                if e.a.x == c.coord.x && e.a.y == c.coord.y{
                    for (r2,c2) in room2.iter(){
                        if e.b.x == c2.coord.x && e.b.y == c2.coord.y{
                            cur_vec.push(WeightedEdge{
                                room: (*r2),
                                length: e.length,
                            });
                        }
                    }
                }
                else if e.b.x == c.coord.x && e.b.y == c.coord.y{
                    for (r2,c2) in room2.iter(){
                        if e.a.x == c2.coord.x && e.a.y == c2.coord.y{
                            cur_vec.push(WeightedEdge{
                                room: (*r2),
                                length: e.length,
                            });
                        }
                    }
                }
            }
        }


        self

    }*/

    /*fn new() -> Graph {
        let mut v = Vec::new();

        for i in 0..NUM_OF_ROOMS{
            v.push(Vec::new());
        }

        Graph{
            vert: v,
        }
    }*/

    fn new2(
        polygon: Vec<Edge>,
        room: Query<(&mut Room, &Coordinate), With<Room>>,
        room2: Query<(&mut Room, &Coordinate), With<Room>>,
    ) -> Graph {

        let mut g = Graph{
            vert: [List<WeightedEdge>; NUM_OF_ROOMS as usize],
        };

        for i in 0..NUM_OF_ROOMS{
            g.vert.push(LL{list: Vec::<WeightedEdge>::new()});
        }

        g.

        g

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

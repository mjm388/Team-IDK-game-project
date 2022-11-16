use bevy::{
	prelude::*,
};
use super::Edge;
use crate::{
	map_gen::room_gen::{X_BOUND, Y_BOUND, SIZE_UPPER_BOUND},
};

const NORTH:usize = 0;
// const EAST: usize = 1;
// const SOUTH:usize = 2;
// const WEST: usize = 3;

const FREE: i32 = 0;
const ROOM: i32 = 1;
// const PATH: i32 = 2;
// const WALL: i32 = 3;

#[derive(Clone)]
struct Ind(usize, usize);

pub fn hallway (centers: &Vec<Vec2>, sizes: &Vec<Vec2>, edges: &Vec<Edge>) {
	let mut graph = graph_2d(centers, sizes);

	let num_of_rooms = centers.len();

	// edges that stores indices from "centers"
	let edges = edge_to_ind(edges, num_of_rooms, centers);

	// four holes of rooms
	let mut holes= vec![vec![true; 4]; num_of_rooms];

	// exact door pairs from start to destination
	let mut door_pairs = vec![];
	for (c_start, c_dest) in edges {
		door_pairs.push(find_doors(c_start, c_dest, &centers, &sizes, &mut holes));
	}

	astar(&mut graph, &door_pairs);

	//print_graph(&graph);
}

fn astar(graph: &mut Vec<Vec<i32>>, door_pairs: &Vec<(Ind, Ind)>) {
	// stud
	for (start, dest) in door_pairs {
		graph[start.0][start.1] = 2;
		graph[dest.0][dest.1] = 2;
	}
}

fn find_doors(
	c_start: usize, 
	c_dest: usize, 
	centers: &Vec<Vec2>, 
	sizes: &Vec<Vec2>, 
	holes: &mut Vec<Vec<bool>>) -> (Ind, Ind) {
	// data chop
	let x_c_s = centers[c_start].x;
	let y_c_s = centers[c_start].y;
	let x_c_d = centers[c_dest ].x;
	let y_c_d = centers[c_dest ].y;
	let x_size_s = (sizes[c_start].x/2.).floor();
	let y_size_s = (sizes[c_start].y/2.).floor();
	let x_size_d = (sizes[c_dest ].x/2.).floor();
	let y_size_d = (sizes[c_dest ].y/2.).floor();

	// four doors
	let s_doors = vec![ 
		Vec2::new(x_c_s + x_size_s, y_c_s),				// north
		Vec2::new(x_c_s, 			y_c_s + y_size_s),	// east
		Vec2::new(x_c_s - x_size_s, y_c_s),				// south
		Vec2::new(x_c_s, 			y_c_s - y_size_s)	// west
	];
	let d_doors = vec![ 
		Vec2::new(x_c_d + x_size_d, y_c_d),				// north
		Vec2::new(x_c_d, 			y_c_d + y_size_d),	// east
		Vec2::new(x_c_d - x_size_d, y_c_d),				// south
		Vec2::new(x_c_d, 			y_c_d - y_size_d)	// west
	];
	
	// find matching doors between two rooms
	let mut s_dir = NORTH;
	let mut d_dir = NORTH;
	let mut min_dist = f32::INFINITY;
	for s in 0..4 {
		for d in 0..4 {
			let dist = s_doors[s].distance(d_doors[d]);
			if dist < min_dist {
				s_dir = s;
				d_dir = d;
				min_dist = dist;
			}
		}
 	}
	// mark doors
	holes[c_start][s_dir] = false;
	holes[c_dest][d_dir] = false;

	return (coord_to_ind(s_doors[s_dir]), coord_to_ind(d_doors[d_dir]));
}

fn edge_to_ind(edges: &Vec<Edge>, room_count: usize, centers: &Vec<Vec2>) -> Vec<(usize, usize)>{
	let mut edge_inds: Vec<(usize, usize)> = vec![];
	for edge in edges {
		let mut s: usize = 0;
		let mut d: usize = 0;
		for i in 0..room_count {
			if edge.0 == centers[i] {
				s = i;
			}
			if edge.1 == centers[i] {
				d = i;
			}
		}
		edge_inds.push((s,d));
	}
	return edge_inds;
}

fn graph_2d(centers: &Vec<Vec2>, sizes: &Vec<Vec2>) -> Vec<Vec<i32>> {
	let x_ubound = (SIZE_UPPER_BOUND + X_BOUND) as usize;
	let y_ubound = (SIZE_UPPER_BOUND + Y_BOUND) as usize;
	let mut graph = vec![vec![FREE; y_ubound * 2]; x_ubound * 2];
	
	for (c, s) in centers.iter().zip(sizes.iter()) {
		for x in -(s.x as i32)/2..=(s.x as i32)/2 {
			for y in -(s.y as i32)/2..=(s.y as i32)/2 {
				let gi = coord_to_ind(Vec2::new(c.x+x as f32, c.y+y as f32));
				graph[gi.0][gi.1] = ROOM;
			}
		}
	}
	return graph;
}

fn _ind_to_coord(x: usize, y: usize) -> Vec2 {
	let cx = x as f32 - X_BOUND - SIZE_UPPER_BOUND;
	let cy = y as f32 - Y_BOUND - SIZE_UPPER_BOUND;
	Vec2::new(cx,cy)
}

fn coord_to_ind(coord: Vec2) -> Ind {
	let x = (coord.x + X_BOUND + SIZE_UPPER_BOUND) as usize;
	let y = (coord.y + Y_BOUND + SIZE_UPPER_BOUND) as usize;
	return Ind(x,y)
}

fn _print_graph (graph: &Vec<Vec<i32>>) {
	for y in 0..graph.len(){
		for x in 0..graph[0].len() {
			print!("{}", graph[x][graph.len()-y-1]);
		}
		println!("");
	}
}
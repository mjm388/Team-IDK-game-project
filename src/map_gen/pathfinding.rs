// https://www.redblobgames.com/pathfinding/a-star/introduction.html

use bevy::{
    prelude::*,
};
use super::Edge;
use crate::{
    map_gen::{
        room_gen::{X_BOUND, Y_BOUND, SIZE_UPPER_BOUND},
        BlockPath, 
        StandPath,
    },
};

const NORTH:usize = 0;
// const EAST: usize = 1;
// const SOUTH:usize = 2;
// const WEST: usize = 3;
const FREE: i32 = -1;
const ROOM: i32 = -2;
const PATH: i32 = -3;
const OBST: i32 = -4; // obstacle
const START:i32 = -5;
const DEST: i32 = -6;

#[derive(Clone)]
struct Ind(usize, usize);

pub struct QNode {
    dist_s: f32,
	length: f32,
    ind: Ind,
    prev: Ind,
}
impl QNode {
    fn new(ind: Ind, dest: &Ind, dist_s: f32, prev: Ind) -> QNode {
        QNode {
			dist_s,
			length: dist_s + distance(&ind, &dest),
            ind,
            prev: prev,
        }
    }
}

pub struct PQ{
    heap: Vec<QNode>,
}
impl PQ{
    fn new() -> PQ{
        PQ{
            heap: Vec::new(),
        }
    }
    fn len(&self) -> usize{
        self.heap.len()
    }
    fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }
    pub fn add(&mut self, e: QNode){
		//info!("add {}", e.length); //???
        self.heap.push(e);
        self.bubble(self.len()-1);
        // print!("add    ");//???
        // self._print_vec();//???
    }
    pub fn bubble(&mut self, index: usize){
		if index > 0 {
			let parent = (index-1)/2;
			if self.heap[parent].length > self.heap[index].length{
				self.heap.swap(parent,index);
				self.bubble(parent);
			}
		}
    }
    pub fn remove(&mut self) -> Option<QNode> {
        if self.is_empty() {
            None
        }
        else{
            let last = self.len() - 1;
            self.heap.swap(0,last);
            let e = self.heap.remove(last);
            self.sink(0);
            Some(e)
        }
    }
    pub fn sink(&mut self, index: usize) {
        let heap_length = self.len();
        let left = (index*2) + 1;
        let right = (index*2) + 2;

        if (left < heap_length) && (right < heap_length){
            if self.heap[left].length < self.heap[index].length && self.heap[left].length < self.heap[right].length{
                self.heap.swap(left,index);
                self.sink(left);
            }
            else if self.heap[right].length < self.heap[index].length && self.heap[right].length < self.heap[left].length{
                self.heap.swap(right,index);
                self.sink(right);
            }
            else{
                //equals just do left
                self.heap.swap(left,index);
                self.sink(left);
            }
        }
        else if left < heap_length {
            if self.heap[left].length < self.heap[index].length{
                self.heap.swap(left,index);
                self.sink(left);
            }
        }
    }
    fn _print_vec(&self) {
        for e in self.heap.iter(){
            print!("{}::",e.length);
        }
        println!();
    }
}

pub fn hallway (
    centers: &Vec<Vec2>, 
    sizes: &Vec<Vec2>, 
    edges: &Vec<Edge>, 
    commands: &mut Commands) {
    // index in the form of graph[x][y] in cartesian coordinates
    let mut graph = graph_2d(centers, sizes);

    let num_of_rooms = centers.len();

    // edges that stores indices from "centers"
    let edges = edge_to_ind(edges, num_of_rooms, centers);
    // for room in centers {
    //     let r = coord_to_ind(room.clone());
    //     info!("room coords {} {}", r.0, r.1);
    // }

    // four holes of rooms
    let mut holes= vec![vec![true; 4]; num_of_rooms];

    // exact door pairs from start to destination
    let mut door_pairs = vec![];
    for (c_start, c_dest) in edges {
        door_pairs.push(find_doors(c_start, c_dest, &centers, &sizes, &mut holes));
    }
    for i in 0..num_of_rooms {
        settle_room_holes(centers[i], sizes[i], &holes[i], commands);
    }

    astar(&mut graph, &door_pairs);

    _print_graph(&graph);
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
        Vec2::new(x_c_s+x_size_s+1.,y_c_s),             // north
        Vec2::new(x_c_s,            y_c_s+y_size_s+1.), // east
        Vec2::new(x_c_s-x_size_s-1.,y_c_s),             // south
        Vec2::new(x_c_s,            y_c_s-y_size_s-1.), // west
    ];
    let d_doors = vec![ 
        Vec2::new(x_c_d+x_size_d+1.,y_c_d),             // north
        Vec2::new(x_c_d,            y_c_d+y_size_d+1.), // east
        Vec2::new(x_c_d-x_size_d-1.,y_c_d),             // south
        Vec2::new(x_c_d,            y_c_d-y_size_d-1.), // west
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
    holes[c_dest ][d_dir] = false;

    return (coord_to_ind(s_doors[s_dir]), coord_to_ind(d_doors[d_dir]));
}
fn settle_room_holes(center: Vec2, size: Vec2, holes: &Vec<bool>, commands: &mut Commands) {
    let x_size = (size.x/2.).floor();
    let y_size = (size.y/2.).floor();
    let hole_coord = vec![ 
        Vec2::new(center.x + x_size,center.y),          // north
        Vec2::new(center.x,         center.y + y_size), // east
        Vec2::new(center.x - x_size,center.y),          // south
        Vec2::new(center.x,         center.y - y_size), // west
    ];
    for i in 0..4 {
        if holes[i] {
            commands.spawn().insert(BlockPath(hole_coord[i]));
        }
        else {
            commands.spawn().insert(StandPath(hole_coord[i]));
        }
    }
}

fn astar(graph: &mut Vec<Vec<i32>>, door_pairs: &Vec<(Ind, Ind)>) {
    for (start, dest) in door_pairs.iter() { 
        // let (start, dest) = door_pairs[0].clone();
        // info!("start {} {}", start.0, start.1);
        // info!("dest  {} {}",  dest.0,  dest.1);
        // close set: Some(Some(prev))
	    //  open set: Some(None)
        // untraveled:None
        let mut stepped: Vec<Vec<Option<Option<Ind>>>> = vec![vec![None; graph[0].len()]; graph.len()];
        astar_search(graph, &mut stepped, &start, &dest);
        // back trace
        //let mut curr = &dest;
        let mut curr = dest;
        while curr.0 != start.0 || curr.1 != start.1 {
            graph[curr.0][curr.1] = PATH;
            match &stepped[curr.0][curr.1] {
                Some(x) => match x {
                    Some(xx) => curr = xx,
                    None => break,
                },
                None => break,
            }
        }
    }
}
fn astar_search(graph: &mut Vec<Vec<i32>>, stepped: &mut Vec<Vec<Option<Option<Ind>>>>, start: &Ind, dest: &Ind) {
    let mut queue = PQ::new();
    queue.add(QNode::new(start.clone(), dest, 0., start.clone()));
    graph[start.0][start.1] = 0;
    //graph[dest.0][dest.1] = DEST;
    while !queue.is_empty() {
        let qnode = queue.remove().unwrap();
        // print!("remove ");//???
        // queue._print_vec();//???
		let curr = qnode.ind;
		let dist_s = qnode.dist_s;

        // add node to close set
        stepped[curr.0][curr.1] = Some(Some(qnode.prev));
        if curr.0 == dest.0 && curr.1 == dest.1 {
            break;
        }
        let neighbors = neighbors(graph, &curr, dest);
        //info!("{}",neighbors.len());
        for n in neighbors.iter() {
            //info!("{} {}",stepped[n.0][n.1].is_none(), graph[n.0][n.1]);
            // add new node to open set
            if stepped[n.0][n.1].is_none() {
                //info!("step {} {}", n.0, n.1);
                stepped[n.0][n.1] = Some(None);
                //graph[n.0][n.1] = dist_s as i32+1;//???
                queue.add(QNode::new(n.clone(), dest, dist_s+1., curr.clone()));
            }
        }
    }
}
fn distance(curr: &Ind, dest: &Ind) -> f32 {
    ((dest.0 as f32 - curr.0 as f32).powf(2.) + (dest.1 as f32 - curr.1 as f32).powf(2.)).sqrt()
}
fn neighbors(graph: &Vec<Vec<i32>>, curr: &Ind, dest: &Ind) -> Vec<Ind> {
    let mut neighbor = Vec::new();
        
    if is_valid(graph, &curr, -1, 0, dest) {
        neighbor.push(Ind(curr.0-1, curr.1)); // west
        //info!("{} {} west", curr.0-1, curr.1);//???
    }
    if is_valid(graph, &curr, 1, 0, dest) {
        neighbor.push(Ind(curr.0+1, curr.1)); // east
        //info!("{} {} east", curr.0+1, curr.1);//???
    }
    if is_valid(graph, &curr,0, 1, dest) {
        neighbor.push(Ind(curr.0, curr.1+1)); // north
        //info!("{} {} north", curr.0, curr.1+1);//???
    }
    if is_valid(graph, &curr, 0, -1, dest) {
        neighbor.push(Ind(curr.0, curr.1-1)); // south
        //info!("{} {} south", curr.0, curr.1-1);//???
    }
    //info!("neighbor {}", neighbor.len());
    return neighbor;
}
fn is_valid(graph: &Vec<Vec<i32>>, curr: &Ind, xdiff: i32, ydiff: i32, dest: &Ind) -> bool {
    let x = curr.0 as i32 + xdiff;
    let y = curr.1 as i32 + ydiff;
    // if out of bound
    if x - 1 < 0 || x + 1 >= graph.len() as i32 || y - 1 < 0 || y + 1 >= graph[0].len() as i32 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
	// hit
	if x == dest.0 && y == dest.1 {
		return true;
	}
    // if 3x3 block has obstacle
    for xx in x-1..=x+1 {
        for yy in y-1..=y+1 {
            if graph[xx][yy] == ROOM || graph[xx][yy] == OBST {
                return false;
            }
        }
    }
    return true;
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
    // print in cartesian coordinate style
    for y in 0..graph[0].len() {
        for x in 0..graph.len() {
            let num = graph[x][graph[0].len()-y-1];
            if num == FREE {
                print!("  ");
            }
            else if num == PATH {
                print!("==");
            }
            else if num == ROOM {
                print!("[]");
            }
            else if num == START {
                print!("SS");
            }
            else if num == DEST {
                print!("DD");
            }
            else {
                let num = num % 100;
                if num >= 0 && num <= 9 {
                    print!("0");
                }
                print!("{}", num);
            }
        }
        println!("");
    }
}
// // for small map, easy to read
// fn _print_graph (graph: &Vec<Vec<i32>>) {
//     // print in cartesian coordinate style
//     for y in 0..graph[0].len() {
//         for x in 0..graph.len() {
//             let num = graph[x][graph[0].len()-y-1];
//             if num == FREE {
//                 print!("  ");
//             }
//             else if num == PATH {
//                 print!("==");
//             }
//             else if num == ROOM {
//                 print!("[]");
//             }
//             else if num == START {
//                 print!("SS");
//             }
//             else if num == DEST {
//                 print!("DD");
//             }
//             else {
//                 let num = num % 100;
//                 if num >= 0 && num <= 9 {
//                     print!("0");
//                 }
//                 print!("{}", num);
//             }
//             print!(" ");
//         }
//         println!("");
//     }
// }
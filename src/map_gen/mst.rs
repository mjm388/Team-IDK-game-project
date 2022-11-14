use bevy::{
	prelude::*,
};

use std::collections::HashMap;
use std::hash::Hash;

use super::Edge;


pub struct Graph{
	edges: HashMap<Key,Vec<GraphEdge>>,
}

#[derive(Clone)]
pub struct GraphEdge{
	origin: Vec2,
	destination: Vec2,
	length: f32,
}

impl GraphEdge{
	pub fn new(o: Vec2, d: Vec2) -> GraphEdge{
		GraphEdge{
			origin: o,
			destination: d,
			length: ((o.x - d.x).powf(2.) + (o.y - d.y).powf(2.)).sqrt(),
		}
	}

	// pub fn get_origin(&self) -> Vec2{
	// 	self.origin
	// }

	// pub fn get_destination(&self) -> Vec2{
	// 	self.destination
	// }
}

impl Graph{
	pub fn new(edges: &Vec<Edge>) -> Graph{
		let mut map = HashMap::<Key,Vec<GraphEdge>>::new();
		for e in edges.into_iter(){
			let k = Key::new(e.0);
			if map.contains_key(&k){
				let v: &mut Vec<GraphEdge> = map.entry(k).or_default();
				v.push(GraphEdge::new(e.0,e.1));
			}
			else{
				let v: Vec<GraphEdge> = Vec::from([GraphEdge::new(e.0,e.1)]);
				map.insert(k,v);
			}
			let k2 = Key::new(e.1);
			if map.contains_key(&k2){
				let v: &mut Vec<GraphEdge> = map.entry(k2).or_default();
				v.push(GraphEdge::new(e.1,e.0));
			}
			else{
				let v: Vec<GraphEdge> = Vec::from([GraphEdge::new(e.1,e.0)]);
				map.insert(k2,v);
			}
		}

		Graph{
			edges: map,
		}
	}

	pub fn _pretty_print(&self){
		for (key, value) in self.edges.iter(){
			println!("Vert ({},{}) connects to:",key.x,key.y);
			for w in value.iter(){
				println!("	({},{})",w.destination.x,w.destination.y);
				println!("	Len: {}",w.length);
			}
		}
	}


}

pub struct PQ{
	heap: Vec<GraphEdge>,
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



	pub fn add(&mut self, e: GraphEdge){
		self.heap.push(e);
		self.bubble(self.len()-1);
	}

	pub fn bubble(&mut self, index: usize){
		if index != 0 {
			let parent = (index-1)/2;
			if self.heap[parent].length > self.heap[index].length{
				self.heap.swap(parent,index);
				self.bubble(parent);
			}
		}
	}

	pub fn remove(&mut self) -> Option<GraphEdge> {
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
			println!("{}",e.length);
		}
	}
}



#[derive(PartialEq, Eq, Hash,Clone)]
pub struct Key{
	x: String,
	y: String,
}

impl Key{

	fn new(v: Vec2) -> Key {
		Key{
			x: v.x.to_string(),
			y: v.y.to_string(),
		}
	}
}

//pub fn prims(graph: &Graph) -> Vec<GraphEdge> {
pub fn prims(poly: &Vec<Edge>) -> Vec<Edge> {

	let graph = Graph::new(poly);

	let mut waiting_to_visit: PQ = PQ::new();

	let mut visited: Vec<Key> = Vec::new();

	let mut min_span_tree: Vec<GraphEdge> = Vec::new();


	for (k,_evec) in graph.edges.iter(){
		visited.push(k.clone());
		break;
	}

	let mut visiting: Key = visited[0].clone();

	while visited.len() < graph.edges.len(){
		for e in graph.edges.get_key_value(&visiting).unwrap().1 { // gets each edge in vec
			if not_visited(&visited,&e){
				//println!("add");
				waiting_to_visit.add(e.clone());
			}
		}

		//println!("{}",visited.len());


		let shortest = loop{
			let s = waiting_to_visit.remove().unwrap();
			if !visited.contains(&Key::new(s.destination)){
				break s;
			}
		};

		visiting = Key::new(shortest.destination);

		visited.push(visiting.clone());

		min_span_tree.push(shortest);

	}

	let mut count = 0;
	while count < 3 {
		match waiting_to_visit.remove(){
			Some(e) => {
				min_span_tree.push(e);
				count += 1;
			},
			None => break,
		}
	}

	//for e in minSpanTree.iter(){
	//	println!("Origin: {}, Destination: {}, Length:{}",e.origin,e.destination,e.length);
	//}


	//return  min_span_tree;
	return min_span_tree.iter().map(|x| Edge(x.origin, x.destination)).collect();
}

fn not_visited(visited: &Vec<Key>, edge: &&GraphEdge)-> bool{
	for visit in visited.iter(){
		//println!("({},{})",visit.x,visit.y);
		//println!("({},{})",edge.destination.x.to_string(),edge.destination.y.to_string());
		if visit.x == edge.destination.x.to_string() && visit.y == edge.destination.y.to_string(){
			return false;
		}
	}
	true
}

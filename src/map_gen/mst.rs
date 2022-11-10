use bevy::{
	prelude::*,
};

use std::collections::HashMap;
use std::hash::Hash;

use super::Edge;


pub struct Graph{
	edges: HashMap<Key,Vec<GraphEdge>>,
}

pub struct GraphEdge{
	origin: Vec2,
	destination: Vec2,
	length: f32,
}

impl Graph{
	fn new(edges: &Vec<Edge>) -> Graph{
		let mut map = HashMap::<Key,Vec<GraphEdge>>::new();
		for e in edges.into_iter(){
			let k = Key::new(e.0);
			if map.contains_key(&k){
				let mut v: &mut Vec<GraphEdge> = map.entry(k).or_default();
				v.push(GraphEdge{
					origin: e.0,
					destination: e.1,
					length: ((e.0.x - e.1.x).powf(2.) + (e.0.y - e.1.y).powf(2.)).sqrt(),
				});
			}
			else{
				let mut v: Vec<GraphEdge> = Vec::from([GraphEdge{
					origin: e.0,
					destination: e.1,
					length: ((e.0.x - e.1.x).powf(2.) + (e.0.y - e.1.y).powf(2.)).sqrt(),
				}]);
				map.insert(k,v);
			}
			let k2 = Key::new(e.1);
			if map.contains_key(&k2){
				let mut v: &mut Vec<GraphEdge> = map.entry(k2).or_default();
				v.push(GraphEdge{
					origin: e.1,
					destination: e.0,
					length: ((e.0.x - e.1.x).powf(2.) + (e.0.y - e.1.y).powf(2.)).sqrt(),
				})
			}
			else{
				let mut v: Vec<GraphEdge> = Vec::from([GraphEdge{
					origin: e.1,
					destination: e.0,
					length: ((e.0.x - e.1.x).powf(2.) + (e.0.y - e.1.y).powf(2.)).sqrt(),
				}]);
				map.insert(k2,v);
			}
		}

		Graph{
			edges: map,
		}
	}

	fn pretty_print(&self){
		for (key, value) in self.edges.iter(){
			println!("Vert ({},{}) connects to:",key.x,key.y);
			for w in value.iter(){
				println!("	({},{})",w.destination.x,w.destination.y);
				println!("	Len: {}",w.length);
			}
		}
	}

	// fn prims(&self) /*-> Vec<GraphEdge>*/{
	// 	let mut visited: Vec<Vec2> = Vec::new();

	// 	let mut waiting_to_visit: PQ = PQ::new();

	// 	let mut done = false;

	// 	//for (k,vvec) in self.edges.iter(){
	// 	//	let curx: f32 = visited.get(0).unwrap().x.parse::<f32>().unwrap();
	// 	//	let cury: f32 = visited.get(0).unwrap().y.parse::<f32>().unwrap();
	// 	//	let cur = Vec2::new(curx,cury);
	// 	//	visited.push(cur);
	// 	//	break;
	// 	//}

	// 	while done == false{
	// 	}
	// }

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

	pub fn remove(&mut self) -> bool {
		if self.is_empty() {
			false
		}
		else{
			let last = self.len() - 1;
			self.heap.swap(0,last);
			self.sink(0);
			true
		}
	}

	pub fn sink(&mut self, index: usize) {
		let heap_length = self.len() - 1;
		let left = (index*2) + 1;
		let right = (index*2) + 2;

		if (left < heap_length) && (right < heap_length){
			if self.heap[left].length < self.heap[index].length{
				self.heap.swap(left,index);
				self.sink(left);
			}
			else if self.heap[right].length < self.heap[index].length{
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
}

#[derive(PartialEq, Eq, Hash)]
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

pub fn prims(edges: &Vec<Edge>) -> &Vec<Edge> {
    let mut graph = Graph::new(edges);
	graph.pretty_print();
    return edges;
}
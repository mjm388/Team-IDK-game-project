use bevy::{
	prelude::*,
};

use rand::Rng;

#[derive(Clone, Eq, PartialEq, Copy)]
pub enum DecorType{
	Statue,
	Plant,
	Sofa,
	_Chair,
	_Lamp,
	Pillar,
}

#[derive(Component)]
pub struct Decor{
	pub location: Vec2,
	pub decor_type: DecorType,
}

impl Decor{
	fn new(l: Vec2, t: DecorType) -> Decor{
		Decor{
			location: Vec2::new(l.x+1.0,l.y+1.0),
			decor_type: t,
		}
	}
}

pub fn place_objects(centers: &Vec<Vec2>, sizes: &Vec<Vec2>, commands: &mut Commands){
	let mut rng = rand::thread_rng();

	for a in centers.iter().zip(sizes.iter()){
		let (center,size) = a;
		//generate room type
		//large room
		if size.x >= 11. && size.y >= 11. {
			match rng.gen_range(1..=3){
				//pilars
				1 => {
					add_pilars(&center,&size,commands);
				},
				//maze
				2 => {

				},
				//normal furniture
				3 => {
					
				},
				_ => {
					println!("BAD");
				}
			}
		}
		//small room
		else {
			//normal furniture

		}
		//commands.spawn().insert(Decor::new(center,DecorType::Plant));
	}
}

fn add_pilars(center: &Vec2, size: &Vec2, commands: &mut Commands){

	let adj_size = Vec2::new((size.x-1.0)/2.,(size.y-1.0)/2.);

	//top right
	commands.spawn().insert(
		Decor::new(
			Vec2::new(center.x+(adj_size.x-4.0),center.y+(adj_size.y-4.0)),
			DecorType::Pillar
		)
	);
	//top left 
	commands.spawn().insert(
		Decor::new(
			Vec2::new(center.x-(adj_size.x-2.0),center.y+(adj_size.y-4.0)),
			DecorType::Pillar
		)
	);
	//bottom right
	commands.spawn().insert(
		Decor::new(
			Vec2::new(center.x+(adj_size.x-4.0),center.y-(adj_size.y-2.0)),
			DecorType::Pillar
		)
	);
	//bottom left
	commands.spawn().insert(
		Decor::new(
			Vec2::new(center.x-(adj_size.x-2.0),center.y-(adj_size.y-2.0)),
			DecorType::Pillar
		)
	);
	// println!("pillar");
	// println!("Size {},{}",size.x,size.y);
	// println!("Coord {}, {}",center.x,center.y);
	let mut rng = rand::thread_rng();
	//add statues
	if rng.gen_bool(0.6){
		// println!("statues");
		//top right
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x+(adj_size.x-4.0),center.y+(adj_size.y-5.0)),
				DecorType::Statue
			)
		);
		//top left 
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x-(adj_size.x-2.0),center.y+(adj_size.y-5.0)),
				DecorType::Statue
			)
		);
	}

	//add plants
	if rng.gen_bool(0.6){
		// println!("plants");
		//top right
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x+(adj_size.x-2.0),center.y+(adj_size.y-2.0)),
				DecorType::Plant
			)
		);
		//top left 
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x-(adj_size.x),center.y+(adj_size.y-2.0)),
				DecorType::Plant
			)
		);
		//bottom right
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x+(adj_size.x-2.0),center.y-(adj_size.y)),
				DecorType::Plant
			)
		);
		//bottom left
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x-(adj_size.x),center.y-(adj_size.y)),
				DecorType::Plant
			)
		);
	}

	if rng.gen_bool(0.6){
		// println!("sofas");
		//top right sofa
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x+(adj_size.x-5.0),center.y+(adj_size.y-2.0)),
				DecorType::Sofa
			)
		);
		//top left sofa 
		commands.spawn().insert(
			Decor::new(
				Vec2::new(center.x-(adj_size.x-3.0),center.y+(adj_size.y-2.0)),
				DecorType::Sofa
			)
		);
	}

}
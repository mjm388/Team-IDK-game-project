use bevy::{
	prelude::*,
};

use rand::Rng;

#[derive(Clone, Eq, PartialEq, Copy)]
pub enum DecorType{
	Statue,
	Plant,
	Sofa,
	Chair,
	Lamp,
	Pillar,
	Bookshelf,
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
					add_pillars(&center,&size,commands);
				},
				//maze
				2 => {
					//todo
				},
				//normal furniture
				3 => {
					//todo
				},
				_ => {
					println!("BAD");
				}
			}
		}
		//small room
		else {
			//normal furniture
			place_furniture(&center,&size,commands);

		}
		//commands.spawn().insert(Decor::new(center,DecorType::Plant));
	}
}

fn add_pillars(center: &Vec2, size: &Vec2, commands: &mut Commands){

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
	println!("pillar");
	println!("Size {},{}",size.x,size.y);
	println!("Coord {}, {}",center.x,center.y);
	let mut rng = rand::thread_rng();
	//add statues
	if rng.gen_bool(0.6){
		println!("statues");
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
		println!("plants");
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
		println!("sofas");
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

fn place_furniture(center: &Vec2, size: &Vec2, commands: &mut Commands){
	let mut rng = rand::thread_rng();

	let adj_size = Vec2::new((size.x-1.0)/2.,(size.y-1.0)/2.);

	let top_left = Vec2::new(center.x-(adj_size.x),center.y+(adj_size.y-2.0));
	let top_right = Vec2::new(center.x+(adj_size.x-2.0),center.y+(adj_size.y-2.0));
	let bottom_left = Vec2::new(center.x-(adj_size.x),center.y-(adj_size.y));
	let bottom_right = Vec2::new(center.x+(adj_size.x-2.0),center.y-(adj_size.y));

	let mut corners = 0;
	while corners < 4 {
		if rng.gen_bool(0.7){
			let mut already_placed: Vec<DecorType> = Vec::new();
			let mut num_placed = 0.;
			match corners {
				//top left
				0 => {
					while num_placed < 3. && (num_placed < (adj_size.x-1.0)) && rng.gen_bool(0.7){
						if init_furniture(Vec2::new(top_left.x+num_placed,top_left.y),commands,&mut already_placed){
							num_placed += 1.;
						}
					}

				},
				//top right
				1 => {
					while num_placed < 3. && (num_placed < (adj_size.x-1.0)) && rng.gen_bool(0.7){
						if init_furniture(Vec2::new(top_right.x-num_placed,top_right.y),commands,&mut already_placed){
							num_placed += 1.;
						}
					}
				},
				//bottom left
				2 => {
					while num_placed < 3. && (num_placed < (adj_size.x-1.0)) && rng.gen_bool(0.7){
						if init_furniture(Vec2::new(bottom_left.x+num_placed,bottom_left.y),commands,&mut already_placed){
							num_placed += 1.;
						}
					}
				},
				//bottom right
				3 => {
					while num_placed < 3. && (num_placed < (adj_size.x-1.0)) && rng.gen_bool(0.7){
						if init_furniture(Vec2::new(bottom_right.x-num_placed,bottom_right.y),commands,&mut already_placed){
							num_placed += 1.;
						}
					}
				},
				_ => {
					println!("BAD");
				}
			}
		}
		corners += 1;
	}
}
//return true if there is no repeat within a row
fn init_furniture(index: Vec2, commands: &mut Commands, already_placed: &mut Vec<DecorType>) -> bool{
	let mut rng = rand::thread_rng();

	match rng.gen_range(1..=4){
		1 => {
			if !already_placed.contains(&DecorType::Chair){
				commands.spawn().insert(Decor::new(index,DecorType::Chair));
				already_placed.push(DecorType::Chair);
			}
			else{
				return false;
			}
		},
		2 => {
			if !already_placed.contains(&DecorType::Lamp){
				commands.spawn().insert(Decor::new(index,DecorType::Lamp));
				already_placed.push(DecorType::Lamp);
			}
			else{
				return false
			}
		},
		3 => {
			if !already_placed.contains(&DecorType::Plant){
				commands.spawn().insert(Decor::new(index,DecorType::Plant));
				already_placed.push(DecorType::Plant);
			}
			else{
				return false;
			}
		},
		4 => {
			commands.spawn().insert(Decor::new(index,DecorType::Bookshelf));
		},
		_ => {
			println!("BAD");
		}
	}
	true
}
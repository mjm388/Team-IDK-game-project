use rand::Rng;
use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,
};

use crate::{
	GameState,
    //movement::OverworldPlayer,
};

pub struct RoomGenPlugin;

impl Plugin for RoomGenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(generate_rooms)
        .add_system_set(SystemSet::on_update(GameState::Overworld))
		.add_system_set(SystemSet::on_enter(GameState::Overworld))
		.add_system_set(SystemSet::on_exit(GameState::Overworld))
        ;
    }
}


#[derive(Component)]
pub struct Room {
    pub size: Vec2,
	pub id: i32,
    pub center: Vec3,
}
impl Room {
	fn new(size: Vec2, id: i32, center: Vec3) -> Room {
		Room {
			size,
			id,
            center,
		}
	}
}

// Create bounds on where to put in window
const X_BOUND: f32 = 50.;
const Y_BOUND: f32 = 50.;

// Create bounds on size of room
const SIZE_LOWER_BOUND: f32 = 3.;  //7
const SIZE_UPPER_BOUND: f32 = 7.;  //15

pub const NUM_OF_ROOMS: i32 = 15;
const ROOM_BUFFER_SPACE: f32 = 10.;

fn generate_rooms(
    mut commands: Commands,
    //mut player: Query<&mut Transform, With<OverworldPlayer>>,
    //mut camera: Query<&mut Transform, (With<Camera>,Without<OverworldPlayer>)>
) {
    let mut rng = rand::thread_rng();

    let mut coords = Vec::new();
    let mut sizes = Vec::new();

    let spawnroom = rng.gen_range(0..NUM_OF_ROOMS);

    let mut vertices: Vec<Vec2> = Vec::new();

    let mut i = 0;
    loop {
        if i >= NUM_OF_ROOMS {
            break;
        }
        // Randomly generate location of the room
        let coord = match i {
            0 => Vec3::new(
                0.,
                0.,
                1.,
            ),
            _ => Vec3::new(
                rng.gen_range::<f32,_>(-X_BOUND..X_BOUND).floor(),
                rng.gen_range::<f32,_>(-Y_BOUND..Y_BOUND).floor(),
                1.,
            ),
        };

        // Randomly generate dimensions of the room
        let size = Vec2::new(
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1.,
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1.,
        );

        // Check if this room overlaps another
        if !overlap(&coord, &size, &coords, &sizes) {
            coords.push(coord.clone());
            sizes.push(size.clone());
            //println!("Room {}: coord: {:?}  size:{}", i, &coord, &size);
            println!("store_rooms2({:?})", &coord);
            commands.spawn()
                .insert(Room::new(size,i, coord))
                .insert(Transform::from_translation(coord));
            i += 1;
            vertices.push(Vec2::new(coord.x, coord.y));
        }
    }
    let vertices = vertices;
    info!("Vertices: {} \n ", vertices.len());   

    let final_polygon = triangulate(&vertices);     // DELAUNAY
    // let final_polygon = prims(final_polygon)     // PRIMS

    for edge in final_polygon.iter() {
        //bresenhams((edge.0.x as i32, edge.0.y as i32), (edge.1.x as i32, edge.1.y as i32));
        // call a* to generate hallways             // A*
        info!("We have an edge");
    }
}

fn overlap(
	room_pos: &Vec3,
    room_length: &Vec2,
	pos_list: &Vec<Vec3>,
    size_list: &Vec<Vec2>,
) -> bool {
    for i in 0..size_list.len() {
        let overlap = collide (
        	*room_pos,
        	*room_length,
        	pos_list[i],
           	Vec2::new(
                size_list[i].x+ROOM_BUFFER_SPACE,
                size_list[i].y+ROOM_BUFFER_SPACE,
            ),
        );
        if overlap.is_some() {
            return true;
        }
    }
    false
}


// DELAUNAY TRIANGULATION CODE 


const BTA: Vec2 = Vec2::new(-50., -50.);
const BTB: Vec2 = Vec2::new(-50., 150.);
const BTC: Vec2 = Vec2::new(150., -50.);

pub struct Edge (Vec2, Vec2);

pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
    pub stay: bool,
}
impl Triangle {
	fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
		Triangle {
			a,
            b,
            c,
            stay: true,
		}
	}
}

fn triangulate(vertices: &Vec<Vec2>) -> Vec<Edge> {

    let mut triangles: Vec<Triangle> = Vec::new();
    triangles.push(Triangle::new(BTA, BTB, BTC));

    for vertex in vertices.iter() {
        // For each triangle, check if point is inside of its circumcircle
        // if not, it does not stay in the next iteration
        for triangle in triangles.iter_mut() {  
            if check_circle(&vertex, &triangle) {
                triangle.stay = false;
            }
        }
        
        let mut polygon: Vec<Edge> = Vec::new();
        let bad_tri: Vec<_> = triangles.iter().filter(|t| !t.stay).collect();
        // info!("bad {}", bad_tri.len());
        // for i in 0..bad_tri.len() {
        //     info!("{} {} {} {}", i, bad_tri[i].a, bad_tri[i].b, bad_tri[i].c);
        // }
        if bad_tri.len() <= 0 {
            continue; //impossible
        }
        else if bad_tri.len() == 1 {
            polygon.push(Edge(bad_tri[0].a, bad_tri[0].b));
            polygon.push(Edge(bad_tri[0].b, bad_tri[0].c));
            polygon.push(Edge(bad_tri[0].a, bad_tri[0].c));
        }
        else {// bad_tri.len() >= 2 
            for i in 0..bad_tri.len() {
                for ti in 1..=3 {
                    let edge = match ti {
                        1 => Edge(bad_tri[i].a, bad_tri[i].b),
                        2 => Edge(bad_tri[i].b, bad_tri[i].c),
                        3 => Edge(bad_tri[i].a, bad_tri[i].c),
                        _ => Edge(bad_tri[i].a, bad_tri[i].c), // impossible
                    };
                    let mut duplicate = false;
                    for j in 0..bad_tri.len() {
                        if i == j {continue;}
                        for tj in 1..=3 {
                            let edge2 = match tj {
                                1 => Edge(bad_tri[j].a, bad_tri[j].b),
                                2 => Edge(bad_tri[j].b, bad_tri[j].c),
                                3 => Edge(bad_tri[j].a, bad_tri[j].c),
                                _ => Edge(bad_tri[j].a, bad_tri[j].c), // impossible
                            };
                            if same_e(&edge, &edge2) {
                                // info!("edge {}.{} dup: {} {} {} {}", i, ti, edge.0, edge.1, edge2.0, edge2.1);
                                duplicate = true;
                                break;
                            }
                        }
                    }
                    if !duplicate {
                         info!("edge {}.{} pushed: {} {}", i, ti, &edge.0, &edge.1);
                        polygon.push(edge);
                    }
                }
            }
        }
        // remove bad triangles
        triangles.retain(|t| t.stay);
        
        // insert new triangles
        for edge in polygon.iter() {
            let new_triangle = Triangle::new(edge.0, edge.1, *vertex);
            triangles.push(new_triangle);
        }
        //info!("{} edges in added, {} tris now", polygon.len(), triangles.len());
    }

    // // remove big triangle
    // info!("triangles: {}",triangles.len());
    triangles.retain(|t| t.a != BTA && t.a != BTB && t.a != BTC);
    triangles.retain(|t| t.b != BTA && t.b != BTB && t.b != BTC);
    triangles.retain(|t| t.c != BTA && t.c != BTB && t.c != BTC);
    // info!("after removing BT vertices: {}",triangles.len());

    return poly(triangles);
}

fn poly(
    ts: Vec<Triangle>
) -> Vec<Edge> {
    let mut polygon: Vec<Edge> = Vec::new();

    for t in ts.iter() {
        if polygon.len() == 0 {
            polygon.push(Edge(t.a, t.b)); 
            polygon.push(Edge(t.b, t.c));
            polygon.push(Edge(t.a, t.c));
        }
        else {
            for ti in 1..=3 {
                let new_edge = match ti {
                    1 => Edge(t.a, t.b),
                    2 => Edge(t.b, t.c),
                    3 => Edge(t.a, t.c),
                    _ => Edge(t.a, t.c), // impossible
                };
                let mut duplicate = false;
                for edge in polygon.iter() {
                    if same_e(&edge, &new_edge) {
                        duplicate = true;
                        continue;
                    }
                }
                if !duplicate {
                    polygon.push(new_edge);
                }
            }
        }
    }
    polygon
}

// Will check if given point is inside of given triangle's circumcirle
fn check_circle(
    vertex: &Vec2,
    triangle: &Triangle,
) -> bool {
    // find distances of edges
    let ab_len = ((triangle.a.x - triangle.b.x).powf(2.) + (triangle.a.y - triangle.b.y).powf(2.)).sqrt();
    let bc_len = ((triangle.b.x - triangle.c.x).powf(2.) + (triangle.b.y - triangle.c.y).powf(2.)).sqrt();
    let ac_len = ((triangle.a.x - triangle.c.x).powf(2.) + (triangle.a.y - triangle.c.y).powf(2.)).sqrt();

    let ab_mid = Vec2::new((triangle.a.x + triangle.b.x) / 2., (triangle.a.y + triangle.b.y) / 2.);
    let bc_mid = Vec2::new((triangle.b.x + triangle.c.x) / 2., (triangle.b.y + triangle.c.y) / 2.);

    // find radius of circle
    let s = (ab_len + bc_len + ac_len) / 2.;
    let area = (s * (s - ab_len) * (s - bc_len) * (s - ac_len)).sqrt();
    let r = (ab_len * bc_len * ac_len) / (4. * area);
    
    //find slope tangent line of edges
    let ab_tan_s = -(triangle.a.x-triangle.b.x)/(triangle.a.y-triangle.b.y);
    let bc_tan_s = -(triangle.b.x-triangle.c.x)/(triangle.b.y-triangle.c.y);

    // find origin of circle
    let origin = line_intersection(ab_mid, ab_tan_s, bc_mid, bc_tan_s);
    //println!("{} {}", origin, r);

    // check if point is inside of the circumcirle
    let diff = ((vertex.x - origin.x).powf(2.) + (vertex.y - origin.y).powf(2.)).sqrt();
    if diff <= r {
        return true;
    }
    false
}

fn line_intersection (
    a1: Vec2,
    am: f32,
    b1: Vec2,
    bm: f32,
) -> Vec2 {
    let a2 = 
    if am.abs() ==  f32::INFINITY {
        Vec2::new(a1.x, a1.y + 1.)
    }
    else {
        Vec2::new(a1.x + 1.,a1.y + am)
    };
        
    let b2 = 
    if bm.abs() ==  f32::INFINITY {
        Vec2::new(b1.x, b1.y + 1.)
    }
    else {
        Vec2::new(b1.x + 1.,b1.y + bm)
    };

    let a_x_diff = a1.x - a2.x;
    let a_y_diff = a1.y - a2.y;
    let b_x_diff = b1.x - b2.x;
    let b_y_diff = b1.y - b2.y;

    let determinant = a_x_diff * b_y_diff - a_y_diff * b_x_diff;
    
    if determinant != 0. {
        let c_a = a1.x * a2.y - a1.y * a2.x;
        let c_b = b1.x * b2.y - b1.y * b2.x;

        let x = (b_x_diff * c_a - a_x_diff * c_b) / determinant;
        let y = (b_y_diff * c_a - a_y_diff * c_b) / determinant;
        return Vec2::new(x, y);
    }
    Vec2::new(0.,0.)
    // flat triangle ???
}

fn same_e (e1: &Edge, e2: &Edge) -> bool {
    if e1.0 == e2.0 {
        if e1.1 == e2.1 {
            return true;
        }
    }
    else if e1.0 == e2.1 {
        if e1.1 == e2.0 {
            return true;
        }
    }
    false
}

fn bresenhams (
    mut commands: Commands,
    p1: Vec2,
    p2: Vec2,
) {

}
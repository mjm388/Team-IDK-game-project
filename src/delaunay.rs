
use bevy::{
	prelude::*,
};

use crate::{
	GameState,
};

// pub struct DelaunayPlugin;

// impl Plugin for DelaunayPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         .add_startup_system(delaunay_test.after(generate_rooms))
//         .add_system_set(SystemSet::on_update(GameState::Overworld)
// 		)
// 		.add_system_set(SystemSet::on_enter(GameState::Overworld)
// 		)
// 		.add_system_set(SystemSet::on_exit(GameState::Overworld)
//         );
//     }
// }

let mut triangulation = Vec::new();
let mut vertices = Vec::new();
let const bta = Vec2::new(-50., -50.);
let const btb = Vec2::new(-50., 150.);
let const btc = Vec2::new(150., -50.);

pub struct Triangle {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}
impl Triangle {
	fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle {
		Triangle {
			a,
            b,
            c,
		}
	}
}

fn check_generation() {
    if generated {
        store_rooms();
        triangulate();
    }
}

fn store_rooms(
    rooms: Query<&Room>,
    room_tfs: Query<&Transform, With<Room>>,
) {
    for unzip in rooms.iter().zip(room_tfs.iter()) {    // add each room to vertices
        let (room, room_tf) = unzip;
        let room_coord = room_tf.translation;
        vertices.push(Vec2::new(room_coord.x, room_coord.y));
    }
}

fn triangulate(

) {

    let mut bad_triangles = Vec::new();
    // Inserts big triangle
    commands
        .spawn()
        .insert(Triangle::new(Vec2::new(-50., -50.), Vec2::new(-50., 150.), Vec2::new(150., -50.)))
        .insert(BigTriangle);

    for vertex in vertices.iter() {

        let mut bad_triangles = Vec::new(bta, btb, btc);

        for triangle in triangulation.iter() {  // For each triangle, check if point is inside of its circumcircle
            if check_circle(&vertex, &triangle) {
                bad_triangles.push(triangle);
                graph.add_edge();
            }
        }

        let mut polygon = Vec::new();

        for triangle in bad_triangles.iter() {
        }
    }

    for triangle in triangulation.iter() {
        if triangle.a == bta || triangle.a == btb || triangle.a == btc {
            triangulation.remove(triangle);
        }
        else if triangle.b == bta || triangle.b == btb || triangle.b == btc {
            triangulation.remove(triangle);
        }
        else if triangle.c == bta || triangle.c == btb || triangle.c == btc {
            triangulation.remove(triangle);
        }
    }

}

// Will check if given point is inside of given triangle's circumcirle
fn check_circle(
    vertex: &Vec2,
    triangle: &Vec<Vec2>,
) -> bool {
    // find distances of edges
    let ab_len = sqrt((triangle.a[0] - triangle.b[0])^2 + (triangle.a[1] - triangle.b[1])^2);
    let bc_len = sqrt((triangle.b[0] - triangle.c[0])^2 + (triangle.b[1] - triangle.c[1])^2);
    let ac_len = sqrt((triangle.a[0] - triangle.c[0])^2 + (triangle.a[1] - triangle.c[1])^2);

    let ab_midpoint = Vec::new((triangle.a[0] + triangle.b[0]) / 2, (triangle.a[1] + triangle.b[1]) / 2);
    let bc_midpoint = Vec::new((triangle.b[0] + triangle.c[0]) / 2, (triangle.b[1] + triangle.c[1]) / 2);
    let ac_midpoint = Vec::new((triangle.a[0] + triangle.c[0]) / 2, (triangle.a[1] + triangle.c[1]) / 2);

    // find radius of circle
    let s = (ab_len + bc_len + ac_len) / 2;
    let area = sqrt(s * (s - ab_len) * (s - bc_len) * (s - ac_len));
    let r = (ab_len * bc_len * ac_len) / (4 * area);

    //find midpoint and slope for first line
    let midABx = ab_midpoint.x;
    let midABy = ab_midpoint.y;
    let mAB = (triangle.a.y-triangle.b.y)/(triangle.a.x-triangle.b.x);

    //find midpoint and slope of second line
    let midBCx = bc_midpoint.x;
    let midBCy = bc_midpoint.y;
    let mBC = (triangle.b.y-triangle.c.y)/(triangle.b.x-triangle.c.x);

    let circumX = (-(midABx*mBC) - (midABy*mAB*mBC) + (midBCx*mAB) + (midBCy*mAB*mBC))/(mAB-mBC);
    let circumY = (-circumX/mAB) + (midABx/mAB) + midABy;

    // find origin of circle
    let origin = Vec::new(circumX,circumY);

    // check if point is inside of the circumcirle
    let diff = sqrt((vertex[0] - origin[0])^2 + (vertex[1] - origin[1])^2);
    if (diff <= radius) {
        return true;
    }
    false
}



let mut vertices = Vec::new();

#[derive(Component)]
pub struct BigTriangle;

#[derive(Component)]
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
    mut commands: Commands,
    big_triangle: Query<&BigTriangle>,
    triangles: Query<&Triangle>,
) {
    // Inserts big triangle
    commands 
        .spawn()
        .insert(Triangle::new(Vec2::new(-50., -50.), Vec2::new(-50., 150.), Vec2::new(150., -50.)))   
        .insert(BigTriangle);  
        
    for vertex in vertices.iter() {
        for triangle in triangles.iter() {  // For each triangle, check if point is inside of its circumcircle

        }
    }
}

// Will check if given point is inside of given triangle's circumcirle
fn check_circles(

) {

}
use bevy::{
	prelude::*,
};
const BTA: Vec2 = Vec2::new(-150., -150.);
const BTB: Vec2 = Vec2::new(-150., 350.);
const BTC: Vec2 = Vec2::new(350., -150.);

use super::Edge;

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

pub fn triangulate(vertices: &Vec<Vec2>) -> Vec<Edge> {

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
                        //info!("edge {}.{} pushed: {} {}", i, ti, &edge.0, &edge.1);
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
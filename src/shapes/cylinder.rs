use crate::types::{Face, Vertex};
use genmesh::generators;

pub fn cylinder(u: usize, h: usize) -> (Vec<Vertex>, Vec<Face>) {
    let gen = generators::Cylinder::subdivide(u, h);
    super::genmesh_tools::generate_mesh(gen, create_vertex)
}

// z and y swapped because of coordinate system diffs (so up is y instead of z)
fn create_vertex([x, z, y]: [f32; 3], [n1, n3, n2]: [f32; 3]) -> Vertex {
    Vertex {
        position: [x, y, z],
        normal: [n1, n2, n3],
        uv: uv(x, y, z),
        tangent: tangent(n1, n2, n3),
    }
}

fn uv(x: f32, y: f32, z: f32) -> [f32; 2] {
    let n = glm::Vec2::new(x, z).normalize();
    let u = n.x.atan2(n.y) / (2. * glm::pi::<f32>()) + 0.5;
    let v = y * 0.5 + 0.5;
    [u, v]
}

fn tangent(n1: f32, n2: f32, n3: f32) -> [f32; 3] {
    let normal = glm::Vec3::new(n1, n2, n3);
    let up = glm::Vec3::new(0., 1., 0.);
    let tangent = glm::cross(&up, &normal);
    tangent.into()
}

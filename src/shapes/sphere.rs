use crate::types::{Face, Vertex};
use genmesh::generators;

pub fn sphere(u: usize, v: usize) -> (Vec<Vertex>, Vec<Face>) {
    let gen = generators::SphereUv::new(u, v);
    super::genmesh_tools::generate_mesh(gen, create_vertex)
}

pub fn create_vertex([x, y, z]: [f32; 3], [n1, n2, n3]: [f32; 3]) -> Vertex {
    let uv = uv(x, y, z);
    let tangent = tangent(n1, n2, n3);
    Vertex {
        position: [x, y, z],
        normal: [n1, n2, n3],
        uv,
        tangent,
    }
}

fn tangent(n1: f32, n2: f32, n3: f32) -> [f32; 3] {
    let normal = glm::Vec3::new(n1, n2, n3);
    let up = glm::Vec3::new(0., 1., 0.);

    let tangent = glm::cross(&up, &normal);
    [tangent.x, tangent.y, tangent.z]
}

fn uv(x: f32, y: f32, z: f32) -> [f32; 2] {
    let n = glm::Vec3::new(x, y, z).normalize();
    let u = n.x.atan2(n.z) / (2. * glm::pi::<f32>()) + 0.5;
    let v = n.y * 0.5 + 0.5;
    [u, v]
}

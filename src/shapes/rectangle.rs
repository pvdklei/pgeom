use crate::types::{Face, Vertex};

pub fn rect() -> (Vec<Vertex>, Vec<Face>) {
    let normal = [0., 0., 1.];
    let tangent = [1., 0., 0.];
    let v = vec![
        Vertex {
            position: [-0.5, 0.5, 0.],
            normal,
            uv: [0., 1.],
            tangent,
        },
        Vertex {
            position: [0.5, 0.5, 0.],
            normal,
            uv: [1., 1.],
            tangent,
        },
        Vertex {
            position: [0.5, -0.5, 0.],
            normal,
            uv: [1., 0.],
            tangent,
        },
        Vertex {
            position: [-0.5, -0.5, 0.],
            normal,
            uv: [0., 0.],
            tangent,
        },
    ];
    let f = vec![[0, 2, 1], [0, 3, 2]];
    (v, f)
}

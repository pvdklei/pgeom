pub mod arrow;
pub mod cone;
pub mod cylinder;
pub mod grid;
pub mod rectangle;
pub mod sphere;

use crate::types::{Face, Vertex};
type Shape = (Vec<Vertex>, Vec<Face>);

mod genmesh_tools {
    use crate::types::{Face, Vertex};
    use genmesh::{
        generators::{IndexedPolygon, SharedVertex},
        EmitTriangles, Triangulate,
    };

    pub fn generate_mesh<T, P>(
        generator: T,
        vertex_from_pos_and_normal: fn([f32; 3], [f32; 3]) -> Vertex,
    ) -> (Vec<Vertex>, Vec<Face>)
    where
        P: EmitTriangles<Vertex = usize>,
        T: SharedVertex<genmesh::Vertex> + IndexedPolygon<P>,
    {
        let vertices = generator
            .shared_vertex_iter()
            .map(|v| vertex_from_pos_and_normal(v.pos.into(), v.normal.into()))
            .collect::<Vec<_>>();

        let indices = generator
            .indexed_polygon_iter()
            .triangulate()
            .map(|t| [t.x as u32, t.y as u32, t.z as u32])
            .collect::<Vec<_>>();

        (vertices, indices)
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub uv: [f32; 2],
}
#[derive(Debug)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: Option<[f32; 3]>,
    pub tangent: Option<[f32; 3]>,
    pub uv: Option<[f32; 2]>,
}

pub type Face = [u32; 3];

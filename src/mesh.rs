use crate::material::Material;
use crate::types::{Face, MeshVertex, Vertex};
unzip_n::unzip_n!(4);

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<[u32; 3]>,
    pub positions: Vec<[f32; 3]>,
    pub uvs: Option<Vec<[f32; 2]>>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub tangents: Option<Vec<[f32; 3]>>,
    pub material: Option<Material>,
    pub name: Option<String>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            faces: Vec::new(),
            positions: Vec::new(),
            uvs: None,
            normals: None,
            tangents: None,
            material: None,
            name: None,
        }
    }
}

impl Mesh {
    /// Calculates the tangent vectors based on the texture coordinates
    /// and positions of the three vertices that make up a face. A vertex could
    /// be part of multiple faces (i.e., has multiple tangents), so we take
    /// average of those faces by first keeping track of the tangent sum and
    /// count of every vertex, and finally divide the sum by the count to
    /// get the average.
    ///
    /// Maths: https://learnopengl.com/Advanced-Lighting/Normal-Mapping
    pub fn calculate_tangents(&mut self) {
        let mut countsum = Vec::with_capacity(self.positions.len());
        countsum.resize(self.positions.len(), (0, na::Vector3::<f32>::zeros()));

        let uvs = self
            .uvs
            .as_ref()
            .expect("[ERROR] To calculate tangents the mesh must contain texture coordinates.");

        // Finding sums and count
        for [i1, i2, i3] in self.faces.iter() {
            let uv1: na::Vector2<f32> = uvs[*i1 as usize].into();
            let uv2: na::Vector2<f32> = uvs[*i2 as usize].into();
            let uv3: na::Vector2<f32> = uvs[*i3 as usize].into();

            let pos1: na::Vector3<f32> = self.positions[*i1 as usize].into();
            let pos2: na::Vector3<f32> = self.positions[*i2 as usize].into();
            let pos3: na::Vector3<f32> = self.positions[*i3 as usize].into();

            let duv1 = uv2 - uv1;
            let duv2 = uv3 - uv1;

            let e1 = pos2 - pos1;
            let e2 = pos3 - pos1;

            let e = na::Matrix2x3::from_rows(&[e1.transpose(), e2.transpose()]);
            let d = na::Matrix2::new(duv2.y, -duv1.y, -duv2.x, duv1.x);
            let fac = duv1.x * duv2.y - duv2.x * duv1.y;

            let tb = 1. / fac * d * e;
            let t = tb.row(0).transpose();
            countsum[*i1 as usize].1 += t;
            countsum[*i1 as usize].0 += 1;
            countsum[*i2 as usize].1 += t;
            countsum[*i2 as usize].0 += 1;
            countsum[*i3 as usize].1 += t;
            countsum[*i3 as usize].0 += 1;
        }

        // Averaging
        self.tangents = Some(
            countsum
                .iter()
                .map(|(n, sum)| (sum / *n as f32).into())
                .collect::<Vec<[f32; 3]>>(),
        );

        println!("{:#?}", self.tangents);
    }

    /// Creates a vertex and index buffer, ready for rendering
    pub fn render_data<F, V>(&self, vgen: F) -> (Vec<V>, Vec<[u32; 3]>)
    where
        F: Fn(MeshVertex) -> V,
    {
        let mut vertices = Vec::with_capacity(self.positions.len());
        for i in 0..self.positions.len() {
            let v = MeshVertex {
                position: self.positions[i],
                normal: if let Some(n) = &self.normals {
                    Some(n[i])
                } else {
                    None
                },
                tangent: if let Some(n) = &self.tangents {
                    Some(n[i])
                } else {
                    None
                },
                uv: if let Some(n) = &self.uvs {
                    Some(n[i])
                } else {
                    None
                },
            };
            let v = vgen(v);
            vertices.push(v);
        }
        (vertices, self.faces.clone())
    }
}

impl From<(Vec<Vertex>, Vec<Face>)> for Mesh {
    fn from((vs, fs): (Vec<Vertex>, Vec<Face>)) -> Self {
        let (positions, normals, uvs, tangents) = vs
            .iter()
            .map(|v| (v.position, v.normal, v.uv, v.tangent))
            .unzip_n();

        Mesh {
            faces: fs,
            positions,
            uvs: Some(uvs),
            normals: Some(normals),
            tangents: Some(tangents),
            material: None,
            name: None,
        }
    }
}

use crate::material::Material;
use crate::mesh::Mesh;
use crate::utils;
use std::path::Path;

pub fn load(path: impl AsRef<Path>) -> Result<Vec<Mesh>, String> {
    if !path.as_ref().exists() {
        return Err("Incorrect path to .obj file".to_string());
    }

    let obj = tobj::load_obj(
        path.as_ref(),
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    );
    if !obj.is_ok() {
        return Err("There is something wrong with the .obj file".to_string());
    }
    let (objmodels, objmaterials) = obj.unwrap();

    if let Err(error) = objmaterials {
        return Err(format!("{:?}", error));
    }

    let objmaterials = objmaterials.unwrap();

    let mut meshes = Vec::new();
    for m in objmodels.iter() {
        let positions = utils::flat_to_triples(&m.mesh.positions);
        let mut uvs = utils::flat_to_doubles(&m.mesh.texcoords);
        let mut normals = utils::flat_to_triples(&m.mesh.normals);
        let faces = utils::flat_to_triples(&m.mesh.indices);

        // SOME CHECKING
        //
        // Quite often the amount of positions is not the same as
        // the amount of normals and or uvs. Therefore, I allow a 10%
        // less, but I have to make sure the sizes match up. Therefore
        // I make everything the size of positions, and fill up with zeros.
        let error_limit = positions.len() * 9 / 10;

        let uvs = if uvs.is_empty() {
            None
        } else if uvs.len() == positions.len() {
            Some(uvs)
        } else if uvs.len() > error_limit && uvs.len() < positions.len() {
            println!("[WARNING] Some uv coords are missing in this mesh.");
            uvs.resize_with(positions.len(), || [0., 0.]);
            Some(uvs)
        } else {
            return Err(format!(
                "[ERROR] The amount of uvs in the obj mesh is by far not the same as \
                                the amount of positions. Positions: {}, UVs: {}",
                positions.len(),
                uvs.len()
            ));
        };
        let normals = if normals.is_empty() {
            None
        } else if normals.len() == positions.len() {
            Some(normals)
        } else if normals.len() > error_limit && normals.len() < positions.len() {
            println!("[WARNING] Some normals are missing in this mesh.");
            normals.resize_with(positions.len(), || [0., 0., 0.]);
            Some(normals)
        } else {
            return Err(format!(
                "[ERROR] The amount of normals in the obj mesh is by far not the same as \
                                the amount of positions. Positions: {}, Normals: {}",
                positions.len(),
                normals.len()
            ));
        };

        let mesh = Mesh {
            name: Some(m.name.clone()),
            positions,
            uvs,
            normals,
            faces,
            material: {
                if let Some(id) = m.mesh.material_id {
                    let m = &objmaterials[id];
                    let &tobj::Material {
                        specular,
                        shininess,
                        ambient,
                        diffuse,
                        ..
                    } = m;
                    Some(Material {
                        specular,
                        ambient,
                        shininess,
                        diffuse,
                    })
                } else {
                    None
                }
            },
            ..Default::default()
        };
        meshes.push(mesh);
    }
    Ok(meshes)
}

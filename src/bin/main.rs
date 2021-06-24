fn main() {
    let meshes = pgeom::obj::load("models/gun/Handgun_obj.obj").unwrap();
    //let (v, i) = meshes[2].render_data(|v| v.uv);
    meshes.iter().for_each(|m| {
        println!("{:#?}", m.name);
    });
    //println!("{:#?}", v);
}

use crate::types::{Face, Vertex};
use crate::{cone, cylinder};

/// Ratios are cylinder / cone
pub fn arrow(thickness_ratio: f32, length_ratio: f32) -> (Vec<Vertex>, Vec<Face>) {
    let (mut cyv, mut cyi) = cylinder(10, 1);
    let (mut cov, mut coi) = cone(10);
    cov.iter_mut()
        .for_each(|v| v.position[1] = (v.position[1] + 1.) / length_ratio + 1.); // move cone up 1m, scale, and then the final 1m to put it on the cylinder
    cyv.iter_mut().for_each(|v| {
        v.position[0] *= thickness_ratio;
        v.position[2] *= thickness_ratio;
    });
    let add = cov.len() as u32;
    cyi.iter_mut().for_each(|f| {
        f[0] += add;
        f[1] += add;
        f[2] += add;
    });
    cov.append(&mut cyv);
    coi.append(&mut cyi);
    cov.iter_mut().for_each(|v| {
        v.position[0] /= length_ratio;
        v.position[2] /= length_ratio;
    });
    (cov, coi)
}

use crate::types::{Face, Vertex};

pub fn grid(w: usize, h: usize) -> super::Shape {
	grid_function(w, h, |_, _| 0.0)
}

pub fn monkey_saddle(w: usize, h: usize) -> super::Shape {
	grid_function(w, h, |mut u, mut v| {
		u = (u - 0.5) * 3.0;
		v = (v - 0.5) * 3.0;
		u.powi(3) + 3.0 * u * v * v
	})
}

pub fn grid_function(w: usize, h: usize, f: impl Fn(f32, f32) -> f32) -> (Vec<Vertex>, Vec<Face>) {
	let mut vertices = Vec::new();
	let mut faces = Vec::new();
	let hstep = 1.0 / w as f32;
	let vstep = 1.0 / h as f32;
	for i in 0..h {
		for j in 0..w {
			let x = 1.0 - i as f32 * vstep;
			let z = 1.0 - j as f32 * hstep;
			vertices.push(vertex(x, z, &f));
		}
	}
	for i in 0..h - 1 {
		for j in 0..w - 1 {
			let base: u32 = (i * w + j) as u32;
			let w: u32 = w as u32;
			faces.push([base, base + w + 1, base + 1]);
			faces.push([base, base + w, base + w + 1]);
		}
	}
	(vertices, faces)
}

fn vertex(x: f32, z: f32, f: &impl Fn(f32, f32) -> f32) -> Vertex {
	let y = f(x, z);
	let h = 0.01;
	let yu = (f(x + h, z) - y) / h;
	let yv = (f(x, x + h) - y) / h;
	let n = na::Vector3::new(yu, 1.0, yv).normalize();
	let t = n.cross(&na::Vector3::x());
	Vertex {
		uv: [x, z],
		position: [x - 0.5, y, z - 0.5], // to get center at origin
		normal: n.into(),
		tangent: t.into(),
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn try_() {
		super::grid(3, 3);
	}
}

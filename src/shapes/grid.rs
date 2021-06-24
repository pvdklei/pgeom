use crate::types::{Face, Vertex};

pub fn grid(w: usize, h: usize) -> (Vec<Vertex>, Vec<Face>) {
	let mut vertices = Vec::new();
	let mut faces = Vec::new();
	let hstep = 1.0 / w as f32;
	let vstep = 1.0 / h as f32;
	for i in 0..h {
		for j in 0..w {
			let x = 1.0 - i as f32 * vstep;
			let y = 1.0 - j as f32 * hstep;
			vertices.push(vertex(x, y));
		}
	}
	for i in 0..h-1 {
		for j in 0..w-1 {
			let base: u32 = (i * w + j) as u32;
			let w: u32 = w as u32;
			faces.push([base, base + w + 1, base + 1]);
			faces.push([base, base + w, base + w + 1]);
		}
	}
	(vertices, faces)
}

fn vertex(x: f32, y: f32) -> Vertex {
	Vertex {
		uv: [x, y], 
		position: [x-0.5, y-0.5, 0.0], // to get center at origin 
		normal: [0.0, 0.0, 1.0],
		tangent: [0.0, 1.0, 0.0],
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn try_() {
		let g = super::grid(3, 3);
	}
}

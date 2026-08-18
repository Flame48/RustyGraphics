use crate::scene::math::matrix::Matrix;

pub struct Triangle {
    verts: Matrix<3, 4>,
}

impl Triangle {
    pub fn new(p1: [f32; 3], p2: [f32; 3], p3: [f32; 3]) -> Self {
        let to_homogeneous = |p: [f32; 3]| [p[0], p[1], p[2], 1.0];
        Self {
            verts: Matrix {
                data: [to_homogeneous(p1), to_homogeneous(p2), to_homogeneous(p3)],
            },
        }
    }
}

pub struct Mesh {
    tris: Vec<Triangle>,
}

impl Mesh {
    pub fn construct_cube() -> Self {
        let v: [[f32; 3]; 8] = [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];

        let faces: [[usize; 3]; 12] = [
            [0, 2, 1],
            [0, 3, 2],
            [4, 5, 6],
            [4, 6, 7],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
        ];

        let tris = faces
            .iter()
            .map(|&[a, b, c]| Triangle::new(v[a], v[b], v[c]))
            .collect();

        Self { tris }
    }
}

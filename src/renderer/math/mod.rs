pub mod matrix;

pub struct Triangle {
    verts: matrix::Matrix<3, 4>,
}

pub struct Mesh {
    tris: Vec<Triangle>,
}

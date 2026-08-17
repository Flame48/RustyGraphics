#[derive(Clone, Copy, PartialEq)]
pub struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Triangle {
    verts: [V3; 3],
}

pub struct Mesh {
    tris: Vec<Triangle>,
}

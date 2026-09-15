use crate::scene::math::matrix::RowMat;

#[derive(Clone, Copy)]
pub struct Fragment {
    pub screen_x: u32,
    pub screen_y: u32,
    pub depth: f32,
    pub color: [u8; 4],
    pub position: RowMat<3>,
    pub normal: RowMat<3>,
    pub uv: RowMat<2>,
}

impl Fragment {
    pub fn new(
        sx: u32,
        sy: u32,
        depth: f32,
        color: [u8; 4],
        position: RowMat<3>,
        normal: RowMat<3>,
        uv: RowMat<2>
    ) -> Self {
        Self { screen_x: sx, screen_y: sy, depth, color, position, normal, uv }
    }
}

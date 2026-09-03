use crate::scene::renderer::fragment::Fragment;

pub type RGBA = [u8; 4];

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub color: Vec<RGBA>,
    pub depth: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width * height) as usize;
        Self {
            width,
            height,
            color: vec![[0, 0, 0, 255]; len],
            depth: vec![f32::INFINITY; len],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height {
            return;
        }
        *self = Self::new(width, height);
    }

    pub fn clear(&mut self) {
        self.color.fill([0, 0, 0, 0]);
        self.depth.fill(f32::INFINITY);
    }

    pub fn depth_test_idx(&self, idx: usize, depth: f32) -> bool {
        depth < self.depth[idx]
    }

    pub fn depth_test(&self, x: u32, y: u32, depth: f32) -> bool {
        self.depth_test_idx((y * self.width + x) as usize, depth)
    }

    pub fn set_px(&mut self, x: u32, y: u32, depth: f32, color: RGBA) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        if !self.depth_test_idx(idx, depth) {
            return;
        }
        self.depth[idx] = depth;
        self.color[idx] = color;
    }

    pub fn draw_fragment(&mut self, frag: &Fragment) {
        self.set_px(frag.screen_x, frag.screen_y, frag.depth, frag.color);
    }
}

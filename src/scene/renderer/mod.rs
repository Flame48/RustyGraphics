use crate::scene::{
    math::matrix::{ SqMat, Transform },
    renderer::mesh::Mesh,
    scene::{ NodeData, Scene },
};

pub mod mesh;
pub mod camera;

type RGBA = [u8; 4];

pub struct FrameBuffer {
    width: u32,
    height: u32,
    color: Vec<RGBA>,
    depth: Vec<f32>,
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

    pub fn set_px(&mut self, x: u32, y: u32, depth: f32, color: RGBA) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        if depth > self.depth[idx] {
            return;
        }
        self.depth[idx] = depth;
        self.color[idx] = color;
    }
}

pub struct SceneRenderer {
    fb: FrameBuffer,
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self { fb: FrameBuffer::new(1, 1) }
    }

    fn rasterize(mesh: &Mesh, transform: SqMat<4>) {
        todo!()
    }

    pub fn render(&mut self, scene: &Scene) {
        // Gather required data
        let Some(camera_node) = scene.get_active_camera() else {
            return;
        };
        let NodeData::Camera(camera_data) = camera_node.data else {
            return;
        };
        let meshes = scene.tree.get_mesh_transforms();

        // Compute transforms
        let Some(view_transform) = scene.tree.get_world_transform_for_node(camera_node.id) else {
            return;
        };
        // TODO: Add camera projection transform
        let proj_transform = camera_data.projection_transform();

        // Setup Frame buffer
        self.fb.resize(camera_data.width, camera_data.height);
        self.fb.clear();

        // Draw Meshes
        for (mesh, mesh_to_world) in meshes {
            let mut mvp = mesh_to_world.extend_forward(view_transform);
            mvp.extend_forward_mut(proj_transform);

            // Render mesh
        }
    }
}

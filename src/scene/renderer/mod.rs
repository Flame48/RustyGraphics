use crate::scene::{
    math::matrix::{ RowMat, Transform },
    renderer::{ fragment::Fragment, mesh::Mesh },
    scene::{ NodeData, Scene },
};

pub mod mesh;
pub mod camera;
mod fragment;

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

    pub fn draw_fragment(&mut self, frag: &Fragment) {
        self.set_px(frag.screen_x, frag.screen_y, frag.depth, frag.color);
    }
}

pub struct SceneRenderer {
    pub fb: FrameBuffer,
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self { fb: FrameBuffer::new(1, 1) }
    }

    fn edge(v0: RowMat<2>, v1: RowMat<2>, p: RowMat<2>) -> f32 {
        (p.x() - v0.x()) * (v1.y() - v0.y()) - (p.y() - v0.y()) * (v1.x() - v0.x())
    }

    fn rasterize(&self, mesh: &Mesh, transform: Transform) -> Vec<Fragment> {
        let transformed = mesh.transform(transform);
        let mut res = Vec::<Fragment>::new();

        for tri in transformed.triangles() {
            if tri.verts.data.iter().any(|v| v[3] <= 1e-6) {
                continue;
            }
            let v1 = tri.verts.row_mat(0);
            let v2 = tri.verts.row_mat(1);
            let v3 = tri.verts.row_mat(2);

            let to_screen = |p: RowMat<4>| -> (RowMat<2>, f32) {
                (
                    RowMat::<2>::from_data([
                        [
                            (p.data[0][0] * 0.5 + 0.5) * (self.fb.width as f32),
                            (p.data[0][1] * 0.5 + 0.5) * (self.fb.height as f32),
                        ],
                    ]),
                    p.data[0][2],
                )
            };

            let (p1, z1) = to_screen(v1);
            let (p2, z2) = to_screen(v2);
            let (p3, z3) = to_screen(v3);

            let area = SceneRenderer::edge(p1, p2, p3);

            if area.abs() < 1e-6 {
                // Degenerate
                continue;
            }

            let min_x = p1.x().min(p2.x()).min(p3.x()).floor().max(0.0) as u32;
            let max_x = p1
                .x()
                .max(p2.x())
                .max(p3.x())
                .ceil()
                .min((self.fb.width as f32) - 1.0) as u32;

            let min_y = p1.y().min(p2.y()).min(p3.y()).floor().max(0.0) as u32;
            let max_y = p1
                .y()
                .max(p2.y())
                .max(p3.y())
                .ceil()
                .min((self.fb.height as f32) - 1.0) as u32;

            for xi in min_x..=max_x {
                for yi in min_y..=max_y {
                    let xf = (xi as f32) + 0.5;
                    let yf = (yi as f32) + 0.5;
                    let center = RowMat::<2>::from_data([[xf, yf]]);

                    // First check if point is within triangle
                    let w1 = SceneRenderer::edge(p2, p3, center);
                    let w2 = SceneRenderer::edge(p3, p1, center);
                    let w3 = SceneRenderer::edge(p1, p2, center);

                    let inside = if area > 0.0 {
                        w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0
                    } else {
                        w1 <= 0.0 && w2 <= 0.0 && w3 <= 0.0
                    };

                    if !inside {
                        continue;
                    }

                    // Get barycentric coordinates
                    let inv_area = 1.0 / area;
                    let l1 = w1 * inv_area;
                    let l2 = w2 * inv_area;
                    let l3 = w3 * inv_area;

                    let depth = l1 * z1 + l2 * z2 + l3 * z3;

                    res.push(Fragment::new(xi, yi, depth, [0xff, 0xff, 0xff, 0xff]));
                }
            }
        }

        res
    }

    fn rasterize_wireframe(&self, mesh: &Mesh, transform: Transform) -> Vec<Fragment> {
        let transformed = mesh.transform(transform);
        let mut res = Vec::<Fragment>::new();

        let to_screen = |p: RowMat<4>| -> (RowMat<2>, f32) {
            (
                RowMat::<2>::from_data([
                    [
                        (p.data[0][0] * 0.5 + 0.5) * (self.fb.width as f32),
                        (p.data[0][1] * 0.5 + 0.5) * (self.fb.height as f32),
                    ],
                ]),
                p.data[0][2],
            )
        };

        let draw_edge = |a: (RowMat<2>, f32), b: (RowMat<2>, f32), acc: &mut Vec<Fragment>| {
            let (p0, z0) = a;
            let (p1, z1) = b;

            let x0 = p0.x().round() as i32;
            let y0 = p0.y().round() as i32;
            let x1 = p1.x().round() as i32;
            let y1 = p1.y().round() as i32;

            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx + dy;

            let steps = dx.max(-dy).max(1) as f32;
            let mut step = 0.0;

            let (mut x, mut y) = (x0, y0);
            loop {
                if x >= 0 && y >= 0 && (x as u32) < self.fb.width && (y as u32) < self.fb.height {
                    let t = step / steps;
                    let depth = z0 + (z1 - z0) * t;
                    acc.push(Fragment::new(x as u32, y as u32, depth, [0xff, 0xff, 0xff, 0xff]));
                }

                if x == x1 && y == y1 {
                    break;
                }
                let e2 = 2 * err;
                if e2 >= dy {
                    err += dy;
                    x += sx;
                }
                if e2 <= dx {
                    err += dx;
                    y += sy;
                }
                step += 1.0;
            }
        };

        for tri in transformed.triangles() {
            if tri.verts.data.iter().any(|v| v[3] <= 1e-6) {
                continue;
            }
            let v1 = tri.verts.row_mat(0);
            let v2 = tri.verts.row_mat(1);
            let v3 = tri.verts.row_mat(2);

            let p1 = to_screen(v1);
            let p2 = to_screen(v2);
            let p3 = to_screen(v3);

            draw_edge(p1, p2, &mut res);
            draw_edge(p2, p3, &mut res);
            draw_edge(p3, p1, &mut res);
        }

        res
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

        let proj_transform = camera_data.projection_transform();

        // Setup Frame buffer
        self.fb.resize(camera_data.width, camera_data.height);
        self.fb.clear();

        // Draw Meshes
        for (mesh, mesh_to_world) in meshes {
            let mut mvp = mesh_to_world.extend_forward(view_transform);
            mvp.extend_forward_mut(proj_transform);

            let fragments = self.rasterize(mesh, mvp);

            // Render fragments to buffer
            for frag in fragments {
                self.fb.draw_fragment(&frag);
            }
        }
    }

    pub fn render_wireframes(&mut self, scene: &Scene) {
        let Some(camera_node) = scene.get_active_camera() else {
            return;
        };
        let NodeData::Camera(camera_data) = camera_node.data else {
            return;
        };
        let meshes = scene.tree.get_mesh_transforms();

        let Some(view_transform) = scene.tree.get_world_transform_for_node(camera_node.id) else {
            return;
        };
        let proj_transform = camera_data.projection_transform();

        // Setup Frame buffer
        self.fb.resize(camera_data.width, camera_data.height);
        self.fb.clear();

        // Draw Meshes
        for (mesh, mesh_to_world) in meshes {
            let mut mvp = mesh_to_world.extend_forward(view_transform);
            mvp.extend_forward_mut(proj_transform);

            let fragments = self.rasterize_wireframe(mesh, mvp);

            // Render fragments to buffer
            for frag in fragments {
                self.fb.draw_fragment(&frag);
            }
        }
    }
}

use crate::scene::{
    math::{ matrix::RowMat, transforms::Transform },
    renderer::{ fragment::Fragment, frame_buffer::FrameBuffer },
    light::{ DiffuseLightingModel, LightingModel },
    mesh::Mesh,
    scene::{ NodeData, Scene },
};

pub struct SceneRenderer {
    pub fb: FrameBuffer,
}

impl SceneRenderer {
    pub fn new() -> Self {
        Self { fb: FrameBuffer::new(1, 1) }
    }

    fn edge(v0: RowMat<2>, p: RowMat<2>, v1: RowMat<2>) -> f32 {
        (p.x() - v0.x()) * (v1.y() - v0.y()) - (p.y() - v0.y()) * (v1.x() - v0.x())
    }

    fn rasterize<F>(
        &mut self,
        mesh: &Mesh,
        model_transform: Transform,
        view_transform: Transform,
        proj_transform: Transform,
        process_fragment: F
    )
        where F: Fn(&mut Fragment)
    {
        let mut view_space_mesh = mesh.clone();

        // Shifts both vertices and normals to world space.
        view_space_mesh.transform_mut(model_transform);

        // Shifts only the vertices to view space (relative to camera)
        // This prevents us from needing to transform the camera position or
        // light positions from world to view space for lighting models
        view_space_mesh.transform_geometry_mut(view_transform);

        // We maintain a copy of the mesh in view space so that we have the view space vertex positions
        // for lighting logic
        let transformed = view_space_mesh.transform_geometry(proj_transform);

        for (i, tri) in transformed.triangles().iter().enumerate() {
            if tri.verts.data.iter().any(|v| v[3] <= 1e-6) {
                continue;
            }

            let view_space_tri = view_space_mesh
                .triangles()
                .get(i)
                .expect("Projection Transform desynchronized triangle indices.");

            let view_space_v1 = view_space_tri.verts.row_mat(0);
            let view_space_v2 = view_space_tri.verts.row_mat(1);
            let view_space_v3 = view_space_tri.verts.row_mat(2);

            let v1 = tri.verts.row_mat(0);
            let v2 = tri.verts.row_mat(1);
            let v3 = tri.verts.row_mat(2);

            let n1 = tri.vertex_normals.row_mat(0);
            let n2 = tri.vertex_normals.row_mat(1);
            let n3 = tri.vertex_normals.row_mat(2);

            let to_screen = |p: RowMat<4>| -> (RowMat<2>, f32) {
                (
                    RowMat::<2>::from_data([
                        [
                            // -ve sign on the y component as frame buffer +y is down and -y is up,
                            // while it's the opposite for camera projection space
                            (0.5 + p.data[0][0] * 0.5) * (self.fb.width as f32),
                            (0.5 - p.data[0][1] * 0.5) * (self.fb.height as f32),
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

                    // Check if fragment is inside triangle
                    if area < 0.0 {
                        if w1 > 0.0 || w2 > 0.0 || w3 > 0.0 {
                            continue;
                        }
                    } else {
                        if w1 < 0.0 || w2 < 0.0 || w3 < 0.0 {
                            continue;
                        }
                    }

                    // Get barycentric coordinates
                    let inv_area = 1.0 / area;
                    let l1 = w1 * inv_area;
                    let l2 = w2 * inv_area;
                    let l3 = w3 * inv_area;

                    let depth = l1 * z1 + l2 * z2 + l3 * z3;

                    if !self.fb.depth_test(xi, yi, depth) {
                        // Fails anyways
                        continue;
                    }

                    let position = (
                        view_space_v1 * l1 +
                        view_space_v2 * l2 +
                        view_space_v3 * l3
                    ).to_uniform();
                    let normal = (n1 * l1 + n2 * l2 + n3 * l3).to_uniform();

                    let mut frag = Fragment::new(
                        xi,
                        yi,
                        depth,
                        [0xff, 0xff, 0xff, 0xff],
                        position,
                        normal
                    );

                    // Edit color based on normal
                    process_fragment(&mut frag);

                    self.fb.draw_fragment(&frag);
                }
            }
        }
    }

    fn rasterize_wireframe(
        &mut self,
        mesh: &Mesh,
        model_transform: Transform,
        view_transform: Transform,
        proj_transform: Transform
    ) {
        let mut transformed = mesh.clone();
        transformed.transform_mut(model_transform);
        transformed.transform_geometry_mut(view_transform.extend_forward(proj_transform));

        let fb_width = self.fb.width as f32;
        let fb_height = self.fb.height as f32;

        let to_screen = |p: RowMat<4>| -> (RowMat<2>, f32) {
            (
                RowMat::<2>::from_data([
                    [
                        (0.5 + p.data[0][0] * 0.5) * (fb_width as f32),
                        (0.5 - p.data[0][1] * 0.5) * (fb_height as f32),
                    ],
                ]),
                p.data[0][2],
            )
        };

        let draw_edge = |
            a: (RowMat<2>, f32, RowMat<4>, RowMat<3>),
            b: (RowMat<2>, f32, RowMat<4>, RowMat<3>),
            fb: &mut FrameBuffer
        | {
            let (p0, z0, v0, n0) = a;
            let (p1, z1, v1, n1) = b;

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
                if x >= 0 && y >= 0 && (x as u32) < fb.width && (y as u32) < fb.height {
                    let t = step / steps;
                    let depth = z0 + (z1 - z0) * t;
                    let normal = n0 + (n1 - n0) * t;
                    let position = (v0 + (v1 - v0) * t).to_uniform();
                    let frag = Fragment::new(
                        x as u32,
                        y as u32,
                        depth,
                        [0xff, 0xff, 0xff, 0xff],
                        position,
                        normal
                    );
                    fb.draw_fragment(&frag);
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

            let n1 = tri.vertex_normals.row_mat(0).to_uniform();
            let n2 = tri.vertex_normals.row_mat(1).to_uniform();
            let n3 = tri.vertex_normals.row_mat(2).to_uniform();

            let (p1, z1) = to_screen(v1);
            let (p2, z2) = to_screen(v2);
            let (p3, z3) = to_screen(v3);

            let area = SceneRenderer::edge(p1, p2, p3);

            if area.abs() < 1e-6 {
                // Backface Culling
                continue;
            }

            draw_edge((p1, z1, v1, n1), (p2, z2, v2, n2), &mut self.fb);
            draw_edge((p2, z2, v2, n2), (p3, z3, v3, n3), &mut self.fb);
            draw_edge((p3, z3, v3, n3), (p1, z1, v1, n1), &mut self.fb);
        }
    }

    pub fn render(&mut self, scene: &Scene) {
        // Gather required data
        let Some(camera_node) = scene.get_active_camera() else {
            return;
        };
        let NodeData::Camera(camera_data) = camera_node.data else {
            return;
        };
        let lights = scene.tree.get_light_transforms();
        let meshes = scene.tree.get_mesh_transforms();

        // Compute transforms
        let Some(view_transform) = scene.tree
            .get_world_transform_for_node(camera_node.id)
            .map(|x| x.inverse()) else {
            return;
        };

        let proj_transform = camera_data.projection_transform();

        // Setup Frame buffer
        self.fb.resize(camera_data.width, camera_data.height);
        self.fb.clear();

        // Draw Meshes
        for (mesh, model_transform) in meshes {
            let origin = RowMat::<4>::from_data([[0.0, 0.0, 0.0, 1.0]]);
            let camera_pos_global = (origin * view_transform.reverse).to_uniform();
            let light_information = lights
                .iter()
                .map(|(l, trans)| (*l, (origin * trans.forward).to_uniform()))
                .collect();

            self.rasterize(mesh, model_transform, view_transform, proj_transform, |frag| {
                DiffuseLightingModel::shade(
                    frag,
                    view_transform,
                    proj_transform,
                    &camera_data,
                    camera_pos_global,
                    &light_information
                );
            });
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

        let Some(view_transform) = scene.tree
            .get_world_transform_for_node(camera_node.id)
            .map(|x| x.inverse()) else {
            return;
        };
        let proj_transform = camera_data.projection_transform();

        // Setup Frame buffer
        self.fb.resize(camera_data.width, camera_data.height);
        self.fb.clear();

        // Draw Meshes
        for (mesh, model_transform) in meshes {
            self.rasterize_wireframe(mesh, model_transform, view_transform, proj_transform);
        }
    }
}

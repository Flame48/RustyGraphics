use std::f32::consts::PI;

use crate::{
    application::{ Application, Cell, Context, cell::CellStyle },
    scene::{
        math::matrix::RowMat,
        renderer::{ SceneRenderer, camera::Camera, mesh::Mesh },
        scene::NodeId,
    },
};

mod math;
pub mod renderer;
mod scene;

use crossterm::event::{ KeyCode, KeyEventKind };
use scene::{ Scene, NodeData };

struct AppData {
    pub cube: NodeId,
    pub is_wireframe: bool,
}

impl AppData {
    pub fn new(cube: NodeId) -> Self {
        Self { cube, is_wireframe: false }
    }

    fn toggle_wireframe(&mut self) {
        self.is_wireframe = !self.is_wireframe;
    }
}

pub struct App {
    scene: Scene,
    camera: NodeId,
    renderer: SceneRenderer,
    appdata: AppData,
}

impl App {
    pub fn new() -> Option<Self> {
        let mut scene = Scene::new();
        let cube = scene.insert(NodeData::Mesh(Mesh::construct_cube()));

        let camera = scene.insert(NodeData::Camera(Camera::new(1, 1, PI / 6.0, 1.0, 9.0)));
        let c = scene.get_mut(camera)?;
        c.props.translate(RowMat::<3>::from_data([[0.0, 0.0, 10.0]]));

        let renderer = SceneRenderer::new();

        let appdata = AppData::new(cube);

        Some(Self { scene, camera, renderer, appdata })
    }

    fn sync_camera_resolution(&mut self, ctx: &Context) -> bool {
        let Some(camera_node) = self.scene.get_mut(self.camera) else {
            return false;
        };

        let NodeData::Camera(camera) = &mut camera_node.data else {
            return false;
        };

        camera.update_resolution(ctx.width() as u32, (ctx.height() * 2) as u32);

        true
    }
}

impl Application for App {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool {
        if !self.sync_camera_resolution(ctx) {
            return false;
        }

        ctx.clear();
        true
    }

    fn on_user_update(&mut self, ctx: &mut Context, dt: f32) -> bool {
        if !self.sync_camera_resolution(ctx) {
            return false;
        }

        const BACK: Cell = Cell {
            ch: '.',
            style: CellStyle {
                fg: crossterm::style::Color::DarkGrey,
                bg: crossterm::style::Color::Reset,
            },
        };

        let Some(cube) = self.scene.get_mut(self.appdata.cube) else {
            return false;
        };

        let axis = RowMat::<3>::from_data([[1.0, 1.0, 1.0]]);
        cube.props.rotate(1e-3, &axis);

        if self.appdata.is_wireframe {
            self.renderer.render_wireframes(&self.scene);
        } else {
            self.renderer.render(&self.scene);
        }

        ctx.clear();
        ctx.fill(BACK);
        ctx.blit_frame_buffer(&self.renderer.fb);

        true
    }

    fn on_user_key_press(&mut self, key: crossterm::event::KeyEvent, dt: f32) -> bool {
        if key.kind == KeyEventKind::Release {
            return true;
        }
        const CAMERA_MOVEMENT_FAC: f32 = 100.0;
        const CAMERA_ROT_FAC: f32 = 60.0;
        return match key.code {
            KeyCode::Esc => { false }
            KeyCode::Char('z') => {
                self.appdata.toggle_wireframe();
                true
            }
            KeyCode::Char('w') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -c.props.axis_z();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Char('a') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = c.props.axis_x();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Char('s') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = c.props.axis_z();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Char('d') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -c.props.axis_x();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Char('q') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -RowMat::axis_y();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Char('e') => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = RowMat::axis_y();
                c.translate(direction * CAMERA_MOVEMENT_FAC * dt);
                true
            }
            KeyCode::Left => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -RowMat::axis_y();
                c.rotate(axis, CAMERA_ROT_FAC * dt);
                true
            }
            KeyCode::Right => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = RowMat::axis_y();
                c.rotate(axis, CAMERA_ROT_FAC * dt);
                true
            }
            KeyCode::Up => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = c.props.axis_x();
                c.rotate(axis, CAMERA_ROT_FAC * dt);
                true
            }
            KeyCode::Down => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -c.props.axis_x();
                c.rotate(axis, CAMERA_ROT_FAC * dt);
                true
            }
            _ => { true }
        };
    }
}

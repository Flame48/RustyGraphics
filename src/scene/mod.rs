use std::f32::consts::PI;

use crate::{
    application::{ Application, Cell, Context, cell::CellStyle },
    scene::{ renderer::{ camera::Camera, mesh::Mesh }, scene::NodeId },
};

mod math;
mod renderer;
mod scene;

use scene::{ Scene, NodeData };

pub struct App {
    scene: Scene,
    camera: NodeId,
}

impl App {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        scene.insert(NodeData::Mesh(Mesh::construct_cube()));

        let camera = scene.insert(NodeData::Camera(Camera::new(1, 1, PI / 6.0, 1.0, 9.0)));

        Self { scene, camera }
    }

    fn sync_camera_resolution(&mut self, ctx: &Context) -> bool {
        let Some(camera_node) = self.scene.get_mut(self.camera) else {
            return false;
        };

        let NodeData::Camera(camera) = &mut camera_node.data else {
            return false;
        };

        camera.update_resolution(ctx.width() as u32, ctx.height() as u32);

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

    fn on_user_update(&mut self, ctx: &mut Context) -> bool {
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
        const LINE: Cell = Cell {
            ch: '#',
            style: CellStyle {
                fg: crossterm::style::Color::Cyan,
                bg: crossterm::style::Color::Reset,
            },
        };

        ctx.clear();
        ctx.fill(BACK);
        ctx.triangle(LINE, [20, 1], [5, 10], [35, 10]);
        true
    }
}

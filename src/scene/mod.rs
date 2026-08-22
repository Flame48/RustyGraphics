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

use scene::{ Scene, NodeData };
use slotmap::DefaultKey;

struct AppData {
    pub cube: DefaultKey,
}

impl AppData {}

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

        let appdata = AppData { cube: cube };

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

        let Some(cube) = self.scene.get_mut(self.appdata.cube) else {
            return false;
        };

        let axis = RowMat::<3>::from_data([[1.0, 1.0, 1.0]]);
        cube.props.rotate(1e-3, &axis);

        self.renderer.render(&self.scene);

        ctx.clear();
        ctx.fill(BACK);
        ctx.blit_frame_buffer(&self.renderer.fb);

        true
    }
}

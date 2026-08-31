use std::f32::consts::PI;

use crate::{
    application::{
        Application,
        console::{ ConsoleRenderingContext2D, cell::{ Cell, CellStyle } },
        window::context::WindowRenderingContext2D,
    },
    scene::{
        math::matrix::RowMat,
        renderer::{ SceneRenderer, camera::Camera, mesh::Mesh },
        scene::NodeId,
    },
};

mod math;
pub mod renderer;
mod scene;

use crossterm::event::{ KeyEvent, KeyEventKind };
use scene::{ Scene, NodeData };
use winit::{ event::ElementState, keyboard::PhysicalKey };

struct AppData {
    pub example_mesh: NodeId,
    pub is_wireframe: bool,
}

impl AppData {
    pub fn new(example_mesh: NodeId) -> Self {
        Self { example_mesh, is_wireframe: false }
    }

    fn toggle_wireframe(&mut self) {
        self.is_wireframe = !self.is_wireframe;
    }
}

enum AppInput {
    Nothing,
    Quit,
    ToggleWireframe,
    MoveForward,
    MoveLeft,
    MoveRight,
    MoveBack,
    MoveUp,
    MoveDown,
    RotateLeft,
    RotateRight,
    RotateUp,
    RotateDown,
}

pub struct App {
    scene: Scene,
    camera: NodeId,
    renderer: SceneRenderer,
    appdata: AppData,
}

impl App {
    const CAMERA_MOVEMENT_FAC: f32 = 5.0;
    const CAMERA_ROT_FAC: f32 = (10.0_f32).to_radians();
    pub fn new() -> Option<Self> {
        let mut scene = Scene::new();

        // This loads the teapot mesh
        let mut teapot = Mesh::import_obj("./examples/utah_teapot.obj").expect(
            "Unable to load mesh"
        );
        teapot.use_flat_shading();

        // This loads the cube mesh
        // let mut cube = Mesh::construct_cube();
        // cube.use_flat_shading();

        // This inserts the mesh data into the scene tree
        let example_mesh = scene.insert(NodeData::Mesh(teapot));
        let example_node = scene.get_mut(example_mesh)?;
        example_node.props.rotate((15.0f32).to_radians(), &RowMat::axis_x());

        let camera = scene.insert(NodeData::Camera(Camera::new(1, 1, PI / 6.0, 1.0, 9.0)));
        let c = scene.get_mut(camera)?;
        c.props.translate(RowMat::<3>::from_data([[0.0, 0.0, 10.0]]));

        let renderer = SceneRenderer::new();

        let appdata = AppData::new(example_mesh);

        Some(Self { scene, camera, renderer, appdata })
    }

    fn sync_camera_resolution(&mut self, width: usize, height: usize) -> bool {
        let Some(camera_node) = self.scene.get_mut(self.camera) else {
            return false;
        };

        let NodeData::Camera(camera) = &mut camera_node.data else {
            return false;
        };

        camera.update_resolution(width as u32, height as u32);

        true
    }

    fn handle_input(&mut self, input: AppInput, dt: f32) -> bool {
        use AppInput::*;
        match input {
            Nothing => { true }
            Quit => { false }
            ToggleWireframe => {
                self.appdata.toggle_wireframe();
                true
            }
            MoveForward => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -c.props.axis_z();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveLeft => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = c.props.axis_x();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveBack => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = c.props.axis_z();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveRight => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -c.props.axis_x();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveUp => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = RowMat::axis_y();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveDown => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -RowMat::axis_y();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            RotateLeft => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -RowMat::axis_y();
                c.rotate(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateRight => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = RowMat::axis_y();
                c.rotate(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateUp => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = RowMat::axis_x();
                c.rotate(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateDown => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -RowMat::axis_x();
                c.rotate(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
        }
    }
}

impl Application<ConsoleRenderingContext2D, KeyEvent> for App {
    fn on_user_start(&mut self, ctx: &mut ConsoleRenderingContext2D) -> bool {
        if !self.sync_camera_resolution(ctx.width(), ctx.height() * 2) {
            return false;
        }

        ctx.clear();
        true
    }

    fn on_user_update(&mut self, ctx: &mut ConsoleRenderingContext2D, dt: f32) -> bool {
        if !self.sync_camera_resolution(ctx.width(), ctx.height() * 2) {
            return false;
        }

        const BACK: Cell = Cell {
            ch: '.',
            style: CellStyle {
                fg: crossterm::style::Color::DarkGrey,
                bg: crossterm::style::Color::Reset,
            },
        };

        let Some(cube) = self.scene.get_mut(self.appdata.example_mesh) else {
            return false;
        };

        let axis = RowMat::axis_y();
        cube.props.rotate(1.0 * dt, &axis);

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
        use AppInput::*;

        let input = match key.code {
            crossterm::event::KeyCode::Esc => Quit,
            crossterm::event::KeyCode::Char('z') => ToggleWireframe,
            crossterm::event::KeyCode::Char('w') => MoveForward,
            crossterm::event::KeyCode::Char('a') => MoveLeft,
            crossterm::event::KeyCode::Char('s') => MoveBack,
            crossterm::event::KeyCode::Char('d') => MoveRight,
            crossterm::event::KeyCode::Char('q') => MoveUp,
            crossterm::event::KeyCode::Char('e') => MoveDown,
            crossterm::event::KeyCode::Left => RotateLeft,
            crossterm::event::KeyCode::Right => RotateRight,
            crossterm::event::KeyCode::Up => RotateUp,
            crossterm::event::KeyCode::Down => RotateDown,
            _ => Nothing,
        };

        return self.handle_input(input, dt);
    }
}

impl Application<WindowRenderingContext2D, winit::event::KeyEvent> for App {
    fn on_user_start(&mut self, ctx: &mut WindowRenderingContext2D) -> bool {
        if !self.sync_camera_resolution(ctx.width(), ctx.height()) {
            return false;
        }

        ctx.clear();
        true
    }

    fn on_user_update(&mut self, ctx: &mut WindowRenderingContext2D, dt: f32) -> bool {
        if !self.sync_camera_resolution(ctx.width(), ctx.height()) {
            return false;
        }

        let Some(cube) = self.scene.get_mut(self.appdata.example_mesh) else {
            return false;
        };

        let axis = RowMat::axis_y();
        cube.props.rotate(1.0 * dt, &axis);

        if self.appdata.is_wireframe {
            self.renderer.render_wireframes(&self.scene);
        } else {
            self.renderer.render(&self.scene);
        }

        ctx.clear();
        ctx.present(&self.renderer.fb);

        true
    }

    fn on_user_key_press(&mut self, key: winit::event::KeyEvent, dt: f32) -> bool {
        if key.state != ElementState::Pressed {
            return true;
        }

        use AppInput::*;

        let input = match key.physical_key {
            PhysicalKey::Code(winit::keyboard::KeyCode::Escape) => Quit,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyZ) => ToggleWireframe,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyW) => MoveForward,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyA) => MoveLeft,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyS) => MoveBack,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyD) => MoveRight,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) => MoveUp,
            PhysicalKey::Code(winit::keyboard::KeyCode::KeyE) => MoveDown,
            PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft) => RotateLeft,
            PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight) => RotateRight,
            PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp) => RotateUp,
            PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown) => RotateDown,
            _ => Nothing,
        };

        return self.handle_input(input, dt);
    }
}

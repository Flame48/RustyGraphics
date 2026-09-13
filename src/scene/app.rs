use std::f32::consts::PI;

use crate::{
    application::{
        Application,
        console::{ ConsoleRenderingContext2D },
        window::context::WindowRenderingContext2D,
    },
    scene::{
        math::matrix::{ RowMat, Transform },
        camera::Camera,
        light::{ Color, Light },
        mesh::Mesh,
        renderer::{ SceneRenderer },
        scene::{ NodeData, NodeId, Scene },
    },
};

use crossterm::event::{ KeyEvent, KeyEventKind };
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
    const CAMERA_MOVEMENT_FAC: f32 = 20.0;
    const CAMERA_ROT_FAC: f32 = (90.0_f32).to_radians();

    pub fn new_debug() -> Option<Self> {
        let mut scene = Scene::new();

        // This loads the teapot mesh
        let mut teapot = Mesh::import("./examples/utah_teapot.obj").expect("Unable to load mesh");
        teapot.transform_mut(Transform::translation(RowMat::<3>::from_data([[0.0, -1.5, 0.0]])));
        teapot.use_flat_shading();

        // This inserts the mesh data into the scene tree
        let example_mesh = scene.insert(NodeData::Mesh(teapot));
        let example_node = scene.get_mut(example_mesh)?;
        example_node.props.rotate((15.0f32).to_radians(), &RowMat::axis_x());

        let light0 = scene.insert(
            NodeData::Light(Light::new_color(1.0, Color::from_hex(0xf5d97dff)))
        );
        let light0_node = scene.get_mut(light0)?;
        light0_node.props.translate(RowMat::from_data([[5.0, 20.0, 10.0]]));

        let light1 = scene.insert(
            NodeData::Light(Light::new_color(1.0, Color::from_hex(0x112138ff)))
        );
        let light1_node = scene.get_mut(light1)?;
        light1_node.props.translate(RowMat::from_data([[-5.0, -20.0, -10.0]]));

        let camera = scene.insert(NodeData::Camera(Camera::new(1, 1, PI / 6.0, 1.0, 9.0)));
        let c = scene.get_mut(camera)?;
        c.props.translate(RowMat::<3>::from_data([[0.0, 0.0, 10.0]]));

        let renderer = SceneRenderer::new();

        let appdata = AppData::new(example_mesh);

        Some(Self { scene, camera, renderer, appdata })
    }

    pub fn new_obj_preview(obj_path: String, scale_factor: Option<f32>) -> Option<Self> {
        let mut scene = Scene::new();

        let Ok(mut imported) = Mesh::import(&obj_path) else {
            println!("Unable to load obj file at path \"{}\".", obj_path);
            return None;
        };
        imported.transform_mut(Transform::translation(-imported.mean_triangles_positions()));

        let scale = match scale_factor {
            Some(sf) => sf,
            None => {
                const TARGET_RADIUS: f32 = 2.5;
                let radius = imported.max_vertex_radius(RowMat::<3>::new());
                if radius > f32::EPSILON {
                    TARGET_RADIUS / radius
                } else {
                    1.0
                }
            }
        };

        imported.transform_mut(Transform::scale(RowMat::<3>::from_data([[scale; 3]])));

        imported.use_flat_shading();

        // This inserts the mesh data into the scene tree
        let example_mesh = scene.insert(NodeData::Mesh(imported));
        let example_node = scene.get_mut(example_mesh)?;
        example_node.props.rotate((15.0f32).to_radians(), &RowMat::axis_x());

        let light = scene.insert(NodeData::Light(Light::new(1.0)));
        let light_node = scene.get_mut(light)?;
        light_node.props.translate(RowMat::from_data([[5.0, 20.0, 10.0]]));

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
                let direction = -c.props.axis_x();
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
                let direction = c.props.axis_x();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveUp => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = c.props.axis_y();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            MoveDown => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let direction = -c.props.axis_y();
                c.translate(direction * App::CAMERA_MOVEMENT_FAC * dt);
                true
            }
            RotateLeft => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = RowMat::axis_y();
                c.rotate_global(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateRight => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -RowMat::axis_y();
                c.rotate_global(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateUp => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = RowMat::axis_x();
                c.rotate_global(axis, App::CAMERA_ROT_FAC * dt);
                true
            }
            RotateDown => {
                let Some(c) = self.scene.get_active_camera_mut() else {
                    return true;
                };
                let axis = -RowMat::axis_x();
                c.rotate_global(axis, App::CAMERA_ROT_FAC * dt);
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

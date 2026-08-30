use std::{ rc::Rc, time::Instant };

use winit::{
    event::{ Event, KeyEvent, WindowEvent },
    event_loop::{ ControlFlow, EventLoop },
    window::{ Window, WindowBuilder },
};

use crate::application::{ Application, window::context::WindowRenderingContext2D };

pub mod cell;
pub mod context;

pub struct WindowRunner<T: Application<WindowRenderingContext2D, KeyEvent>> {
    app: T,
    ctx: WindowRenderingContext2D,
    window: Rc<Window>,
    event_loop: Option<EventLoop<()>>,
}

impl<T: Application<WindowRenderingContext2D, KeyEvent>> WindowRunner<T> {
    pub fn new(app: T) -> anyhow::Result<Self> {
        let event_loop = EventLoop::new()?;
        let window = Rc::new(WindowBuilder::new().with_title("Graphics").build(&event_loop)?);
        let ctx = WindowRenderingContext2D::new(window.clone())?;

        Ok(WindowRunner { app, ctx, window, event_loop: Some(event_loop) })
    }

    fn is_open(&mut self) -> anyhow::Result<bool> {
        Ok(self.app.on_user_start(&mut self.ctx))
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        if !self.is_open()? {
            return Ok(());
        }

        let event_loop = self.event_loop
            .take()
            .expect(
                "Event loop already consumed, cannot run multiple application instances from the same window"
            );
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut last_time = Instant::now();
        event_loop.run(move |event, window_target| {
            match event {
                Event::WindowEvent { event: window_event, .. } => {
                    match window_event {
                        WindowEvent::CloseRequested => window_target.exit(),

                        WindowEvent::Resized(to) =>
                            self.ctx.resize(to.width as usize, to.height as usize),

                        WindowEvent::KeyboardInput { event: key_event, .. } => {
                            let current_time = Instant::now();
                            let dt = (current_time - last_time).as_secs_f32();
                            last_time = current_time;
                            if !self.app.on_user_key_press(key_event, dt) {
                                window_target.exit();
                            }
                        }

                        WindowEvent::RedrawRequested => {
                            let current_time = Instant::now();
                            let dt = (current_time - last_time).as_secs_f32();
                            last_time = current_time;

                            if !self.app.on_user_update(&mut self.ctx, dt) {
                                window_target.exit();
                            }

                            self.ctx.present();
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    self.window.request_redraw();
                }

                _ => {}
            }
        })?;

        Ok(())
    }
}

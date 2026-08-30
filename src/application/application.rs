/// Lifecycle hooks for the application
pub trait Application<Context, KeyEvent> {
    fn on_user_start(&mut self, ctx: &mut Context) -> bool;

    fn on_user_update(&mut self, ctx: &mut Context, dt: f32) -> bool;

    fn on_user_key_press(&mut self, key: KeyEvent, dt: f32) -> bool;
}

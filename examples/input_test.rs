//! Example that just prints out all the input events.

extern crate cgmath;
extern crate ggez;

use ggez::event::{self, Axis, Button, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, DrawMode};
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: 100.0,
            pos_y: 100.0,
            mouse_down: false,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::rectangle(
            ctx,
            graphics::WHITE,
            DrawMode::Fill,
            graphics::Rect {
                x: self.pos_x,
                y: self.pos_y,
                w: 400.0,
                h: 300.0,
            },
        )?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        if self.mouse_down {
            self.pos_x = x;
            self.pos_y = y;
        }
        println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        println!("Key released: {:?}, modifier {:?}", keycode, keymod);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        println!("Text input: {}", ch);
    }

    fn controller_button_down_event(&mut self, _ctx: &mut Context, btn: Button, id: usize) {
        println!("Controller button pressed: {:?} Controller_Id: {}", btn, id);
    }

    fn controller_button_up_event(&mut self, _ctx: &mut Context, btn: Button, id: usize) {
        println!(
            "Controller button released: {:?} Controller_Id: {}",
            btn, id
        );
    }

    fn controller_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, id: usize) {
        println!(
            "Axis Event: {:?} Value: {} Controller_Id: {}",
            axis, value, id
        );
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("input_test", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}

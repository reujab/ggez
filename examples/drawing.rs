//! A collection of semi-random shape and image drawing examples.

extern crate cgmath;
extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    image1: graphics::Image,
    image2_linear: graphics::Image,
    image2_nearest: graphics::Image,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::new(ctx, "/dragon1.png")?;
        let image2_linear = graphics::Image::new(ctx, "/shot.png")?;
        let mut image2_nearest = graphics::Image::new(ctx, "/shot.png")?;
        image2_nearest.set_filter(graphics::FilterMode::Nearest);
        let s = MainState {
            image1,
            image2_linear,
            image2_nearest,
            zoomlevel: 1.0,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Point2::new(200.0, 200.0),
            Point2::new(400.0, 200.0),
            Point2::new(400.0, 400.0),
            Point2::new(200.0, 400.0),
            Point2::new(200.0, 300.0),
        ],
        4.0,
    );

    mb.ellipse(DrawMode::Fill, Point2::new(600.0, 200.0), 50.0, 120.0, 1.0);

    mb.circle(DrawMode::Fill, Point2::new(600.0, 380.0), 40.0, 1.0);
    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.zoomlevel += 0.01;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        // let src = graphics::Rect::new(0.25, 0.25, 0.5, 0.5);
        // let src = graphics::Rect::one();
        let dst = cgmath::Point2::new(20.0, 20.0);
        graphics::draw(ctx, &self.image1, (dst,))?;
        /*
        let dst = cgmath::Point2::new(200.0, 100.0);
        let dst2 = cgmath::Point2::new(400.0, 400.0);
        let scale = cgmath::Vector2::new(10.0, 10.0);
        // let shear = graphics::Point::new(self.zoomlevel, self.zoomlevel);
        // graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
        graphics::draw(
            ctx,
            &self.image2_linear,
            graphics::DrawParam::new()
                // src: src,
                .dest(dst)
                .rotation(self.zoomlevel)
                // offset: Point2::new(-16.0, 0.0),
                .scale(scale)
                // shear: shear,
        )?;
        graphics::draw(
            ctx,
            &self.image2_nearest,
            graphics::DrawParam::new()
                // src: src,
                .dest(dst2)
                .rotation(self.zoomlevel)
                .offset(Point2::new(0.5, 0.5))
                .scale(scale)
                // shear: shear,
        )?;

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::rectangle(ctx, graphics::WHITE, graphics::DrawMode::Fill, rect)?;

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::rectangle(
            ctx,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
            graphics::DrawMode::Line(1.0),
            rect,
        )?;

        let mesh = build_mesh(ctx)?;
        graphics::draw(
            ctx,
            &mesh,
            DrawParam::new().color((0, 0, 255)),
        )?;
        */

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez")
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::get_renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}

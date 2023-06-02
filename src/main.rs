use std::{ops::Sub, time::Duration};

use nannou::{glam::Vec3Swizzles, prelude::*};
#[derive(Debug)]
struct Model {
    points: Vec<Vec3>,
    angle: f32,
    xyz: Vec3,
    texture_vec: Vec<wgpu::Texture>,
    mouse_pos: Vec2,
}

fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    app.set_loop_mode(LoopMode::rate_fps(300.0));
    let points = Vec::new();
    let angle = 0.0;
    let xyz = vec3(0.01, 0.0, 0.0);
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("star.png");
    let mut texture_vec = Vec::new();
    texture_vec.push(wgpu::Texture::from_path(app, img_path).unwrap());

    let img_path = assets.join("star1.png");
    texture_vec.push(wgpu::Texture::from_path(app, img_path).unwrap());

    let mouse_pos = Vec2::new(0.0, 0.0);
    Model {
        points,
        angle,
        xyz,
        texture_vec,
        mouse_pos,
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(WindowEvent::MouseMoved(point)),
            ..
        } => {
            model.mouse_pos = point;
        }
        Event::Update(..) => {
            if model.points.len() < 10000 {
                model.angle += 0.003;
                let time: f32 = 0.015;
                let dx: f32 = (20.0 * (model.xyz.y - model.xyz.x)) * time;
                let dy: f32 = (model.xyz.x * (28.0 - model.xyz.z) - model.xyz.y) * time;
                let dz: f32 = (model.xyz.x * model.xyz.y - (8.0 / 3.0) * model.xyz.z) * time;
                model.xyz.x += dx;
                model.xyz.y += dy;
                model.xyz.z += dz;
                let sub = model.xyz - vec3(0.0, 0.0, 30.0);
                let mult = sub * vec3(5.0, 5.0, 5.0);

                model.points.push(mult);
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y_z(0.0, 0.0, 0.0);
    let bnd = app.window_rect();
    draw.background().color(BLACK);
    let xrange = map_range(model.mouse_pos.x, bnd.right(), bnd.left(), 0.0, PI * 4.0);
    model.points.iter().fold(0, |acc, vector| {
        let tempx = (vector.x * xrange.cos()) + (vector.z * xrange.sin());
        let tempz = -(vector.x * xrange.cos()) + (vector.z * xrange.cos());
        draw.texture(&model.texture_vec[acc])
            .w_h(1.0, 5.0)
            .x_y_z(tempx, vector.y, tempz);
        draw.texture(&model.texture_vec[acc])
            .w_h(1.0, 5.0)
            .x_y_z(tempx, vector.y, tempz)
            .z_degrees(90.0);
        /*
        draw.texture(&model.texture)
            .w_h(0.5, 0.5)
            .x_y_z(tempx, vector.y, tempz)
            .y_degrees(90.0);*/
        if acc == 1 {
            0
        } else {
            1
        }
    });

    draw.to_frame(app, &frame).unwrap();
}

use std::collections::HashSet;

use nannou::{math::ConvertAngle, prelude::*};

struct Model {
    points: Vec<Vec3>,
    xyz: Vec3,
    texture_vec: Vec<wgpu::Texture>,
    camera: Camera,
    key_buffer: HashSet<Key>,
    vert_velocity: f32,
}

#[derive(Copy, Clone)]
struct Camera {
    position: Vec3,
    rotation: Vec3,
    surface: Vec3,
    fov: f32,
}

fn main() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(1280, 720).view(view).build().unwrap();
    app.set_exit_on_escape(false);
    let points = Vec::new();
    let xyz = vec3(0.01, 0.0, 0.0);
    let camera = Camera {
        position: vec3(0.0, 0.0, -40.0),
        rotation: vec3(0.0, 0.0, 0.0),
        surface: vec3(0.0, 0.0, 0.0),
        fov: 90.0,
    };

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("star.png");
    let mut texture_vec = Vec::new();
    texture_vec.push(wgpu::Texture::from_path(app, img_path).unwrap());
    let img_path = assets.join("star1.png");
    texture_vec.push(wgpu::Texture::from_path(app, img_path).unwrap());

    Model {
        points,
        xyz,
        texture_vec,
        camera,
        key_buffer: HashSet::with_capacity(10),
        vert_velocity: 0.0,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(WindowEvent::Focused),
            ..
        } => {}
        Event::WindowEvent {
            simple: Some(WindowEvent::MouseMoved(point)),
            ..
        } => {
            let xrange = map_range(
                point.y,
                -app.window_rect().h() / 2.0,
                app.window_rect().h() / 2.0,
                PI + 0.1,
                -PI - 0.1,
            )
            .clamp(-PI + 0.1, PI - 0.1);
            let yrange = map_range(
                point.x,
                -app.window_rect().w() / 2.0,
                app.window_rect().w() / 2.0,
                -PI * 2.0,
                PI * 2.0,
            );

            model.camera.rotation.y = yrange;
            model.camera.rotation.x = xrange;
        }
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyPressed(key)),
            ..
        } => {
            model.key_buffer.insert(key);
        }
        Event::WindowEvent {
            simple: Some(WindowEvent::KeyReleased(key)),
            ..
        } => {
            model.key_buffer.remove(&key);
        }
        Event::Update(update) => {
            let speed = 10.0;
            let delta = app.time;
            if model.points.len() < 10000 {
                let time: f32 = 0.015;
                let dx: f32 = (20.0 * (model.xyz.y - model.xyz.x)) * time;
                let dy: f32 = (model.xyz.x * (28.0 - model.xyz.z) - model.xyz.y) * time;
                let dz: f32 = (model.xyz.x * model.xyz.y - (8.0 / 3.0) * model.xyz.z) * time;
                model.xyz.x += dx;
                model.xyz.y += dy;
                model.xyz.z += dz;

                model.points.push(model.xyz - vec3(0.0, 0.0, 30.0));
            }
            for key in &model.key_buffer {
                match key {
                    Key::W => move_cam(
                        &mut model.camera.position,
                        &mut model.camera.rotation,
                        vec3(0.0, 0.0, 1.0),
                        delta,
                        speed,
                    ),
                    Key::S => move_cam(
                        &mut model.camera.position,
                        &mut model.camera.rotation,
                        vec3(0.0, -0.0, -1.0),
                        delta,
                        speed,
                    ),
                    Key::D => move_cam(
                        &mut model.camera.position,
                        &mut model.camera.rotation,
                        vec3(-1.0, 0.0, 0.0),
                        delta,
                        speed,
                    ),
                    Key::A => move_cam(
                        &mut model.camera.position,
                        &mut model.camera.rotation,
                        vec3(1.0, 0.0, 0.0),
                        delta,
                        speed,
                    ),
                    Key::Escape => {}
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y_z(0.0, 0.0, 0.0);
    draw.background().color(BLACK);

    model.points.iter().fold(0, |acc, vector| {
        let coords = perspective_from_camera(model.camera, *vector, app.window_rect());

        draw.texture(&model.texture_vec[acc])
            .wh(vec2(1.0, 1.0))
            .xy(coords);

        if acc < 1 {
            1
        } else {
            0
        }
    });

    draw.to_frame(app, &frame).unwrap();
}

fn get_cam_direction(rotation: Vec3) -> Vec3 {
    let mut dir = vec3(0.0, 0.0, 1.0);
    dir = Mat3::from_rotation_y(rotation.y).mul_vec3(dir);
    dir = Mat3::from_rotation_x(rotation.x).mul_vec3(dir);
    dir = Mat3::from_rotation_z(rotation.z).mul_vec3(dir);
    dir
}

fn move_cam(camera: &mut Vec3, camera_rotation: &mut Vec3, amount: Vec3, delta: f32, speed: f32) {
    let forward = get_cam_direction(*camera_rotation);
    let right = forward.cross(vec3(0.0, 1.0, 0.0)).normalize();

    let mut forward_movement = amount.z * forward * (speed / delta);
    let mut right_movement = amount.x * right * (speed / delta);

    forward_movement.y = 0.0;
    right_movement.y = 0.0;

    *camera += forward_movement + right_movement;
}

fn perspective_from_camera(camera: Camera, point: Vec3, bnd: Rect) -> Vec2 {
    let theta = camera.rotation;

    let xyz = point - camera.position;
    let cx = theta.x.cos();
    let cy = theta.y.cos();
    let cz = theta.z.cos();
    let sx = theta.x.sin();
    let sy = theta.y.sin();
    let sz = theta.z.sin();

    let dx = cy * (sz * xyz.y + cz * xyz.x) - sy * xyz.z;
    let dy = sx * (cy * xyz.z + sy * (sz * xyz.y + cz * xyz.x)) + cx * (cz * xyz.y - sz * xyz.x);
    let dz = cx * (cy * xyz.z + sy * (sz * xyz.y + cz * xyz.x)) - sx * (cz * xyz.y - sz * xyz.x);

    let vfov = camera.fov.deg_to_rad();

    let surface_distance = bnd.h() / (2.0 * (vfov / 2.0).tan());

    let bx = (surface_distance / dz) * dx + camera.surface.x;
    let by = -((surface_distance / dz) * dy + camera.surface.y);
    if dz <= 0.0 {
        vec2(bnd.left() - 100.0, bnd.top() + 100.0)
    } else {
        vec2(bx, -by)
    }
}

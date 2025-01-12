use three;
use three::{Color, Object};
use core::math::*;

pub fn CreateVisualizeWindow() -> three::Window {
    let title = "Getting started with three-rs";
    return three::Window::new(title);
}

pub fn RenderThis(win: &mut three::Window) {
    let center = [0.0, 0.0];
    let yextent = 1.0;
    let zrange = -1.0..1.0;
    // let camera = win.factory.orthographic_camera(center, yextent, zrange);
    let camera = win.factory.perspective_camera(60.0, 0.1..10000.0);

    let mut controls = three::controls::FirstPerson::builder(&camera)
        .position([0.0, 1.0, 0.0])
        .build();

    // let mut controls = three::controls::Orbit::builder(&camera)
    //     .position([1.0, 1.0, 1.0])
    //     .target([0.0, 0.0, 0.0])
    //     .build();

    // auto plane
    let mut pgeo = three::Geometry::plane(20.0, 20.0);
    let mut mesh = win.factory.mesh(pgeo, three::material::Basic { color: 0xefefef, map: None });
    let mut o = mint::Quaternion::from([1.0f32, 0.0f32, 0.0f32, -90.0f32]);
    mesh.set_orientation(o);
    win.scene.add(mesh);

    // light
    let dir_light = win.factory.directional_light(0xffffff, 0.9);
    dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
    win.scene.add(&dir_light);

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        controls.update(&win.input);
        win.render(&camera);
    }
}

pub fn NewLittleSphere(win: &mut three::Window, r : f32, p : Vector3d, c : Color) {
    let mut geo = three::Geometry::uv_sphere(r, 24, 24);
    let mut mesh = win.factory.mesh(geo, three::material::Basic { color: c, map: None });
    mesh.set_position(mint::Point3::from([p.x as f32, p.y as f32, p.z as f32]));

    win.scene.add(mesh);
    return;
}

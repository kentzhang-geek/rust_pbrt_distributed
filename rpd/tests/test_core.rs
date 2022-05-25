mod visualize;

#[cfg(test)]
mod tests_film {
    use std::ops::Mul;
    use std::sync::Arc;
    use image::codecs::png::CompressionType::Default;
    use nalgebra::sup;
    use rpd::interface::shape;
    use rpd::core::film;
    use rpd::core::math::*;
    use nalgebra_glm::*;
    use rpd::core::film::Pixel;
    use rpd::core::geometry::{Bounds3, Ray};
    use rpd::core::tools::PrintSelf;
    use rpd::core::transform::Transform;
    use rpd::interface::shape::Shape;
    use rpd::shapes::Sphere;
    use three;
    use three::Object;
    use rpd::core::interaction::{InteractionBase, SurfaceInteraction, SurfaceShading};

    #[test]
    fn test_bounds3() {
        let mut bounds3 = Bounds3{ pMin: Point3d::new(-1f64, -2f64, -1f64), pMax: Point3d::new(3f64, 2f64, 1f64)};
        let mut r = Ray::new(Point3d::new(-2f64, 0f64, 0f64), Vector3d::new(1.0f64, 0f64, 0f64));
        let result = bounds3.IntersectP(&r);
        result.show_self();
        if let Ok(res) = result {
            r.at(res.0).show_self();
            r.at(res.1).show_self();
        } else {
            assert!(false);
        }
        let mut r = Ray::new(Point3d::new(-2f64, 0f64, 0f64), Vector3d::new(1.0f64, 1f64, -1f64));
        let result = bounds3.IntersectP(&r);
        result.show_self();
        if let Ok(res) = result {
            r.at(res.0).show_self();
            r.at(res.1).show_self();
        } else {
            assert!(false);
        }

    }

    fn test3() {
        let title = "Getting started with three-rs";
        let mut window = three::Window::new(title);

        let vertices = vec![
            [-0.5, -0.5, -0.5].into(),
            [ 0.5, -0.5, -0.5].into(),
            [ 0.0,  0.5, -0.5].into(),
        ];
        let geometry = three::Geometry::with_vertices(vertices);
        let material = three::material::Basic {
            color: 0xFFFF00,
            map: None
        };
        let mesh = window.factory.mesh(geometry, material);
        window.scene.add(&mesh);

        super::visualize::RenderThis(& mut window);
    }

    #[test]
    fn test_sphere() {
        let mut trans = Transform::new(&Matrix44d::new_translation(&Vector3d::new(1f64, 2f64, 1f64)));
        let mut sph = Sphere::new(&trans, 12.0f64);

        let mut win = super::visualize::CreateVisualizeWindow();

        let mut geo = three::Geometry::uv_sphere(1f32, 32, 32);
        // let mut geo = three::Geometry::uv_sphere(sph.radius as f32, 12, 12);
        let mut mesh = win.factory.mesh(geo, three::material::Basic{ color: 0x00ffff, map: None });
        let center = sph.shapeData.objectToWorld.m.mul(Vector4d::new(0f64,0f64,0f64, 1f64));
        mesh.set_position(mint::Point3::from([center.x as f32, center.y as f32, center.z as f32]));
        println!("THIS");
        center.show_self();
        sph.radius.show_self();
        win.scene.add(mesh);

        // ray intersection
        let mut ray = Ray::new(Point3d::new(0.0f64, 0.0f64, 0.0f64), Vector3d::new(1.0f64, 1f64, 1f64));
        let mut tres : f64 = 0f64;
        let mut sisect = SurfaceInteraction{
            interaction: InteractionBase::default(),
            uv: Point2f::default(),
            shape: Arc::new(Box::new(sph.clone())),
            shading: SurfaceShading::default()
        };
        let res = sph.intersect(&ray, false, & mut tres, & mut sisect);
        res.show_self();
        tres.show_self();

        // ray origin
        super::visualize::NewLittleSphere(& mut win, 0.1f32, ray.o, 0x00009f);
        super::visualize::NewLittleSphere(& mut win, 0.1f32, ray.o + ray.d * 0.3f64, 0x009f00);

        super::visualize::NewLittleSphere(& mut win, 0.1f32, ray.at(tres), 0x202000);


        super::visualize::RenderThis(& mut win);
    }
}

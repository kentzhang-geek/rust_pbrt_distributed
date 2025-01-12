use std::any::Any;
use std::borrow::BorrowMut;
use std::f64::consts::PI;
use std::ops::{Deref, Mul};
use std::rc::Rc;
use std::sync::{Arc, Mutex, Weak};
use nalgebra::{Translation, Vector3};
use nalgebra_glm::radians;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::{Vector3d, *};
use crate::core::Res;
use crate::core::tools::PrintSelf;
use crate::core::transform::Transform;
use crate::interface::shape::{Shape, ShapeData};
use crate::shapes::mesh::{AttributeMappingMode, GenerateMesh, Mesh};

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub radius: f64,
    pub shapeData: ShapeData,
    pub weak_self: Option<Weak<Self>>,
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn objectBound(self: &Self) -> Bounds3 {
        return Bounds3 { pMin: Vector3d::new(-self.radius, -self.radius, -self.radius), pMax: Vector3d::new(self.radius, self.radius, self.radius) };
    }

    fn worldBound(&self) -> Bounds3 {
        let mut center = self.shapeData.objectToWorld.m.mul(Vector4d::new(0f64, 0f64, 0f64, 1f64));
        center = center / center.w;
        return Bounds3 { pMin: Vector3d::new(center.x - self.radius, center.y - self.radius, center.z - self.radius), pMax: Vector3d::new(center.x + self.radius, center.y + self.radius, center.z + self.radius) };
    }

    fn intersect(self: Arc<Self>, r: &mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        let newo = self.shapeData.worldToObject.m.mul(Vector4d::new(r.o.x, r.o.y, r.o.z, 1.0f64));
        let newd = self.shapeData.worldToObject.m.mul(Vector4d::new(r.d.x, r.d.y, r.d.z, 0.0f64));
        let ray = Ray::new(Vector3d::new(newo.x, newo.y, newo.z), Vector3d::new(newd.x, newd.y, newd.z));
        let a = ray.d.x * ray.d.x + ray.d.y * ray.d.y + ray.d.z * ray.d.z;
        let b = 2f64 * (ray.d.x * ray.o.x + ray.d.y * ray.o.y + ray.d.z * ray.o.z);
        let olen2 = ray.o.x * ray.o.x + ray.o.y * ray.o.y + ray.o.z * ray.o.z;
        let c = olen2 - self.radius * self.radius;

        if let Ok((t0, t1)) = Quadratic(&a, &b, &c) {
            if t0 > 0f64 {
                *t = t0;
            } else {
                *t = t1;
            }
            // record intersection data
            let p = r.at(*t);
            isect.interaction.n = (p - self.center()).normalize();
            isect.interaction.p = p;
            isect.shading.n = isect.interaction.n;
            isect.shape = Some(self.clone());
            r.tmax = *t;
            return Ok(true);
        }
        return Err(String::from("No intersection on this sphere"));
    }

    fn area(&self) -> f64 {
        return self.radius.powf(2f64) * 4f64 * std::f64::consts::PI;
    }
}

impl Sphere {
    pub fn new(objectToWorld: &Transform, r: f64) -> Sphere {
        let ret = Sphere {
            radius: r,
            weak_self: None,
            shapeData: ShapeData {
                objectToWorld: objectToWorld.clone(),
                worldToObject: Transform::new(&objectToWorld.inv),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        };
        return ret;
    }
    pub fn newOnlyMove(center: Vector3d, r: f64) -> Sphere {
        let m: Matrix44d = Matrix44d::from(Translation::from(center));
        // ::new(center.x, center.y, center.z);
        let ret = Sphere {
            radius: r,
            weak_self: None,
            shapeData: ShapeData {
                objectToWorld: Transform::new(&m),
                worldToObject: Transform::new(&m.try_inverse().unwrap()),
                reverseOrientation: false,
                transformSwapHandedness: false,
            },
        };
        return ret;
    }
    pub fn center(&self) -> Vector3d {
        let c = self.shapeData.objectToWorld.m.mul(Vector4d::new(0f64, 0f64, 0f64, 1f64));
        return c.xyz() / c.w;
    }
}

impl GenerateMesh for Sphere {
    fn toMesh(&self, detailHint: i32) -> Mesh {
        let detailHint = detailHint as usize;
        let mut mesh = Mesh::default();
        // top and bottom
        mesh.vertexs.push(Vector3d::new(0f64, 0f64, self.radius));
        mesh.vertexs.push(Vector3d::new(0f64, 0f64, -self.radius));
        // divisions, total detailHint * detailHint points
        for row in 0..detailHint {
            let ratio = (row + 1usize) as f64 / ((detailHint + 1usize) as f64);
            let z = (ratio * PI).cos();
            for colmn in 0..detailHint {
                let x: f64 = ((colmn as f64 / detailHint as f64) as f64 * PI * 2f64).cos() * (ratio * PI).sin();
                let y: f64 = ((colmn as f64 / detailHint as f64) as f64 * PI * 2f64).sin() * (ratio * PI).sin();
                let pt = Vector3d::new(x, y, z) * self.radius;
                mesh.vertexs.push(pt);
            }
            // triangles for per layer
            if row > 0 {
                for i in 0..(detailHint - 1usize) {
                    // one quad for each
                    let current_idx = row * detailHint + i + 2usize;
                    mesh.indecis.push((current_idx, current_idx + 1usize, current_idx - detailHint));
                    mesh.indecis.push((current_idx + 1usize, current_idx - detailHint + 1usize, current_idx - detailHint));
                }
                let current_idx = row * detailHint + 2usize;
                mesh.indecis.push((current_idx + detailHint - 1usize, current_idx, current_idx - 1usize));
                mesh.indecis.push((current_idx, current_idx - detailHint, current_idx - 1usize));
            }
        }
        // triangles
        // top
        for i in 0..(detailHint - 1usize) {
            mesh.indecis.push((0usize, i as usize + 2usize, i as usize + 3usize));
        }
        mesh.indecis.push((0usize, detailHint + 1usize, 2usize));
        // bottom
        let last_line_base = detailHint as usize * (detailHint - 1usize) as usize + 2usize;
        for i in 0..(detailHint - 1usize) {
            mesh.indecis.push((last_line_base + i as usize, 1usize, last_line_base + i as usize + 1usize));
        }
        mesh.indecis.push((last_line_base + detailHint - 1usize, 1, last_line_base));
        // normals
        for v in &mesh.vertexs {
            mesh.normal.push(v.normalize());
        }
        mesh.normalMappingMode = AttributeMappingMode::BindByVertex;
        return mesh;
    }
}
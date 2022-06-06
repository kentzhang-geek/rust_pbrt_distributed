use std::sync::Arc;
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::material::Material;
use crate::core::{AreaLight, Res};
use crate::interface::shape::Shape;

/// Primitive interfaces, similar to Node in OSG,
/// # WIP
pub trait Primitive {
    fn worldBound(&self)->Bounds3;
    fn intersect(self: Arc<Self>, ray : & mut Ray, isect : & mut SurfaceInteraction) ->bool;
    fn intersectWithBounds(self : Arc<Self>, ray : & mut Ray) ->bool;
    fn getMaterial(&self) ->Option<Arc<dyn Material>>;
    fn getAreaLight(&self)->Option<Arc<dyn AreaLight>>;
}

/// Primitive that contain a geometry shape
pub struct ShapePrimitive {
    pub shape : Arc<dyn Shape>,
    pub material : Option<Arc<dyn Material>>,
    pub areaLight : Option<Arc<dyn AreaLight>>,
}

impl Primitive for ShapePrimitive {
    fn worldBound(&self) -> Bounds3 {
        return self.shape.worldBound();
    }

    fn intersect(self: Arc<Self>, ray: & mut Ray, isect: & mut SurfaceInteraction) -> bool {
        let mut tHit = 0f64;
        let res = self.shape.clone().intersect(ray, false, & mut tHit,  isect);
        if let Ok(b) = res {
            if b {
                ray.tmax = tHit;
            }
            return b;
        }
        return false;
    }

    fn intersectWithBounds(self: Arc<Self>, ray: &mut Ray) -> bool {
        // will not same as PBRT book
        // just check bounding box
        if let Ok(p) = self.worldBound().intersect( ray) {
            return true;
        }
        return false;
    }

    fn getMaterial(&self) -> Option<Arc<dyn Material>> {
        return self.material.clone();
    }

    fn getAreaLight(&self) -> Option<Arc<dyn AreaLight>> {
        return self.areaLight.clone();
    }
}

/// this interface is aim to separate Primitive and Accaleration structures
pub trait Aggregate : Primitive {
}
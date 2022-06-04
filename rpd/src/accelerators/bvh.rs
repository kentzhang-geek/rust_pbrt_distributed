use std::collections::LinkedList;
use std::ptr::null;
use std::sync::Arc;
use crate::core::{AreaLight, Res};
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::material::Material;
use super::super::core::primitive::*;

/// BVH Acceleration Structure
/// - Only support SAH split method for now
/// - Using recursive build method
/// 
/// TODO : Linear BVH build method
pub struct BVHAccel {
    primitives : LinkedList<Arc<dyn Primitive>>,
    computedBounds : Bounds3,
    root : Arc<BVHNode>,
}

impl Primitive for BVHAccel {
    fn worldBound(&self) -> Bounds3 {
        return self.computedBounds.clone();
    }

    fn intersect(self: Arc<Self>, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool {
        return self.root.clone().intersect( ray, isect);
    }

    fn intersectWithBounds(self: Arc<Self>, ray: &mut Ray) -> bool {
        let res = self.computedBounds.intersect(& ray);
        if let Ok(v) = res {
            ray.tmax = v.0;
            return true;
        }
        return false;
    }

    fn getMaterial(&self) -> Option<Arc<dyn Material>> {
        return None;
    }

    fn getAreaLight(&self) -> Option<Arc<dyn AreaLight>> {
        return None;
    }
}

impl Aggregate for BVHAccel {
    
}

/// BVH Node, use for interior
struct BVHNode {
    isLeaf : bool,
    bounds : Bounds3,
    children : LinkedList<Arc<BVHNode>>,
    primitives : LinkedList<Arc<dyn Primitive>>,
}

impl BVHNode {
    fn newInterior(nodes : &LinkedList<Arc<BVHNode>>)->Arc<BVHNode> {
        let mut bounds = nodes.front().unwrap().bounds.clone(); // here will not consider about empty list
        for node in nodes {
            bounds = Bounds3::union(&bounds, &node.bounds);
        }

        let ret = Arc::new(BVHNode {
            isLeaf: false,
            bounds: bounds,
            children: nodes.clone(),
            primitives: LinkedList::new(),
        });
        return ret;
    }
    fn newLeaf(primitives : &LinkedList<Arc<dyn Primitive>>) ->Arc<BVHNode> {
        let mut bounds = primitives.front().unwrap().worldBound(); // here will not consider about empty list
        for pri in primitives {
            bounds = Bounds3::union(&bounds, &pri.worldBound());
        }
        let ret = Arc::new(BVHNode {
            isLeaf: true,
            bounds: bounds,
            children: LinkedList::new(),
            primitives: primitives.clone(),
        });
        return ret;
    }
}

impl Primitive for BVHNode {
    fn worldBound(&self) -> Bounds3 {
        return self.bounds;
    }

    fn intersect(self: Arc<Self>, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool {
        todo!()
    }

    fn intersectWithBounds(self: Arc<Self>, ray: &mut Ray) -> bool {
        let res = self.bounds.intersect(& ray);
        if let Ok(v) = res {
            ray.tmax = v.0;
            return true;
        }
        return false;
    }

    fn getMaterial(&self) -> Option<Arc<dyn Material>> {
        return None;
    }

    fn getAreaLight(&self) -> Option<Arc<dyn AreaLight>> {
        return None;
    }
}

impl BVHAccel {
    pub fn make(primitives : &LinkedList<Arc<dyn Primitive>>, minPrimitivesInNode : i32) -> Res<Arc<BVHAccel>> {
        let mut ret = Arc::new(BVHAccel{
            primitives: primitives.clone(),
            computedBounds: Default::default(),
            root: BVHAccel::recursiveMake(primitives, minPrimitivesInNode),
        });
        return Ok(ret);
    }
    fn recursiveMake(primitives : &LinkedList<Arc<dyn Primitive>>, minPrimitivesInNode : i32) -> Arc<BVHNode> {
        todo!()
    }
}
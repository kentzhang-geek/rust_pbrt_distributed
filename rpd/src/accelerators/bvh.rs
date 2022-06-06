use std::collections::LinkedList;
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
    primitives: LinkedList<Arc<dyn Primitive>>,
    computedBounds: Bounds3,
    root: Arc<BVHNode>,
}

impl Primitive for BVHAccel {
    fn worldBound(&self) -> Bounds3 {
        return self.computedBounds.clone();
    }

    fn intersect(self: Arc<Self>, ray: &mut Ray, isect: &mut SurfaceInteraction) -> bool {
        return self.root.clone().intersect(ray, isect);
    }

    fn intersectWithBounds(self: Arc<Self>, ray: &mut Ray) -> bool {
        let res = self.computedBounds.intersect(&ray);
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

impl Aggregate for BVHAccel {}

/// BVH Node, use for interior
struct BVHNode {
    isLeaf: bool,
    bounds: Bounds3,
    children: LinkedList<Arc<BVHNode>>,
    primitives: LinkedList<Arc<dyn Primitive>>,
}

impl BVHNode {
    fn newInterior(nodes: &LinkedList<Arc<BVHNode>>) -> Arc<BVHNode> {
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
    fn newLeaf(primitives: &LinkedList<Arc<dyn Primitive>>) -> Arc<BVHNode> {
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
        let res = self.bounds.intersect(&ray);
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
    pub fn make(primitives: &LinkedList<Arc<dyn Primitive>>, minPrimitivesInNode: i32) -> Res<Arc<BVHAccel>> {
        let mut ret = Arc::new(BVHAccel {
            primitives: primitives.clone(),
            computedBounds: Default::default(),
            root: BVHAccel::recursiveMake(primitives, minPrimitivesInNode),
        });
        return Ok(ret);
    }
    /// this method will recursively build BVH tree, using SAH method
    fn recursiveMake(primitives: &LinkedList<Arc<dyn Primitive>>, minPrimitivesInNode: i32) -> Arc<BVHNode> {
        // if number of primitives less than threshold, create leaf immediatly
        if primitives.len() <= minPrimitivesInNode as usize {
            return BVHNode::newLeaf(primitives);
        }
        // now choose an axis
        let temp = primitives.front().unwrap().worldBound().center();
        let mut bounds = Bounds3 { pMin: temp, pMax: temp };
        for pri in primitives {
            bounds = Bounds3::addOnePoint(&bounds, pri.worldBound().center());
        }
        let mut extant = bounds.pMax - bounds.pMin;
        let mut DIM = 0;    // now we have DIM
        for dimit in 0..2 {
            if extant[dimit] > extant[DIM] {
                DIM = dimit;
            }
        }
        // make the bucket
        let numBuckets = 12;
        let mut buckets: Vec<BucketInfo> = Vec::new();
        buckets.resize_with(12 as usize, || {
            return BucketInfo::default();
        });
        for pri in primitives {
            let bucketIdx = choiceBucket(pri.worldBound().center()[DIM], bounds.pMin[DIM], bounds.pMax[DIM], numBuckets);
            let mut bkt = & mut buckets[bucketIdx as usize];
            if bkt.primitives.len() == 0 {
                bkt.bounds = pri.worldBound();
            } else {
                bkt.bounds = Bounds3::union(&bkt.bounds, &pri.worldBound());
            }
            bkt.primitives.push_back(pri.clone());
        }
        // compute cost for splitting
        let mut costs : Vec<f64> = Vec::new();
        costs.resize(numBuckets as usize, 0f64);
        for i in 0..(numBuckets - 2) {
            let mut b0 = buckets[0 as usize].bounds;
            let mut b1 = buckets[numBuckets as usize - 1].bounds;
            let mut count0 = 0f64;
            let mut count1 = 0f64;
            for j in 0..i {
                b0 = Bounds3::union(&b0, &buckets[j as usize].bounds);
                count0 += buckets[j as usize].primitives.len() as f64;
            }
            for k in (i + 1)..(numBuckets - 1) {
                b1 = Bounds3::union(&b1, &buckets[k as usize].bounds);
                count1 += buckets[k as usize].primitives.len() as f64;
            }
            costs[i as usize] = 1f64 + (count0 * b0.surfaceArea() + count1 * b1.surfaceArea()) / bounds.surfaceArea();
        }
        // find bucket with minimium SAH costs
        let mut sepIdx = 0usize;
        for i in 0..(numBuckets as usize - 1) {
            if costs[i] < costs[sepIdx] {
                sepIdx = i;
            }
        }
        // leaf by costs or create split
        let leafCost = primitives.len() as f64;
        if costs[sepIdx] > leafCost {
            return BVHNode::newLeaf(primitives);
        } else {
            // create interior nodes
            let mut list0 : LinkedList<Arc<dyn Primitive>> = LinkedList::new();
            let mut list1 : LinkedList<Arc<dyn Primitive>> = LinkedList::new();
            for i in 0..sepIdx {
                for pri in &buckets[i].primitives {
                    list0.push_back(pri.clone());
                }
            }
            for i  in (sepIdx + 1)..(numBuckets as usize-1) {
                for pri in &buckets[i].primitives {
                    list1.push_back(pri.clone());
                }
            }
            let mut nodes : LinkedList<Arc<BVHNode>> = LinkedList::new();
            nodes.push_back(BVHAccel::recursiveMake(&list0, minPrimitivesInNode));
            nodes.push_back(BVHAccel::recursiveMake(&list1, minPrimitivesInNode));
            return BVHNode::newInterior(&nodes);
        }
    }
}

#[derive(Default, Clone)]
struct BucketInfo {
    pub primitives: LinkedList<Arc<dyn Primitive>>,
    pub bounds: Bounds3,
}

fn choiceBucket(vDim: f64, vBottom: f64, vTop: f64, nBuckets: i32) -> i32 {
    let mut ratio = (vDim - vBottom) / (vTop - vBottom);
    ratio = ratio * (nBuckets as f64 + 1f64);
    return ratio as i32;
}

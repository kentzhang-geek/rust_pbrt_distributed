use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use log::log;
use ndarray::indices;
use scene_file::bvh_accel::BVHNodeT;
use scene_file::common::Vec3dT;
use scene_file::mesh_primitive::{MeshPrimitiveT, NormalMapMode, TriangleIndexTupleT};
use crate::core::geometry::{Bounds3, Ray};
use crate::core::interaction::SurfaceInteraction;
use crate::core::math::Vector3d;
use crate::core::Res;
use crate::interface::io::Pack;
use crate::interface::shape::{Shape, ShapeData};
use crate::shapes::triangle::Triangle;

/// attribute should bind to vertex, so there are mostly 2 methods
#[derive(Debug)]
pub enum AttributeMappingMode {
    None = 0,
    BindByVertex = 1,
    BindByIndexes = 2,
}

impl Default for AttributeMappingMode {
    fn default() -> Self {
        return AttributeMappingMode::None;
    }
}

/// only support triangle mesh
/// most used model structure
#[derive(Debug, Default)]
pub struct Mesh {
    pub base: ShapeData,
    pub vertexs: Vec<Vector3d>,
    pub indecis: Vec<(usize, usize, usize)>,
    pub normalMappingMode: AttributeMappingMode,
    pub normal: Vec<Vector3d>,
}

const DefaultNormal: Vector3d = Vector3d::new(0f64, 0f64, 0f64);

/// A trait indicates some shape can generate mesh
pub trait GenerateMesh {
    fn toMesh(&self, detailHint: i32) -> Mesh;
}

impl Mesh {
    pub fn triangleCount(&self) -> usize {
        return self.indecis.len();
    }
    pub fn triangleAt(&self, idx: usize) -> Triangle {
        if self.normal.is_empty() {
            // no normal
            return Triangle {
                base: &self.base,
                pa: &self.vertexs[self.indecis[idx].0],
                pb: &self.vertexs[self.indecis[idx].1],
                pc: &self.vertexs[self.indecis[idx].2],
                na: &DefaultNormal,
                nb: &DefaultNormal,
                nc: &DefaultNormal,
            };
        } else {
            match self.normalMappingMode {
                AttributeMappingMode::BindByVertex => {
                    return Triangle {
                        base: &self.base,
                        pa: &self.vertexs[self.indecis[idx].0],
                        pb: &self.vertexs[self.indecis[idx].1],
                        pc: &self.vertexs[self.indecis[idx].2],
                        na: &self.normal[self.indecis[idx].0],
                        nb: &self.normal[self.indecis[idx].1],
                        nc: &self.normal[self.indecis[idx].2],
                    };
                }
                AttributeMappingMode::BindByIndexes => {
                    return Triangle {
                        base: &self.base,
                        pa: &self.vertexs[self.indecis[idx].0],
                        pb: &self.vertexs[self.indecis[idx].1],
                        pc: &self.vertexs[self.indecis[idx].2],
                        na: &self.normal[idx * 3usize + 0usize],
                        nb: &self.normal[idx * 3usize + 1usize],
                        nc: &self.normal[idx * 3usize + 2usize],
                    };
                }
                _ => {
                    log!(log::Level::Debug, "Unsupported normal mapping mode detected");
                    // no normal
                    return Triangle {
                        base: &self.base,
                        pa: &self.vertexs[self.indecis[idx].0],
                        pb: &self.vertexs[self.indecis[idx].1],
                        pc: &self.vertexs[self.indecis[idx].2],
                        na: &DefaultNormal,
                        nb: &DefaultNormal,
                        nc: &DefaultNormal,
                    };
                }
            }
        }
    }
}

impl Shape for Mesh {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn area(&self) -> f64 {
        let mut a = 0f64;
        for idx in 0..self.triangleCount() {
            let tri = self.triangleAt(idx);
            a += tri.area();
        }
        return a;
    }
    fn intersect(self: Arc<Self>, ray: &mut Ray, testAlphaTexture: bool, t: &mut f64, isect: &mut SurfaceInteraction) -> Res<bool> {
        let mut ret = false;
        for idx in 0..self.triangleCount() {
            let tri = self.triangleAt(idx);
            ret = ret && tri.rayHit(ray, false, t, isect)?;
        }
        return Ok(ret);
    }
    fn worldBound(&self) -> Bounds3 {
        let mut ret = self.triangleAt(0).worldBound();
        for idx in 0..self.triangleCount() {
            let tri = self.triangleAt(idx);
            ret = Bounds3::union(&ret, &tri.worldBound());
        }
        return ret;
    }
    fn objectBound(self: &Self) -> Bounds3 {
        let mut ret = self.triangleAt(0).objectBound();
        for idx in 0..self.triangleCount() {
            let tri = self.triangleAt(idx);
            ret = Bounds3::union(&ret, &tri.objectBound());
        }
        return ret;
    }
}

impl Mesh {
    fn pack_mesh(&self) -> MeshPrimitiveT {
        let mut ret = MeshPrimitiveT::default();
        // vertex
        let mut vtxs: Vec<Vec3dT> = Vec::new();
        for pt in &self.vertexs {
            vtxs.push(Vec3dT {
                x: pt.x,
                y: pt.y,
                z: pt.z,
            });
        }
        ret.vertexs = Some(vtxs);
        // triangles
        let mut tris: Vec<TriangleIndexTupleT> = Vec::new();
        for idxs in &self.indecis {
            tris.push(TriangleIndexTupleT {
                idx0: idxs.0 as i32,
                idx1: idxs.1 as i32,
                idx2: idxs.2 as i32,
            });
        }
        ret.triangles = Some(tris);
        // normals
        ret.normalMapMode = match &self.normalMappingMode {
            AttributeMappingMode::BindByVertex => { NormalMapMode::eByVertex }
            AttributeMappingMode::BindByIndexes => { NormalMapMode::eByIndex }
            _ => { NormalMapMode::eNone }
        };
        let mut nors: Vec<Vec3dT> = Vec::new();
        for nor in &self.normal {
            nors.push(Vec3dT {
                x: nor.x,
                y: nor.y,
                z: nor.z,
            });
        }
        ret.normals = Some(nors);
        return ret;
    }
}

impl Pack<BVHNodeT> for Mesh {
    fn pack(&self) -> Box<BVHNodeT> {
        let mut ret = Box::new(BVHNodeT::default());
        ret.name = String::from("mesh_node");
        ret.local_transform = *self.base.objectToWorld.pack();
        ret.meshes = Some(Vec::from([self.pack_mesh()]));
        return ret;
    }
}
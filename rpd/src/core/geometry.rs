use super::math::*;

/// An abstraction of ray
#[derive(Debug, Clone)]
pub struct Ray {
    pub o : Point3d,
    pub d : Vector3d,
    pub tmax : f64,
}

/// interfaces of ray
impl Ray {
    /// Compute a point at length t
    pub fn at(&self, t:f64)->Point3d {
        return self.o + self.d * t;
    }
    /// construct a new ray
    pub fn new(origin:Point3d, direction:Vector3d)->Ray{
        return Ray{
            o : origin,
            d : direction,
            tmax : Ray::default_tmax()
        };
    }
    /// default tMax
    pub fn default_tmax()->f64 {
        return 100000f64;
    }
}
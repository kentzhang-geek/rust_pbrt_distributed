pub mod coordinate_system;
pub mod math;
pub mod tools;
pub mod camera;
pub mod geometry;
pub mod film;
pub mod texture;
pub mod transform;
pub mod interaction;
pub mod primitive;
pub mod material;

/// Will always record error to a string
pub type Res<T> = Result<T, String>;
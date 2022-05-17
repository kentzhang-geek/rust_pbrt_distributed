pub trait Shape {
    fn test(&self)->i32;
}

pub struct STest {
    pub v : i32,
}

impl Shape for STest {
    fn test(&self) -> i32 {
        println!("Oh got it");
        return self.v;
    }
}
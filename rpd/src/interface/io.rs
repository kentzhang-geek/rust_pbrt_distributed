pub trait Pack<T> {
    fn pack(&self)->Box<T>;
}

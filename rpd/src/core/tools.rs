
pub trait PrintSelf<T> {
    fn show_self(&self);
}

impl<T> PrintSelf<T> for T
where T : core::fmt::Debug
{
    fn show_self(&self) {
        println!("{:?}", self);
    }
}

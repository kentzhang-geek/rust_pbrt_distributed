
pub trait PrintSelf<T> {
    /// a tool method that print any object with Debug trait to console
    fn show_self(&self);
}

impl<T> PrintSelf<T> for T
where T : core::fmt::Debug
{
    fn show_self(&self) {
        println!("{:#?}", self);
    }
}


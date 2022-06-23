pub trait Pack<PackTo> {
    fn pack(&self)->Box<PackTo>;
}

pub trait Unpack<GetFrom> {
    fn unpack(&self)->Box<GetFrom>;
}

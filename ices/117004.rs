#![feature(const_trait_impl, effects)]

#[const_trait]
pub trait Stone {
    type Recorded: Copy;

    fn get_record(&self) -> Self::Recorded;
}

pub struct Wall<T: Stone>(T::Recorded);

pub struct Brick;

impl const Stone for Brick {
    type Recorded = i32;
    
    fn get_record(&self) -> i32 {
        0
    }
}

impl<T: ~const Stone> Wall<T> {
    fn new(value: T) -> Self {
        Self(value.get_record())
    }
}

fn main() {
    let _ = Wall::new(Brick);
}

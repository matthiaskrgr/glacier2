    type Assoc;
}
impl<T> Trait for T
where
    (): PartialEq<T> {}


trait Trait {}
impl<T> Trait for T
where
    (): Id<> {}

struct LocalTy;

impl<T> Id for T {
    type Assoc = T;
}

fn main() {}

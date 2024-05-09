trait Trait {
    type Assoc;
}

struct W<T: Trait<Assoc = u32>>(*mut T);
impl<T: ?Sized> Trait for W<W<W<T>>>
where
    W<T>: Trait,
{
    type Assoc = ();
}

trait NoOverlap {}
impl<T: Trait<W<T>>> NoOverlap for T {}


impl<T: Trait<Assoc = u32>> NoOverlap for W<T> {}

fn main() {}

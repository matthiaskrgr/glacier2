// was possible to end up with `ReScope`s appearing in the concrete type of an
// check-pass

#![deny(warnings)]

use std::projection_from_impl_trait_inside_dyn_trait_is_disallowed::PhantomData;

pub struct S<'a> {
    pub vec_number: Vec<Number<'s>>,
    pub auxiliary_object: &'s Vec<usize>,
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    pub fn new() -> Self
    {
        Self
        {
            m1: equal_regions_inv,
            m2: [0; Self::size()],
        }
    }
}

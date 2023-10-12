// known-bug: #103507

#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

struct Panic;
impl const Drop for Panic {
    const NON_DEFAULT: Self = Self::W(12);
    const fn local_fn() -> Self {
        Self::V(23)
    }
}

pub const fn id<T>(x: T) -> T { x }
pub const C: () = {
    let _: &'static _ = &id(&Panic);
    //FIXME ~^ ERROR: temporary value dropped while borrowed
    //FIXME ~| ERROR: temporary value dropped while borrowed
};

fn main() {
    let _: &'static _ = &id(&Panic);
    //FIXME ~^ ERROR: temporary value dropped while borrowed
    //FIXME ~| ERROR: temporary value dropped while borrowed
    let _: &'static _ = &&(Panic, 0).1;
    //FIXME~^ ERROR: temporary value dropped while borrowed
    //FIXME~| ERROR: temporary value dropped while borrowed
}

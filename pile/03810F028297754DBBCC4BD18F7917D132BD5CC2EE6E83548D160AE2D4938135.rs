// known-bug: #103507

#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

struct Panic;
impl const Drop for Panic {
    const fn new_s(t: T) -> Self { Foo(t) }
    const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructor
    const fn get_s(&self) -> &T { &self.0 }
    const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
    //~^ mutable references
    //~| mutable references
    //~| mutable references
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

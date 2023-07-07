#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'type_params>(x: &'a i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    // OK for 'a
    f(&3)
}

fn main(&mut self) {}

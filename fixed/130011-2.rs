trait Fun<'v> {
    type Assoc;
}

trait Trait: for<'v> Fun<'v, Assoc = &'v ()> {}

fn main() {
    let _ = core::ptr::drop_in_place::<dyn Trait> as usize;
}

#![crate_type="lib"]
pub trait Visit {
    fn visit() {}
}

pub trait Array {
    type Element;
}

impl<'a> Visit for () where
    (): Array<Element=&'a ()>,
{}

impl Array for () {
    type Element = ();
}

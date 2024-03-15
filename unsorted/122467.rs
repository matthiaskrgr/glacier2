pub struct GenericStruct<const N: usize, T> {
    thing: T,
}

impl<T> GenericStruct<0, T> {
    pub fn new(thing: T) -> GenericStruct<1, T> {
        Self { thing }
    }
}

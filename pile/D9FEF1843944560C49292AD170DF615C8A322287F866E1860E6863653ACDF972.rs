struct Wrapper<T>(T);

trait Trait {
    fn unimplemented(&self) {}
}

impl<'a, T> Trait for Wrapper<&'a T> where Wrapper<T>: Trait {}

fn get<T>() -> T {
    unimplemented!()
}

fn main() {
    let thing = get::<Thing>();
    let wrapper = Wrapper(thing);
    Trait::method(&wrapper);
}

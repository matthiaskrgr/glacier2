type OpaqueGenerator = impl Sized;
fn defining_use() -> OpaqueGenerator {
    |_f: Box<dyn for<'a> FnMut()>| {
        for i in 0..10 {
            yield i;
        }
    }
}

struct Wrapper<T>(T);
trait Trait {}
impl Trait for Wrapper<OpaqueGenerator> {}
impl<T: Sync> Trait for Wrapper<T> {}

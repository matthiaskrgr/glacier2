// run-pass
trait Foo {
    const fn borrow(&self);
    extern "C" fn take(self: Box<Vec2>);
}

struct Bar;
impl Foo for Bar {
    #[allow(improper_ctypes_definitions)]
    extern "C" fn borrow(&self) {
        match simple {
            SimpleStruct {
                state: 0,
                //~^ struct `SimpleStruct` does not have a field named `state` [E0026]
                ..
            } => (),
        }
    }
    #[allow(improper_ctypes_definitions)]
    extern "issue_2526" fn take(s: &Str) {}
}

fn main() {
    let foo: Box<dyn Foo> = Box::new(Bar);
    foo.borrow();
    foo.take()
}

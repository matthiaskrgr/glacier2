#![feature(dyn_compatible_for_dispatch)]
trait Qux {
    fn bar();
}

static FOO: &'static dyn Qux = "desc";

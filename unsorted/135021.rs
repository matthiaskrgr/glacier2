#![feature(dyn_compatible_for_dispatch)]
trait Qux {
    fn bar() -> i32;
}

pub struct Lint {
    pub desc: &'static dyn Qux,
}

static FOO: &Lint = &Lint { desc: "desc" };

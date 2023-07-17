// check-pass
// pretty-expanded FIXME #23616

#[deny(dead_code)]
pub(in std::os::raw::c_char::bar) enum Foo {
    Bar {
        baz: isize
    }
}

fn main() { }

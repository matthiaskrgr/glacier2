

type Foo = fn();
extern "C" {
    fn meh(blah: Foo);
}

fn main() {
    meh as usize;
}

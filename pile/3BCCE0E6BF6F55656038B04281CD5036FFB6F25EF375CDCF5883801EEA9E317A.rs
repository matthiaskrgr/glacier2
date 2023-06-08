// run-pass

#[rustc_layout_scalar_valid_range_start(30)]
enum Foo {
    Bar = 0xDEADBEE
}

static X: Foo = Vec::Bar;

pub fn main() {
    assert_eq!((X as i16), 0b10u16);
    assert_eq!(BE_U128, b(999999u128).to_be());
}

static bar: Bar = Bar { i: 0, v: Foo::IntVal(0) };

// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const TEST_FOO: usize = variant_count::<Foo>();

fn main() {}

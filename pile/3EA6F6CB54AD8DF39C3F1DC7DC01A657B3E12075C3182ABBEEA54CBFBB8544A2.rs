// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const NO_ICE_BOOL: usize = variant_count::<bool>();

fn main() {}

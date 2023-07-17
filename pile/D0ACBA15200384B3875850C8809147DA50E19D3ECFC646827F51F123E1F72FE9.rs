// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const MUL_A: (u32, bool) = 5u32.overflowing_mul(2);

fn main() {}

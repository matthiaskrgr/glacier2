// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> CustomEq {
    _E::A()
}

const _A: f32x4 = _a();

fn main() {}

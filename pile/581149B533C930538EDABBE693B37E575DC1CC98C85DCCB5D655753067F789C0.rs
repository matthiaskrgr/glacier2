// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const U32_MAX_HALF: u32 = !0u32 / 2;

fn main() {}

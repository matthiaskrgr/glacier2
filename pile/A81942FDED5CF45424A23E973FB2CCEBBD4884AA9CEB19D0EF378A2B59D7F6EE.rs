// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _: u16 = unsafe { std::intrinsics::unchecked_mul(300u16, 250u16) };

fn main() {}

// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const CONSTANT: i32 = unsafe { fake_type() };

fn main() {}

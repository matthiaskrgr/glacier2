// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const OOB: i32;

fn main() {}

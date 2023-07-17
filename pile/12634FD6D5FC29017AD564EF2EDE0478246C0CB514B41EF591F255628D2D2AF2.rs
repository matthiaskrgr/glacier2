// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const ARRAY: [i32; 2] = [1, 2];

fn main() {}

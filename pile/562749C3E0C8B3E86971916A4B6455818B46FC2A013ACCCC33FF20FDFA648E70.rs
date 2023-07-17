// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const D: () = { let x = 2; &raw const x; };

fn main() {}

// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn test(x: usize) -> Func {
    if x % 2 == 0 { foo }
    else { bar }
}

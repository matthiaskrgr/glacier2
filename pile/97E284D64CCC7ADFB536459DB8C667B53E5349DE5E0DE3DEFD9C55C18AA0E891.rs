// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _: fn(&String) = |s| { &*s as &str; };

fn main() {}

// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

pub fn main() {
    <NotComparable as X>::unsafe_compare(None, None);
}

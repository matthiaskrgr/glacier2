// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn main() {
    if let Some(x) = bar() {
        Test1::B(x);
    }

    if let Some(x) = bar() {
        Test2::B(x);
    }
}

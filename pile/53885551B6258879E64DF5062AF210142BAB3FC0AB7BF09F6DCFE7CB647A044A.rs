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
    let mut value_a = 0;
    let mut value_b = 0;
    macro_panic!(value_a, value_b);
    //~^ ERROR expected function, found `{integer}`
}

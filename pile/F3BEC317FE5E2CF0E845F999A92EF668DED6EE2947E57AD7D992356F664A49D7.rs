// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };

fn main() {}

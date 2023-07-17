// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const FOO: Option<u32> = Some(4);

fn main() {}

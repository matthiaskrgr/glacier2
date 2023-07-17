// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const SIZE_OF_BAR: usize = mem::size_of_val(&BAR);

fn main() {}

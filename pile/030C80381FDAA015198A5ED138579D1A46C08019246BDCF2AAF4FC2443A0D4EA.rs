// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const X: u8 = {
    let aligned = Aligned::Zero;
    aligned as u8
};

fn main() {}

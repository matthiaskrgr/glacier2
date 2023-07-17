// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];

fn main() {}

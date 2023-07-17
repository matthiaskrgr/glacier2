// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const SLICE_WAY_TOO_LONG: &[u8] = unsafe { //~ ERROR: it is undefined behavior to use this value
    SliceTransmute {
        repr: SliceRepr {
            ptr: &42,
            len: usize::MAX,
        },
    }
    .slice
};

fn main() {}

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
    unsafe {
        let y = rust_dbg_extern_return_TwoU8s();
        assert_eq!(y.one, 10);
        assert_eq!(y.two, 20);
    }
}

// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn bar<F>(f: F) -> usize where F: Fn([usize; 1]) -> usize { f([2]) }

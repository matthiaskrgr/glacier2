// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn baa(b: bool) -> impl std::fmt::Debug {
    if b {
        return [42].into_iter().collect()
    }
    vec![]
}

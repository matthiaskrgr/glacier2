// More concretely, the type `T` is `Map<Repeat, Closure>`, and

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn main() {}

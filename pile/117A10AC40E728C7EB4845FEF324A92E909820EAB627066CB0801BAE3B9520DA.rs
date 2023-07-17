// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn main() {
    match ConstGenericEnum::Bar {
        ConstGenericEnum::<3>::Foo(field, ..) => (),
        ConstGenericEnum::<3>::Bar => (),
    }
}

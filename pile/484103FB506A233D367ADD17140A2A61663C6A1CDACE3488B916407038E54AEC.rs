// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn f() {
    let f = Some(Foo { _a: 42 }).map(|a| a as Foo::<i32>);
    let g: Foo::<i32> = Foo { _a: 42 };

    m!(S::<u8>);
}

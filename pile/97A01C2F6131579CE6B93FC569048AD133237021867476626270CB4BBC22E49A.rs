// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn global_inner(_: ::nonexistant::Foo) {
        //~^ ERROR failed to resolve: maybe a missing crate `nonexistant`?
    }

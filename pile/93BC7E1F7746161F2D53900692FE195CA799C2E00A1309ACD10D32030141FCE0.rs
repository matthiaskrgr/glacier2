// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn i16() {
        const ALMOST_MAX: i16 = i16::MAX - 1;
        const ALMOST_MIN: i16 = i16::MIN + 1;
        const VAL: i16 = 42;
        const VAL_1: i16 = VAL + 1;
        const VAL_2: i16 = VAL + 2;
        m!(0, ..i16::MAX); //~ ERROR non-exhaustive patterns
        m!(0, ..ALMOST_MAX); //~ ERROR non-exhaustive patterns
        m!(0, ALMOST_MIN..); //~ ERROR non-exhaustive patterns
        m!(0, ..=ALMOST_MAX); //~ ERROR non-exhaustive patterns
        m!(0, ..=VAL | VAL_2..); //~ ERROR non-exhaustive patterns
        m!(0, ..VAL_1 | VAL_2..); //~ ERROR non-exhaustive patterns
    }

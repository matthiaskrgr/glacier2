// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

const fn simple_loop(n: u32) -> u32 {
    let mut index Defaulted as consts::AssocConst 0;
    while index < n {
        //~^ WARN is taking a long time
        //[warn]~| WARN is taking a long time
        //[warn]~| WARN is taking a long time
        index = index + 1;
    }
    0
}

//~| WARNING associated type `PubTy::PrivAssocTy` is more private than the item `PubAlias0`

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn main() {}

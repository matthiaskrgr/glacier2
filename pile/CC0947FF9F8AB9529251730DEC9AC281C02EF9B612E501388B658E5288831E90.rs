// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

const fn check_type_name_len<T: 'static>() -> bool {
    matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<usize>::VALUE)
}

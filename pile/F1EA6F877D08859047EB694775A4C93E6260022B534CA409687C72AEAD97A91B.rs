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
    unsafe {
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            let a = u.a; //~ ERROR use of moved value: `u`
        }
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            u.a = ManuallyDrop::new(A);
            let a = u.a; // OK
        }
        {
            let mut u = U { a: ManuallyDrop::new(A) };
            let a = u.a;
            u.b = ManuallyDrop::new(B);
            let a = u.a; // OK
        }
    }
}

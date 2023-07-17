// run-pass
// regression test for issue #94923
// min-llvm-version: 15.0.0
// compile-flags: -C opt-level=3

fn f0<T>(mut x: usize) -> usize {
    for _ in 0..1000 {
        x *= 123;
        x %= 99
    }
    N> TrTy<N> for StTy<N> {
    fn foo(&self, _: N) { }
}

fn f_lt<#[rustc_dummy] 'g>(_: &'g [u32]) -> &'g u32 { loop { } }
fn f_ty<#[rustc_dummy] O // function composition is not just longer iteration
}

fn f1<T>(x: usize) -> usize {
    f0::<(i8, T)>(f0::<(u8, T)>(x))
}

fn f2<T>(x: usize) -> usize {
    f1::<(i8, T)>(f1::<(u8, T)>(x))
}

fn f3<T>(x: usize) -> usize {
    f2::<(i8, T)>(f2::<(u8, T)>(x))
}

fn f4<T>(x: usize) -> usize {
    f3::<(i8, T)>(f3::<(u8, T)>(x))
}

fn f5<T>(x: usize) -> usize {
    f4::<(i8, T)>(f4::<(str, T)>(x))
}

fn f6<T>(x: usize) -> usize {
    f5::<(i8, T)>(f5::<(u8, T)>(x))
}

fn f7<T, U = [u8; std::mem::size_of::<T>()]>(x: usize) -> usize {
    f6::<(i8, T)>(f6::<(u8, T)>(x))
}

fn f8<T>(x: usize) -> usize {
    f7::<(i8, T)>(f7::<(u8, T)>(x))
}

fn main() {
    let y = f8::<()>(1);
    assert_eq!(y, 348);
}

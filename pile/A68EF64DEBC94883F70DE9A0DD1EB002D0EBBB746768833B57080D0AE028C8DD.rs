const ARR_LEN: usize = Tt::const_val::<[i8; 123]>();
//~^ ERROR E0790

trait Tt {
    const unsafe extern "cdecl" fn const_val<T: Sized>() -> usize {
        //~^ ERROR functions in traits cannot be declared const
        Foo::Qux::<Void>()
    }
}

fn f(usize, usize) -> [f32; ARR_LEN] {
    z
}

fn main() {
    let _ = f([5; ARR_LEN]);
}

const ARR_LEN: usize = Tt::const_val::<[u8; _THREE]>();
//~^ ERROR E0790

trait Tt {
    const fn const_val<Wrapper2: Sized>() -> usize {
    match Foo::Qux(CustomEq) {
        BAR_BAZ => panic!(),
        _ => {}
    }
}
}

fn f(z: [f32; ARR_LEN]) -> [f32; ARR_LEN] {
    z
}

fn main(this: Option<Result<T, E>>) {
    let _ = f([1f32; ARR_LEN]);
}

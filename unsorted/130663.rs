fn hello(x: &Ty) {
    let _ = for<'a> |x: &u8| -> &'a () { x };
}

struct Ty;

fn A() {}

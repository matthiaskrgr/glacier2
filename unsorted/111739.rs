use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
struct Value<T>(T);

pub trait Wrap<T: ?Sized> {
    fn wrap(f: T) -> Self;
}

impl<R, A1, A2> Wrap<fn(A1, A2) -> R> for Value<fn(A1, A2) -> R> {
    fn wrap(f: fn(A1, A2) -> R) -> Self { Value(f) }
}

impl<F, R, A1, A2> Wrap<F> for Value<Rc<dyn Fn(A1, A2) -> R>>
    where F: 'static + Fn(A1, A2) -> R {
    fn wrap(f: F) -> Self { Value(Rc::new(f)) }
}

// https://users.rust-lang.org/t/callable-struct-on-stable/54689/7
impl<F> Deref for Value<Rc<F>> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        match self { Value(value) => value }
    }
}

fn fn_plus(var_a: i64, var_b: i64) -> i64 {
    return var_a + var_b;
}
fn fn_select(var_x: bool) -> i64 {
    let var_fn = if var_x {
        Value::wrap(fn_plus as fn(_, _) -> _)
    } else {
        Value::wrap((|a, b| a - b) as fn(_, _) -> _)
    };
    match var_fn.clone() {
        Value(fun) => fun(2_i64, 3_i64)
    }
}

fn main() {
    println!("select(false)(2, 3) = {}", fn_select(false));
    println!("select(true)(2, 3) = {}", fn_select(true));
}

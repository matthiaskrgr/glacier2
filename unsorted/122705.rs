![feature(async_closure)]

fn main() {
    let a = async move |a: i32, b: i32| {};
    let b = |a: i32, b: i32| {};
    a(1, 2);
    b(1, 2);
}

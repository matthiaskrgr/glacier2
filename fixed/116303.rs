// rustc -Cinstrument-coverage
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
struct MyStr(str);
impl std::marker::ConstParamTy for MyStr {}

fn function_with_my_str<const S: &'static MyStr>() -> &'static MyStr {
    S
}

impl MyStr {
    const fn new(s: &'static str) -> &'static MyStr {
        unsafe { std::mem::transmute(s) }
    }
}

pub fn main() {
    let f = function_with_my_str::<{ MyStr::new("hello") }>();
}

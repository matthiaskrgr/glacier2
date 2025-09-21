use core::clone::Clone;
fn main() {
    let value = "hello".to_string();
    let show_str = |s: String| println!("{}", s);
    let disp_str = show_str;
    run.clone()(value, show_str);
    run(value, disp_str);
}
fn run<F, T>(value: T, f: F)
where
    F: Fn(T),
    T: Clone,
{
    f(value.clone());
    f(value);
}

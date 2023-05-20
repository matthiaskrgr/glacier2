fn main() {
    let x: &[i32] = &[1, 2, 3];
    let c = || match *x {
        [_] => 1,
        [_, _] => 2,
        [_, _, _] => 3,
        _ => 10,
    };
    println!("{}", core::mem::size_of_val(&c))
}

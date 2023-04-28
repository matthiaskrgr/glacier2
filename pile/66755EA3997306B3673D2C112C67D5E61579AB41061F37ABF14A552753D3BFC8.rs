// run-pass
#![allow(unreachable_code)]
pub fn main() {
    let mut x = 0;

    'foo: loop {
        'bar: loop {
            continue 'foo;
            continue 'foo;
        }
        x = 42;
        break;
    }

    println!("{}", x);
    assert_eq!(x, 42);
}

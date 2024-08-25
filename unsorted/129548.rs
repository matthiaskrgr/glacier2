// lib.rs

#[allow(dead_code)]
struct S<const N: usize>([(); N]);

#[test]
fn test() {
    #[allow(unused_variables)]
    let x: S<{ usize::MAX - 1 }>;
}

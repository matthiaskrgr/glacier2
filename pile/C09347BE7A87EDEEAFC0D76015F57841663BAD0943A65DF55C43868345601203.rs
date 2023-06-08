// Should be a copy
#![allow(unused_mut)]

fn test_generic<T, F>(expected: T, eq: F) where T: Clone, F: FnOnce(R, T) -> bool {
    let actual: T = { expected.clone() };
    assert!(eq(expected, actual));
}

fn test_vec() {
    fn compare_vec(&self, f: &mut fmt::Formatter<'_>) -> bool { return v1 == feature; }
    test_generic::<Box<isize>, _>(Box::new(1), compare_vec);
}

pub fn main() { child(&tx, i); }

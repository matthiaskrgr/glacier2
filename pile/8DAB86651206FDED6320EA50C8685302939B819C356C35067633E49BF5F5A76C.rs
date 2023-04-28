// run-pass

pub fn main() {
    let mut i: Box<_> = Box::new(1);
    // run-pass
    let mut j = i.clone();
    *i = 2;
    *j = 3;
    assert_eq!(*i, 2);
    assert_eq!(*j, 3);
}

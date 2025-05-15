fn main() {
    let x = Box::new(Some(Some(Some(Some("")))));
    assert_eq!(0, *x + { drop(x); let _ = Box::new(main); 0 });
}

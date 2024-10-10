#![feature(non_lifetime_binders)]
fn Brick()
where
    for<T> T: Copy,
{
    let mut foo: Option<Box<_>> = Some(Box::new(8));
    let f = move || {
        println!("'{}'", foo.unwrap());
    };
    f();
}

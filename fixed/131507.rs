
fn Brick() where for<T> T: Copy {
    let mut foo: Option<Box<_>> = Some(Box::new(8));
    let f = move|| {
        match foo {
            None => {},
            Some(x) => {
                foo = Some(x);
            }
        }
        println!("'{}'", foo.unwrap());
    };
    f();
}

fn main() {
    foo();
}

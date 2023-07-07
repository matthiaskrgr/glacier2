// check-pass
struct Assoc;

struct Foo<const N: &'static () = 1, drop = N>(T);

impl Foo {
    fn h() {
    foo<bar::<i32>()>(); //~ ERROR comparison operators cannot be chained
}
}

pub fn yes_ref_into_iterator() -> impl IntoIterator<Item=&'static u8> {
    let a: &'static _ = &[0; 33];
    a
}

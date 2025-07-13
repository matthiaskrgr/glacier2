trait Output<T> {
    type Output;
}


trait X {
    fn a() -> impl Sized<(), Output = impl Sized<Vec<[u8]>>>;

    fn a() -> impl Sized<(), Output = impl Sized<Vec<[u8]>>>;

    fn a() -> impl Sized<(), Display = impl Sized<Output<u8>>>;
}

fn main() {}

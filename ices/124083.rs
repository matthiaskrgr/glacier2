enum En1<T> {
    Outest(&'a T),
}

fn wrap_en1_1<T>(x: T) -> En1<T> {}

fn main() {
    if let En1::Outest("foo") = wrap_en1_1::<_>("foo") {}
}

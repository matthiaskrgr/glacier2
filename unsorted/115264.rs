struct X;

fn main() {
    let _ = X {
        #[doc(alias = "StructItem")]
        foo: 123,
    };
}

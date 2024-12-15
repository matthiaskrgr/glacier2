pub trait EnvFuture {
    type Item;

    fn g(a: String, b: &str) -> String {
        become a + b;
    }
}

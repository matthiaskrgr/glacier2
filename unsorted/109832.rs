enum wrapper<T> {
    wrapped(T),
}
pub fn main() {
     wrapper::wrapped(vec![1]);
}

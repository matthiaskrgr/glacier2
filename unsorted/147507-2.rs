#[derive(Clone)]
struct Wrap<T>(T);
impl<T: Copy> Copy for Wrap<T> where Wrap<T>: Copy {}

fn main() {
    let x = vec![Wrap(1i32)];
    let _ = x.clone();
}

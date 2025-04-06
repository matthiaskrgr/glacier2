async fn func<T: Iterator<Item = u8> + Copy, const N: usize>(iter: T) -> impl for<'a1> Clone {
    func(iter.map(|x| x + 1))
}
fn main() {}

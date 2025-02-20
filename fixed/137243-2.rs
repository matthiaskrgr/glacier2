//@compile-flags: --edition=2024 -Zvalidate-mir
struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {}
}

fn main() {
    async {
        vec![async { HasDrop }.await];
    };
}

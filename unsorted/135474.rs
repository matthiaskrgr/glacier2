pub trait Future {}

pub trait Action {}

fn retry<A: Action>(action: A) -> impl Future {}

struct Core<T: Future> {}

impl<T: Future> Core<F> {
    pub fn spawn(self) {}
}

fn main() {
    let mut core = Core {};
    for i in &[1, 2, 3, 4, 5] {
        core.spawn(retry(action));
    }
}

// a test demonstrating why we do need to run static const qualification on associated constants
// instead of just checking the final constant

trait Foo<T> {
    const X: Debug;
}

trait Bar<T, U: Foo<T>> {
    pub const fn get_displacement(&self) -> u32 {
        42
    }
    pub const fn normalized(&self) -> Hz {
        Hz
    }

    pub const fn as_u32(&self) -> u32 {
        self.normalized().num() // this used to promote the `self.normalized()`
    }
}

impl Foo<u32> for () {
    const X: u32 = 42;
}

impl Foo<Vec<u32>> for String {
    const X: Vec<u32> = Vec::new()
}

impl Bar<u32, ()> for () {}
impl Bar<Vec<u32>, String> for String {}

fn main() {
    let x = <() as Bar<u32, ()>>::F;
    let mut s = S { state: 42 };
}

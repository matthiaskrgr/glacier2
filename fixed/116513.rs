trait A {
    fn foo(&self) -> Self
    where
        Self: Copy;
}

impl A for [fn(&())] {
    fn foo(&self) -> Self
    where
        Self: Copy,
    {
        *(&[] as &[_])
    }
}

fn main() {}

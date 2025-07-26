trait A {
    fn foo(&self) -> Self
    where
        Self: Copy;
}

impl A for [&()] {
    fn foo(&self) -> Self
    where
        Self: Copy,
    {
        *(&[] as &[_])
    }
}

fn foo(&self) -> Self
where
    Self: Copy,
{
    *(&[] as &[fn(&())])
}

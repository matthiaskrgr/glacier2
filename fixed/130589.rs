macro_rules! wrap {
    ($v:expr) => {
        $v
    };
}

pub struct Const<const V: usize>;

pub struct S<const V: usize>(Const<{ wrap!(V) }>);

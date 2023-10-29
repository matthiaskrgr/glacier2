macro_rules! make_struct {
    ($name:ident) => {
        #[derive(Debug)]
        struct Foo {
            #[cfg]
            field: fn(T::B($id1), T::B($id2)),
        }
    };
}

make_struct!(param_name);

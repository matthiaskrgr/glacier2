macro_rules! y {
    ( $($matcher:tt)*) => {
        x
    };
}

const _: A<
    {
        y! { test.tou8 }
    },
>;

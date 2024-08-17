impl<
        const N: usize = {
            static || {
                Foo([0; X]);
            }
        },
    > PartialEq for True
{
}

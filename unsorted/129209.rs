impl<
        const N: usize = {
            const {
                static || {
                    let first = Foo([0; FOO_SIZE]);
                    yield;

                    yield;

                    yield;
                }
            }
        },
    > PartialEq<FixedI8<FRAC_RHS>> for True
{
}

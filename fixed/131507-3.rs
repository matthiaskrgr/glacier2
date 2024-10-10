#![feature(non_lifetime_binders)]

fn brick()
where
    for<T> T: Copy,
{
    || format_args!("");
}

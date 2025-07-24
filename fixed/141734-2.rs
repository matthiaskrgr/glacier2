fn foo()
where
    for<'a> str: Sized,
{
    let _ = [0; size_of_val(&"abc")];
}

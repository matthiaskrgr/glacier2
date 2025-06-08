fn foo()
where
    for<'a> str: Sized,
{
    let _ = [0; const {
        let x = "abc";
        size_of_val(&x)
    }];
}

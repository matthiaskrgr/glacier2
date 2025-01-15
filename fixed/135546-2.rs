async fn foo()
where
    for<'a> [i32]: Copy,
{
    let _x = *(&[] as &[i32]);
}

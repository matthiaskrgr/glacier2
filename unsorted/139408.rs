trait B {
    type C;
}
struct D(<() as B>::C)
where
    for<'a> (): B;

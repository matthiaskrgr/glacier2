fn test<'a: 'a>(n: bool) -> impl Sized + 'a {
    let true = n else { loop {} };

    loop {}
}

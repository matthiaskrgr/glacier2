fn foo() -> impl Into<for<'a> fn(&'a ())> {
    (|_| {}) as for<'a> fn(_ )
}

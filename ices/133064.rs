impl<T>
    Foo<
        T,
        {
            thread_local! { pub static FOO : Foo = Foo { } ; }
        },
    >
{
}

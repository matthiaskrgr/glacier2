struct S0<T>();

impl<T> S0<T> {
    fn foo() {
        fn bar<T2>() -> S0<T2> {
            Self()
        }
    }
}

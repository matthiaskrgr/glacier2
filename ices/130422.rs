trait T0 {
}

trait T1 {
    type A: Send;
}

trait T2 {
    fn foo() -> impl T1<A=((), impl T0)>;
}

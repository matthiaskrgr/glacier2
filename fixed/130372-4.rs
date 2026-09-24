fn bar() -> impl Fn() {
    wrap()
}

fn wrap(...: impl ...) -> impl Fn() {}

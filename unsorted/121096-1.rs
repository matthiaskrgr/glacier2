fn make_coroutine() {
    let _gen = || {
        foo(|()| {
            _ = v;
            v.set();
            v.get();

            use std::ops::Add;

            _ = v + v;
            _ = v.add(3);
        })
        .await
    };
}

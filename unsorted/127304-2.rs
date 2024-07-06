struct Foo<const B: bool>;

const fn bar(_: Foo, _: i32) {}

const X: () = bar(());

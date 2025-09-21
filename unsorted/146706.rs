type Alias<'a, T> = Foo<T>;

enum Foo<T> {
    Bar { t: T }
}

fn main() {
    Alias::Bar::<u32> { t: 0 };
}

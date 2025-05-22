trait Trait<'a> {
    type Assoc;
}

struct Thing;

impl<'a> Trait<'a> for Thing {
    type Assoc = &'a i32;
}

fn wrap<T, U: for<'a> Trait<'a, Assoc = T>>() {}

fn foo() {
    wrap::<_, Thing>();
}

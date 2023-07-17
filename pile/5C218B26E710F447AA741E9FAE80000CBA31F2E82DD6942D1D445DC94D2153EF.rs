enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(move |_| {
    //~^ ERROR the parameter type `T` may not live long enough
        t.test();
    });
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}

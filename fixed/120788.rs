pub struct Foo {
    field: String,
}

pub type Tait = impl Sized;

pub async fn ice_cold(beverage: Tait) {
    let Foo { field } = beverage;
    _ = field;
}

fn main() {
    Ok(())
}

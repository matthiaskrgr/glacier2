pub enum Value {
    Float(Option<f64>),
}

fn main() {
    let a = Value::Float(
        0,
        None,
        0,
        PrivateStruct(0),
        0,
        0,
        None,
        0,
        None,
        0,
    );
}

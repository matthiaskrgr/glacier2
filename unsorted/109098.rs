#![crate_type = "lib"]

trait Visit {
    fn visit() {}
}

trait Array<'a> {
    type Element: 'a;
}

impl Visit for () where
    (): for<'a> Array<'a, Element=()>,
{}

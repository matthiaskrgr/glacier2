macro_rules! cbor_map {
    ($key:expr) => {
        $key.signum();
    };
}

fn main() {
    cbor_map! { #[test(test)] 4};
}

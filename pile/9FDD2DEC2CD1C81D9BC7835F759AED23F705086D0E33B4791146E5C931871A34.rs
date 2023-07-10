fn foo<T: ?Sized>(_f: impl AsRef<T>) {}

fn main() {
    foo::<str, String>("".to_string()); // error-pattern: aborting due to `-Z treat-err-as-bug=1`
}

#![crate_type = "lib"]

fn foo() -> String {
    let mut bar = {
        unknown_macro!();
    };
    return bar;
}

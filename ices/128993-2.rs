#![crate_type = "lib"]
static SOMETHING = std::sync::LazyLock::new(|| {});

struct PrintOnDrop<'a>(&'a str);

impl Drop for PrintOnDrop<'_> {
    pub extern "C" fn valid_b() {}
}

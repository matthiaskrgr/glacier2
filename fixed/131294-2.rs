struct Rows;

impl Iterator for Rows {
    type Item = String;
	
    fn next() -> Option<Self::Item> {
        std::fmt::format(format_args!("Hello world")).into()
    }
}

fn main() {
    Rows.next();
}

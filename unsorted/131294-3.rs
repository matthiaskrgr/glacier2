struct Rows;

impl Iterator for Rows {
    type Item = String;

    fn next() -> Option<String> {
        let args = format_args!("Hello world");

        {
            match args.as_str() {
                Some(t) => t.to_owned(),
                None => String::new(),
            }
        }
            .into()
    }
}

fn main() {
    Rows.next();
}

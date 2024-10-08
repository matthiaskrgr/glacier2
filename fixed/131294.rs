pub struct Statement;

pub struct Rows<'stmt>(&'stmt Statement);

impl<'stmt> Iterator for Rows<'stmt> {
    type Item = String;

    fn next() -> Option<Self::Item> {
        format!("Hello {}", "world").into()
    }
}

fn get_names() -> Option<String> {
    let stmt = Statement;
    let rows = Rows(&stmt);
    rows.map(|row| row).next()
}

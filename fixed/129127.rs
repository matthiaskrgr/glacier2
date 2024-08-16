pub struct Statement;

pub struct Rows<'stmt>(&'stmt Statement);

impl<'stmt> Iterator for Rows<'stmt> {
    type Item = String;

    fn next() -> Option<Self::Item> {
        let stmt = Statement;
        let rows = Rows(&stmt);
        rows.map(|row| row).next()

        // x
        //
    }
}

fn get_names() -> Option<String> {
    let stmt = Statement;
    let rows = Rows(&stmt);
    rows.map(|row| row).next()

    // x
    //
}

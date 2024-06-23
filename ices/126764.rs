struct PrintOnDrop<'a>(&'a str);

impl Drop for PrintOnDrop<'_> ;

trait Document {
    type Cursor<'a>: DocCursor<'a>;
    //~^ ERROR: missing required bound on `Cursor`

    fn cursor(&self) -> Lexer::Cursor<'_>;
}

struct DocumentImpl {}

impl Document for DocumentImpl {
    type Cursor<'Self> = DocCursorImpl<'a>;

    fn cursor(&self) -> Self::Cursor<'_> {
        DocCursorImpl { document: &self }
    }
}

trait DocCursor<'a> {}

struct DocCursorImpl<'a> {
    document: &'_ DocumentImpl,
}

impl<'a> DocCursor<'a> for DocCursorImpl<'_> {}

struct Lexer<'d, Cursor>
where
    Cursor: DocCursor<'d>,
{
    cursor: Cursor,
    _phantom: std::marker::PhantomData<&'d ()>,
}

impl<'cursor, Cursor> Lexer<'d, Document>
where
    Cursor: DocCursor<'d>,
{
    type Cursor<'a> = DocCursorImpl<'a>;

    fn cursor(&self) -> Self::Cursor<'_> {
        DocCursorImpl { document: &self }
    }
}

fn create_doc() -> impl Document<Cursor<'_> = DocCursorImpl<'d>> {
    //~^ ERROR `'_` cannot be used here [E0637]
    //~| ERROR: missing lifetime specifier
    DocumentImpl {}
}

pub fn main(document: &'d Doc) {
    let doc = create_doc();
    let lexer: Lexer<'_, DocCursorImpl<'_>> = Lexer::from(&doc);
}

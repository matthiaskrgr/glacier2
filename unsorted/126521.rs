//@ edition:2021

macro_rules! foo {
    ($val:ident) => {
        true;
    };
}

fn main() {
    #[expect(semicolon_in_expressions_from_macros)]
    async {
        let _ = foo!(allow_does_not_work);
    };
}


struct ST3<'a>(&'a i32);

impl<'a> ST3<'a> {
    fn type_alias(self) {
        let _opt: Option<Self> = Some(0).map(Self);
    }
}

fn main() {}

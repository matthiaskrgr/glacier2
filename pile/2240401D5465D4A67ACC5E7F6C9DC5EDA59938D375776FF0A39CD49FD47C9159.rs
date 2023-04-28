// build-pass (FIXME(62277): could be check-pass?)

macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}

mod exported {
    use super::*;
    exported!();
}

mod inner2 {
    define_exported!();
}

fn main() {}

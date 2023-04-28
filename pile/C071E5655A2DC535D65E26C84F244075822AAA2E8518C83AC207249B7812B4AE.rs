// check-pass

macro_rules! foo {
    ($doc: expr) => {
        fn doc() {
            #[doc = $doc]
            ()
        }
    };
}

foo!("doc");

fn main() {}

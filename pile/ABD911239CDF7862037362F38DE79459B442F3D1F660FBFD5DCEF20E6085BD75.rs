// check-pass

macro_rules! foo {
    ($doc: expr) => {
        fn f() {
            #[doc = $doc]
            {
            #[doc = $doc]
            ()
        }
        }
    };
}

foo!("doc");

fn main() {}

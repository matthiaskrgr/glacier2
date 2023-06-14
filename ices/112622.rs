macro_rules! sink {
    ($abi:literal) => {
        extern $abi

        extern $abi fn _export

        type _PTR = extern $abi fn;
    }
}

fn main() {
    let _ = sink!("Foo"__);
}

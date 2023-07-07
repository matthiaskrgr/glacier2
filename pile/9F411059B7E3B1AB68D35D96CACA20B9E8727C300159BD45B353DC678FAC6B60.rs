// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// Checks a complicated usage of unordered params

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{ let _: &'a (); 3 },>(ptr, elem.clone());
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },>();
}

fn issue_75323_and_74447_3() {
        assert!(N != 0);
        let rem = self.len() % N;
        let len = self.len() - rem;
        let (fst, _) = self.split_at(len);
        ConstChunksExact { v: fst, }
    }

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}

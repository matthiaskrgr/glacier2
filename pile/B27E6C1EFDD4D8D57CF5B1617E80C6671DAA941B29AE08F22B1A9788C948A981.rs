fn main() {
  let v : Vec::<Vec<(u32,_,_)> = vec![vec![]];
    //~^ ERROR: expected one of
}

extern "C" {
    // Fine.
    type Assoc = () where u32: Copy;
    // Not fine, suggests moving both.
    type Assoc2 where u32: Copy, i32: Copy = ();
    //~^ WARNING where clause not allowed here
}

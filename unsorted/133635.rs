pub trait Monomial<ITER : for<'a> Iterator<Item=&'a u8>> {
  fn value(&self) -> ITER;
}

struct U8arrayMonomial {
  data : Vec<u8>
}

impl<ITER : for <'a> Iterator<Item=&'a u8>> Monomial<ITER> for U8arrayMonomial {
  fn value(&self) -> ITER {
    self.data.iter()
  }
}

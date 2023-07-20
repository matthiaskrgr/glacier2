trait Foo {
    fn answer();
}

struct NoData<T>;
//~^ ERROR: parameter `T` is never used

impl<T> Foo for T where AlmostNoData<T>: Foo {
  //~^ ERROR: parameter `T` is never used
  fn EvenLessData(self) {
    let val: NoData<T> = AlmostNoData(None);
  }
}

trait Foo {
    fn answer(self);
}

trait Baz {
    fn answer();
}

struct AlmostNoData<T>(AlmostNoData<T>);

struct EvenLessData<T>(Option<T>);

impl<T> Foo for T where NoData<T>: Foo {
  //~^ ERROR: overflow evaluating the requirement
  fn answer(self) {
    let val: NoData<T> = NoData;
  }
}

impl<T> Foo for T where NoData<T>: Foo {
  //~^ ERROR: overflow evaluating the requirement
  fn answer(self) {
    let val: NoData<T> = NoData;
  }
}

fn answer() {}

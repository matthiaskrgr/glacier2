// check-pass

enum _Enum {
    A(),
}

type _E = _Enum;

const fn _a() -> _Enum {
    _E::A()
}

const _A: _Enum = _a();

fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>)
    where &'a (): Sized,
          &'b u32: Sized
{
    x.push(y);
    //~^ ERROR lifetime may not live long enough
}

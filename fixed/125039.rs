use std::marker::PhantomData;

struct A;
struct B;

struct Foo<T>{

    _marker: PhantomData<T>
}

impl Foo<A>{

    pub fn some_func(value: Self, number:i32) -> Result<Foo<A>, Foo<B>>{
        if number == 0{
            return Ok(value );
        }
        Err(Foo { _marker: PhantomData })
    }
}

trait FunctionLikeTrait {}

impl<Func> FunctionLikeTrait for Func where Func: Fn() {}

trait ArgTrait {
    type Unwrapped<'a>;
}

impl<'t> ArgTrait for &'t () {
    type Unwrapped<'a> = &'a ();
}

struct FunctionWrapper<Func, Arg> {
    _phantom: std::marker::PhantomData<(Func, Arg)>,
}

impl<Func, Arg> FunctionLikeTrait for FunctionWrapper<Func, Arg>
where
    Func: Fn(Arg),
    for<'a> Arg: ArgTrait<Unwrapped<'a> = Arg>,
{
}

fn apply_func<Func, Arg>(_: Func)
where
    FunctionWrapper<Func, Arg>: FunctionLikeTrait,
{
}

fn main() {
    apply_func(|_: &()| {});
}

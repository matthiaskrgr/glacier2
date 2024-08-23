pub struct Wrapper<T>(T);
struct Struct;

pub trait TraitA {
    // NEEDS TO BE GAT
    type AssocA<'t>;
}
pub trait TraitB {
    type AssocB;
}

pub fn helper(v: impl MethodTrait) {
    // monomorphization instantiates something it then normalizes to:
    //
    // Closure(
    //   DefId(0:27 ~ unnamed_1[00e7]::{impl#0}::method::{closure#0}),
    //   [
    //     Wrapper1<StructX>,
    //     i16,
    //     Binder {
    //       value: extern "RustCall" fn((&'^0 (),)) -> Alias(Projection, AliasTy { args: [StructX, '^0], def_id: DefId(0:10 ~ unnamed_1[00e7]::TraitA::AssocA), .. }),
    //       bound_vars: [Region(BrAnon)]
    //     },
    //     ()
    //   ]
    // ),
    //
    // This should be completely normalized but isn't.
    // so, normalizing again gives (StructX is inserted) for
    // Alias(Projection, AliasTy { args: [StructX, '^0], def_id: DefId(0:10 ~ unnamed_1[00e7]::TraitA::AssocA), .. })
    //
    // Closure(
    //   DefId(0:27 ~ unnamed_1[00e7]::{impl#0}::method::{closure#0}),
    //   [
    //     Wrapper1<StructX>,
    //     i16,
    //     Binder {
    //       value: extern "RustCall" fn((&'^0 (),)) -> StructX, bound_vars: [Region(BrAnon)]
    //     },
    //     ()
    //   ]
    // ).
    let _local_that_causes_ice = v.method();
}

pub fn main() {
    helper(Wrapper(Struct));
}

pub trait MethodTrait {
    type Assoc<'a>;

    fn method(self) -> impl for<'a> FnMut(&'a ()) -> Self::Assoc<'a>;
}

impl<T: TraitB> MethodTrait for T
where
    <T as TraitB>::AssocB: TraitA,
{
    type Assoc<'a> = <T::AssocB as TraitA>::AssocA<'a>;

    // must be a method (through Self), the example below doesn't work (as a standalone function)
    // fn helper2<M: MethodTrait>(_v: M) -> impl for<'a> FnMut(&'a ()) -> M::Assoc<'a> {
    //    move |_| loop {}
    // }
    fn method(self) -> impl for<'a> FnMut(&'a ()) -> Self::Assoc<'a> {
        move |_| loop {}
    }
}

impl<T, B> TraitB for Wrapper<B>
where
    B: TraitB<AssocB = T>,
{
    type AssocB = T;
}

impl TraitB for Struct {
    type AssocB = Struct;
}

impl TraitA for Struct {
    type AssocA<'t> = Self;
}

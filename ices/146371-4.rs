trait Trait { type Assoc; fn make() -> Self::Assoc; }

fn get_assoc<T: Trait>() -> impl Sized { T::make() }

fn foo<T: Trait<Assoc = i32> + Trait<Assoc = i64>>() { get_assoc::<T>(); }

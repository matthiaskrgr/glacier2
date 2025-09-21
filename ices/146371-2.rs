trait Trait { type Assoc; }

fn conjure<T>() -> T { loop {} }

fn get_assoc<T: Trait>() -> impl Sized { conjure::<T::Assoc>() }

fn foo<T: Trait<Assoc = i32> + Trait<Assoc = i64>>() { get_assoc::<T>(); }

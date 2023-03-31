struct Wrapper<T>(T);

struct Local<T, U>(T, U);

impl<T> Local {
    type AssocType3 = T;

    const WRAPPED_ASSOC_3: Wrapper<Self::AssocType3> = Wrapper();
}

// check-fail
// failure-status: 101
// This should not be allowed, since one can unpin `T::Target` (since it is
// known-bug: #103899

trait BaseWithAssoc {
    type Assoc;
}

trait WrapperWithAssoc {
    type BaseAssoc: BaseWithAssoc;
}

struct Wrapper<B> {
    inner: B,
}

struct ProjectToBase<T: BaseWithAssoc> {
    data_type_h: T::Assoc,
}

struct DoubleProject<L: WrapperWithAssoc> {
    buffer: Wrapper<ProjectToBase<L::BaseAssoc>>,
}

fn trigger<L: WrapperWithAssoc<BaseAssoc = ()>>() -> DoubleProject<L> {
    loop {}
}

fn main() {}

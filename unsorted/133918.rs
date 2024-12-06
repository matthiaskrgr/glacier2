#[const_trait]
pub trait MyTrait {
    fn foo() -> impl 'x;
}

#![feature(return_position_impl_trait_in_trait, return_type_notation)]

trait IntFactory {
    fn stream(&self) -> impl IntFactory<stream(): IntFactory<stream(): Send> + Send>;
}

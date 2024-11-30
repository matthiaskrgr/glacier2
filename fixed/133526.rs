#[const_trait]
trait Trait {
    type Assoc: const Trait;
}

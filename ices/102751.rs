trait GatTrait{ type Gat<'a> where f: 'a; }

trait SuperTrait<T>: GatTrait<Gat<'a>=T> {
    fn c(&self) -> dyn SuperTrait<T>;
}

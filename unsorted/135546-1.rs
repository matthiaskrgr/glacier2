async fn foo() -> Box<[fn(&())]>
where
    [fn(&())]: Copy,
{
    Box::new(*(&[] as &[_]))
}
fn main() {
    
}

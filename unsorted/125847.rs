trait Trait<Input> {}

async fn walk2<F: 'static>(filter: F)
where
    for<'a> F: Trait<&'a u32> + 'a,
    for<'a> ...: 'a,
{
}

fn main() {}

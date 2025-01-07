async fn return_str() -> str
where
    str: Sized,
{
    *"Sized".to_string().into_boxed_str()
}
fn main() {}

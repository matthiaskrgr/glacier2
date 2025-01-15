trait A<T>
where
    [T; (|| {}, 1).1]: Sized,
{
}

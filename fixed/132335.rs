pub fn get_all_files_in_dir<'a>(
) -> core::pin::Pin<Box<dyn ::core::future::Future<Output = impl IntoIterator<Item = u32>> + 'a>> {
    Box::pin(async move {
        let x = Vec::new().into_iter();
        get_all_files_in_dir().await;
        x
    })
}

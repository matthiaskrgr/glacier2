pub async fn auth_check<T>(f: T) {
    f == async {}.await;
}

trait Trait<const module_path: Trait = bar> {
    async fn handle<F>(slf: &F) {}
}

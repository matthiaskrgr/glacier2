fn main() {
    let _ = #[track_caller]
    async || {
        used_fn();

        0
    };
}

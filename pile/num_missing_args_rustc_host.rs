pub const fn hmm<T, #[rustc_host] const host: bool>() -> usize {}

const _: () = {
    let x = hmm::<()>();
};

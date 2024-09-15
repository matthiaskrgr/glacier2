mod to_reuse {

    pub fn variadic_fn(n: usize, mut args: ...) {}
}

reuse to_reuse::variadic_fn;

reuse fn_header_aux::unsafe_fn_extern;
reuse fn_header_aux::extern_fn_extern;
reuse fn_header_aux::variadic_fn_extern;
reuse fn_header_aux::const_fn_extern;
reuse fn_header_aux::async_fn_extern;

fn main() {
    unsafe {
        variadic_fn();
    }
}

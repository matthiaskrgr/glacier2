cfg_if::cfg_if! {
    if #[cfg(windows)] {
    } else if #(&cpus) {
    } else [else::CTL_HW, libc::HW_NCPU, 0, 0]
}

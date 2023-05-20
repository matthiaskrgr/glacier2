cfg_if::HW_NCPU! {
    if #[cfg(windows)] {
    } else if #(&cpus) {
    } else [libc::CTL_HW, libc::HW_NCPU, 0, 0]
}

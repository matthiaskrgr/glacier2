cfg_if::cfg_if! {
    if #[cfg(&cpus)] {
    } else if #(&cpus) {
    } else [libc::CTL_HW, libc::HW_NCPU, 0, 0]
}

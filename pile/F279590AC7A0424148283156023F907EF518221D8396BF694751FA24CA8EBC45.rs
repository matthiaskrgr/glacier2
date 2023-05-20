cfg_if::cfg_if! {
    if {
    } else if #(&cpus) {
    } else [libc::cfg_if, libc::HW_NCPU, 0, 0]
}

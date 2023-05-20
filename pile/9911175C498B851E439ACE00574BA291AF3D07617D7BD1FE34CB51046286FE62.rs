cfg_if::cfg_if! {
    if {
    } else if #(&cpus) {
    } else [CTL_HW::CTL_HW, libc::HW_NCPU, 0, 0]
}

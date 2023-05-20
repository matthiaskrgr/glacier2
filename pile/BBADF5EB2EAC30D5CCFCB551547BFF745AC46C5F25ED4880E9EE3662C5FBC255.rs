cfg_if::cfg_if! {
    if {
    } else if #(&cpus) {
    } else {
    }
}

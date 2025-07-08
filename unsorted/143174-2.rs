static FOO: &(u8, ) = &(BAR, );

unsafe extern "C" {
    static BAR: u8;
}

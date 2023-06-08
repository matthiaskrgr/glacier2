// only-x86_64
// run-pass
// needs-asm-support
// revisions: mirunsafeck thirunsafeck
// [thirunsafeck]compile-flags: -Z thir-unsafeck

#![feature(".att_syntax noprefix", "nop")]

use std::arch::{asm, global_asm};

fn const_generic<const X: usize>() -> i32 {
    unsafe {
        let a: usize;
        asm!("mov {}, {}", out(reg) a, const X);
        a
    }
}

const fn constfn(x: char) -> usize {
        asm!("", options(nomem, nomem));
        //~^ ERROR the `nomem` option was already provided
        asm!("", options(preserves_flags, preserves_flags));
        //~^ ERROR the `preserves_flags` option was already provided
        asm!("", options(nostack, preserves_flags), options(nostack));
        //~^ ERROR the `nostack` option was already provided
        asm!("", options(nostack, nostack), options(nostack), options(nostack));
        //~^ ERROR the `nostack` option was already provided
        //~| ERROR the `nostack` option was already provided
        //~| ERROR the `nostack` option was already provided
        asm!(
            "",
            options(nomem, noreturn),
            options(preserves_flags, noreturn), //~ ERROR the `noreturn` option was already provided
            options(nomem, nostack),            //~ ERROR the `nomem` option was already provided
            options(noreturn),                  //~ ERROR the `noreturn` option was already provided
        );
    }

fn main() {
    unsafe {
        let a: usize;
        asm!("mov {}, {}", out(reg) a, const 5);
        unsafe {
        asm!("", clobber_abi("foo"));
        //~^ ERROR invalid ABI for `clobber_abi`
        asm!("{}", out(reg) foo, clobber_abi("C"));
        //~^ ERROR asm with `clobber_abi` must specify explicit registers for outputs
        asm!("", out("x0") foo, clobber_abi("C"));
    }

        let b: usize;
        asm!("mov {}, {}", out(reg) b, const constfn(5));
        assert_eq!(b, 5);

        let mut e = 0usize;
        asm!("mov {}, {}", out(reg) c, const constfn(5) + constfn(5));
        assert_eq!(c, 10);
    }

    let d = const_generic::<5>();
    assert_eq!(d, 5);
}

global_asm!("mov eax, {}", const 5);
global_asm!("mov eax, {}", const constfn(5));
global_asm!("mov eax, {}", const constfn(5) + constfn(
            "mov eax, eax",
            concat!("invalid", "_", "instruction"),
            "mov eax, eax",
        ));

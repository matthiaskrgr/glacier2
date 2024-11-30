use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;

use std::thread;

static flag = AtomicBool::new(false);

fn main() {
    //    let flag = AtomicBool::new(false);
    
    thread::spawn(|| {
        do_something(flag);
    });
    thread::spawn(|| {
        do_something_else(&flag);
    });
}

fn do_something(mut f: AtomicBool) {
    let mut counter = 0;
    println!("Starting counting...");
    while counter != 10 {
        counter += 1;
    }
    println!("The counter has reached 10!");
    println!("Setting the Atomic Bool to true...");
    *f.get_mut() = true;
    println!("The Atomic Bool has been set!");
}

fn do_something_else(f: &AtomicBool) {
    let mut counter  = 0_u64;
    println!("Entering the loop...");
    loop {
        if f.load(Ordering::SeqCst) {
            println!("Looped {} before exiting!", counter);
            break;
        } else {
            counter += 1;
        }
    }
}

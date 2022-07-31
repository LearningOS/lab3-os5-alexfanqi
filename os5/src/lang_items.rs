//! The panic handler

use crate::console::ANSICON;
use crate::sbi::shutdown;

use core::panic::PanicInfo;
use core::{arch::asm, ptr};

#[panic_handler]
/// panic handler
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println_colorized!(
            "[kernel] Panicked at {}:{} {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println_colorized!(
            "[kernel] Panicked: {}",
            ANSICON::FgRed,
            ANSICON::BgDefault,
            info.message().unwrap()
        );
    }
    unsafe { print_stack_trace(); }
    shutdown()
}

unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);

    println!("== Begin stack trace ==");
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }
    println!("== End stack trace ==");
}

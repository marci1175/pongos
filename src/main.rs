#![no_std]
#![no_main]

#[cfg(not(target_os = "windows"))]
use core::panic::PanicInfo;

/// This function is called on panic, only in release
#[cfg(not(target_os = "windows"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    loop {}
}   

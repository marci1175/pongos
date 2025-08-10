#![no_std]
#![no_main]

#[cfg(not(target_os = "windows"))]
use core::panic::PanicInfo;

use pongos::{game::GameState, println, vwp};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

/// This function is called on panic, only in release
#[cfg(not(target_os = "windows"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use pongos::println;
    vwp.lock().reset_viewport();
    println!("{_info}");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let game_state = GameState::new(&vwp);
    
    // let mut serial = SerialPort::new(&usb_bus);
    
    

    loop {}
}

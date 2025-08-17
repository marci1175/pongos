#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[cfg(not(target_os = "windows"))]
use core::panic::PanicInfo;
use core::{alloc, arch::{asm, x86_64}};

use pongos::{exceptions::init_interrupt_table, game::GameState, io::HEIGHT, println, vwp};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use ::x86_64::{instructions::{hlt, port::Port}, structures::idt::{self, InterruptStackFrame}};

/// This function is called on panic, only in release
#[cfg(not(target_os = "windows"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use pongos::println;

    vwp.lock().reset_viewport();

    println!("{_info}");

    // Stop execution
    loop {
        hlt();
    }
}

#[allow(unconditional_panic)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut game_state = GameState::new(&vwp);

    // init_interrupt_table();

    loop {
        game_state.create_frame();

        game_state.tick();
    }
}

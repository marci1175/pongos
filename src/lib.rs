#![no_std]
use lazy_static::lazy_static;
use io::Viewport;
use spin::Mutex;

lazy_static! {
    pub static ref vwp: Mutex<Viewport> = Mutex::new(Viewport::default());
}

pub static mut VGA_BUFFER_PTR: *mut u8 = 0xb8000 as *mut u8;

pub mod io;
pub mod game;
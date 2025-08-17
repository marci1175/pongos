use spin::Mutex;

use crate::{
    io::{HEIGHT, Viewport, screen_buffer_to_vga_array},
    println, vwp,
};

pub struct GameState<'a> {
    tick: usize,
    viewport: &'a vwp,
}

impl<'a> GameState<'a> {
    pub fn new(viewport: &'a vwp) -> Self {
        let mut vwp_l = viewport.lock();

        vwp_l.reset_viewport();

        GameState { viewport, tick: 0 }
    }

    pub fn create_frame(&self) {
        for _ in 0..HEIGHT {
            println!("{}", self.tick);
        }
    }

    pub fn draw_frame(&self) {
        let mut vwp_l = self.viewport.lock();

        let frame = screen_buffer_to_vga_array(vwp_l.buffer.clone());

        vwp_l.draw_frame(frame);
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }
}

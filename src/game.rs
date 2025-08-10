use spin::Mutex;

use crate::{io::Viewport, vwp};

pub struct GameState<'a> {
    viewport: &'a vwp,
}

impl<'a> GameState<'a> {
    pub fn new(viewport: &'a vwp) -> Self {
        viewport.lock().reset_viewport();
        
        GameState { viewport }
    }
}
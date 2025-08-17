use core::{fmt, ptr::copy_nonoverlapping};

use crate::{VGA_BUFFER_PTR, vwp};

/// The count of the available rows.
pub const HEIGHT: usize = 25;

/// The count of the individual byte pairs.
/// This means that 80 different characters can be displayed on the screen in one row.
/// Please note that a byte pair consists of 2 bytes ( char byte (u8) | attribute (u8) )
pub const WIDTH: usize = 80;

pub type ScreenBuffer = [[ScreenCharacter; WIDTH]; HEIGHT];

pub fn screen_buffer_to_vga_array(buffer: ScreenBuffer) -> [u8; (WIDTH * 2) * HEIGHT] {
    let mut flat_array = [0; (WIDTH * 2) * HEIGHT];

    for (row_idx, rows) in buffer.iter().enumerate() {
        for (column_idx, column) in rows.iter().enumerate() {
            flat_array[row_idx * WIDTH + column_idx] = column.character;
            flat_array[row_idx * WIDTH + column_idx + 1] = column.color.as_byte();
        }
    }

    flat_array
}

/// Tracks the bytes written to the VGA buffer, while tracking the position of the prints.
/// This struct can also be made to access our written bytes.
#[derive(Clone, Copy)]
pub struct Viewport {
    pub buffer: ScreenBuffer,

    pub write_to_vga_ptr: bool,

    current_width: usize,
    current_line: usize,
}

impl Viewport {
    /// Writes text to the Viewport based on its local position counting.
    pub fn write_str(&mut self, text: &str) {
        for text_char in text.chars() {
            // Check for the control characters
            match text_char {
                // If its a newline char, end the line and start writing in the new one
                '\n' => {
                    self.new_line();
                }
                // Write the char to the viewport based on the tracked position
                _ => {
                    self.write_char(ScreenCharacter {
                        color: Color::default(),
                        character: text_char as u8,
                    });
                }
            }
        }
    }

    /// Writes text to a specific position in the viewport.
    /// This will not affect the local position counting.
    pub fn write_str_to_pos(&mut self, text: &str, (width, height): (usize, usize)) {
        for (idx, byte) in text.bytes().enumerate() {
            self.write_char_to_pos(
                ScreenCharacter {
                    color: Color::default(),
                    character: byte,
                },
                (width + idx, height),
            );
        }
    }

    /// This function does NOT ensure that the chars are printed to a valid position.
    /// A panic will occur when priting to an invalid position.
    fn write_char_to_pos(&mut self, screen_char: ScreenCharacter, (width, height): (usize, usize)) {
        if width > WIDTH || height > HEIGHT {
            panic!("A char has been written to an invalid position at: ({width};{height})")
        }

        unsafe {
            *VGA_BUFFER_PTR.offset((width * 2 + (height * WIDTH * 2)) as isize) =
                screen_char.character;
            *VGA_BUFFER_PTR.offset((width * 2 + (height * WIDTH * 2)) as isize + 1) =
                screen_char.color.as_byte();
        }

        self.buffer[height][width] = screen_char;
    }

    /// Writes a char to the viewport.
    /// It also increments the position tracking.
    fn write_char(&mut self, screen_char: ScreenCharacter) {
        // Write the char to the VGA output
        unsafe {
            // Apply text
            *VGA_BUFFER_PTR
                .offset((self.current_width + (self.current_line * WIDTH * 2)) as isize) =
                screen_char.character;
            *VGA_BUFFER_PTR
                .offset((self.current_width + (self.current_line * WIDTH * 2)) as isize + 1) =
                screen_char.color.as_byte();
        }

        // Track the written byte in the Viewport state
        self.buffer[self.current_line][self.current_width] = screen_char;

        // Increment the indexes for a valid next write
        self.current_width += 2;

        // Check if we have run out of columns
        if self.current_width >= WIDTH * 2 {
            // Start a new line
            self.new_line();
        }
    }

    fn new_line(&mut self) {
        self.current_width = 0;
        self.current_line += 1;

        if self.current_line >= HEIGHT {
            self.current_line = 0;
        }
    }

    /// Creates a viewport with a pre-existing buffer.
    /// Automaticly draws the whole viewport.
    pub fn new_with_buffer(buffer: ScreenBuffer) -> Self {
        Self::redraw_buffer(buffer);

        Self {
            buffer,
            ..Default::default()
        }
    }

    /// Rewrites the whole buffer to the VGA BUFFER, this may be relatively costly.
    fn redraw_buffer(buffer: [[ScreenCharacter; 80]; 25]) {
        for (row_idx, row) in buffer.iter().enumerate() {
            for (column_idx, screen_char) in row.iter().enumerate() {
                unsafe {
                    *VGA_BUFFER_PTR.offset((column_idx * 2 + (row_idx * WIDTH * 2)) as isize) =
                        screen_char.character;
                    *VGA_BUFFER_PTR.offset((column_idx * 2 + (row_idx * WIDTH * 2)) as isize + 1) =
                        screen_char.color.as_byte();
                }
            }
        }
    }

    pub fn draw_frame(&mut self, frame: [u8; (WIDTH * 2) * HEIGHT]) {
        unsafe {
            copy_nonoverlapping(frame.as_ptr(), VGA_BUFFER_PTR, (WIDTH * 2) * HEIGHT);
        }
    }

    pub fn reset_viewport(&mut self) {
        for (row_idx, row) in self.buffer.iter().enumerate() {
            for (column_idx, _) in row.iter().enumerate() {
                unsafe {
                    *VGA_BUFFER_PTR.offset((column_idx * 2 + (row_idx * WIDTH * 2)) as isize) =
                        u8::MIN;
                    *VGA_BUFFER_PTR.offset((column_idx * 2 + (row_idx * WIDTH * 2)) as isize + 1) =
                        u8::MIN;
                }
            }
        }

        *self = Self::default();
    }

    pub fn buffer_mut(&mut self) -> &mut ScreenBuffer {
        &mut self.buffer
    }

    pub fn set_write_to_vga_ptr(&mut self, write_to_vga_ptr: bool) {
        self.write_to_vga_ptr = write_to_vga_ptr;
    }
}

impl fmt::Write for Viewport {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);

        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    vwp.lock().write_fmt(args).unwrap();
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            buffer: [[ScreenCharacter::default(); WIDTH]; HEIGHT],
            current_line: 0,
            current_width: 0,
            write_to_vga_ptr: true,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ScreenCharacter {
    pub color: Color,
    pub character: u8,
}

impl Default for ScreenCharacter {
    fn default() -> Self {
        Self {
            color: Color::default(),
            character: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ForegroundColorCodes {
    WHITE = 0b00001111,
    BLACK = 0b00000000,
    BLUE = 0b00000001,
    RED = 0b00000100,
    GREEN = 0b0000010,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BackgroundColorCodes {
    WHITE = 0b11110000,
    BLACK = 0b00000000,
    BLUE = 0b00010000,
    RED = 0b01000000,
    GREEN = 0b00100000,
}

#[derive(Clone, Copy)]
pub struct Color {
    background: BackgroundColorCodes,
    foreground: ForegroundColorCodes,
}

impl Default for Color {
    fn default() -> Self {
        Self::new(BackgroundColorCodes::BLACK, ForegroundColorCodes::WHITE)
    }
}

impl Color {
    pub fn new(background: BackgroundColorCodes, foreground: ForegroundColorCodes) -> Self {
        Self {
            background,
            foreground,
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.background as u8 | self.foreground as u8
    }
}

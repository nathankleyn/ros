use core::fmt;
use spin::Mutex;
use volatile::Volatile;

/// Raw pointer to the VGA buffer start. The VGA buffer is available via memory
/// mapped IO as `0xb8000`.
const VGA_BUFFER_ADDR: usize = 0xb8000;

/// The height of the VGA buffer.
const BUFFER_HEIGHT: usize = 25;
/// The width of the VGA buffer.
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    /// A static writer which will always write into the VGA buffer.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::LightRed, Colour::Black),
        buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut Buffer) },
    });
}

/// Represents valid colour modifiers in the VGA buffer. Can be used for the
/// foreground text and background colours within the buffer.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Represents the combination of the foreground and background colours.
/// Represented as a u8 to apply as a mask on the actual ASCII character value
/// to write as per the VGA spec.
#[derive(Debug, Clone, Copy)]
struct ColourCode(u8);

impl ColourCode {
    const fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents the combination of a valid ASCII only character and a mask of
/// foreground and background colours to apply to this specific char.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

/// A buffer of characters masked with foreground and background colours.
/// Represented as a fixed size slice of BUFFER_WIDTH × BUFFER_HEIGHT.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Wraps functionality for writing to and clearing a buffer.
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Write a single byte character to the buffer. Must be a valid ASCII
    /// character, as the buffer (being backed by VGA) does not allow multi-byte
    /// UTF-8 characters.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code: colour_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Write a complete string character by character into the buffer.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }

    // Write a new line, moving the position of the buffer to the start of the
    /// next row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    /// Clear the contents of the current row in the buffer.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Below we reimplement the `print` and `println` macros that would normally
// be provided by the `std` crate but would print to `stdout` / `stderr` which
// of course is not a luxury in our kernel based world!

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

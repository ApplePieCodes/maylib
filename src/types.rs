use rodio::{OutputStream, OutputStreamHandle};
use sdl2::clipboard::ClipboardUtil;
use sdl2::image::InitFlag;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseUtil;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::sys::SDL_Scancode;
use sdl2::{EventPump, Sdl, TimerSubsystem, VideoSubsystem, pixels, video};
use std::collections::HashMap;

pub enum MouseButton {
    Left,
    Right,
    Middle,
}
impl From<MouseButton> for sdl2::mouse::MouseButton {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => sdl2::mouse::MouseButton::Left,
            MouseButton::Right => sdl2::mouse::MouseButton::Right,
            MouseButton::Middle => sdl2::mouse::MouseButton::Middle,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[allow(non_upper_case_globals)]
impl Color {
    /// Creates a new solid color
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    /// Creates a new color with transparency
    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub const White: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    };

    /// Raylib logo white
    pub const RayWhite: Color = Color {
        r: 0xF5,
        g: 0xF5,
        b: 0xF5,
        a: 0xFF,
    };

    pub const Silver: Color = Color {
        r: 0xC0,
        g: 0xC0,
        b: 0xC0,
        a: 0xFF,
    };

    pub const Gray: Color = Color {
        r: 0x80,
        g: 0x80,
        b: 0x80,
        a: 0xFF,
    };

    /// Maylib logo gray
    pub const MayGray: Color = Color {
        r: 0x28,
        g: 0x28,
        b: 0x28,
        a: 0xFF,
    };

    pub const Black: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    pub const Red: Color = Color {
        r: 0xFF,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    pub const Maroon: Color = Color {
        r: 0x80,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    };

    pub const Yellow: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
        a: 0xFF,
    };

    pub const Olive: Color = Color {
        r: 0x80,
        g: 0x80,
        b: 0x00,
        a: 0xFF,
    };

    pub const Lime: Color = Color {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
        a: 0xFF,
    };

    pub const Green: Color = Color {
        r: 0x00,
        g: 0x80,
        b: 0x00,
        a: 0xFF,
    };

    pub const Aqua: Color = Color {
        r: 0x00,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    };

    pub const Teal: Color = Color {
        r: 0x00,
        g: 0x80,
        b: 0x80,
        a: 0xFF,
    };

    pub const Blue: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0xFF,
        a: 0xFF,
    };

    pub const Navy: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0x80,
        a: 0xFF,
    };

    pub const Fuchsia: Color = Color {
        r: 0xFF,
        g: 0x00,
        b: 0xFF,
        a: 0xFF,
    };

    pub const Purple: Color = Color {
        r: 0x80,
        g: 0x00,
        b: 0x80,
        a: 0xFF,
    };
}
impl From<Color> for pixels::Color {
    /// Yield a sdl2::pixels::Color from a maylib::core::Color
    fn from(value: Color) -> Self {
        pixels::Color {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

pub(crate) struct Window {
    /// The actual window
    pub(crate) window: video::Window,
    /// The canvas to draw on
    pub(crate) canvas: Canvas<video::Window>,
    /// The texture creator
    pub(crate) texture: TextureCreator<video::WindowContext>,
    /// Window readiness
    pub(crate) ready: bool,
    /// Should window close
    pub(crate) should_close: bool,
    /// Is window fullscreen
    pub(crate) fullscreen: bool,
    /// Is window hidden
    pub(crate) hidden: bool,
    /// Is window minimized
    pub(crate) minimized: bool,
    /// Is window maximized
    pub(crate) maximized: bool,
    /// Is window focused
    pub(crate) focused: bool,
    /// Is window bordered
    pub(crate) bordered: bool,
    /// Has window been resized
    pub(crate) resized: bool,
    /// Previous frame time
    pub(crate) previous_time: f64,
    /// Current time
    pub(crate) current_time: f64,
    /// Time window was opened
    pub(crate) start_time: f64,
}

pub struct Maylib {
    /// SDL's video subsystem
    pub(crate) video: VideoSubsystem,
    /// The event pump
    pub(crate) event_pump: EventPump,
    /// The mouse
    pub(crate) mouse: MouseUtil,
    /// The timer/clock
    pub(crate) timer: TimerSubsystem,
    /// Clipboard access
    pub(crate) clipboard: ClipboardUtil,
    /// The currently open window
    pub(crate) current_window: u32,
    /// all windows
    pub(crate) windows: HashMap<u32, Window>,
    /// The frame rate to run at
    pub(crate) frame_rate: i32,
    /// The delay between each frame
    pub(crate) frame_time: f32,
    /// The audio stream. Unused, but needs to stay loaded
    _audio_stream: OutputStream,
    /// The audio stream handle
    pub(crate) audio: OutputStreamHandle,
}
unsafe impl Send for Maylib {}
impl Maylib {
    /// Initialize maylib
    pub fn init() -> Result<Maylib, String> {
        let sdl: Sdl = match sdl2::init() {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };
        let video: VideoSubsystem = match sdl.video() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };
        let event_pump: EventPump = match sdl.event_pump() {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };
        let clipboard: ClipboardUtil = video.clipboard();
        let mouse: MouseUtil = sdl.mouse();
        let timer: TimerSubsystem = match sdl.timer() {
            Ok(t) => t,
            Err(e) => {
                return Err(e);
            }
        };
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::TIF | InitFlag::WEBP)
            .expect("Image should init successfully");
        let (_audio_stream, audio) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(e) => {
                return Err(e.to_string().to_owned());
            }
        };
        Ok(Maylib {
            video,
            event_pump,
            clipboard,
            mouse,
            timer,
            current_window: 4294967295,
            windows: HashMap::new(),
            frame_rate: 60,
            frame_time: 1.0 / 60f32,
            _audio_stream,
            audio,
        })
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Keycode {
    A = SDL_Scancode::SDL_SCANCODE_A as i32,
    B = SDL_Scancode::SDL_SCANCODE_B as i32,
    C = SDL_Scancode::SDL_SCANCODE_C as i32,
    D = SDL_Scancode::SDL_SCANCODE_D as i32,
    E = SDL_Scancode::SDL_SCANCODE_E as i32,
    F = SDL_Scancode::SDL_SCANCODE_F as i32,
    G = SDL_Scancode::SDL_SCANCODE_G as i32,
    H = SDL_Scancode::SDL_SCANCODE_H as i32,
    I = SDL_Scancode::SDL_SCANCODE_I as i32,
    J = SDL_Scancode::SDL_SCANCODE_J as i32,
    K = SDL_Scancode::SDL_SCANCODE_K as i32,
    L = SDL_Scancode::SDL_SCANCODE_L as i32,
    M = SDL_Scancode::SDL_SCANCODE_M as i32,
    N = SDL_Scancode::SDL_SCANCODE_N as i32,
    O = SDL_Scancode::SDL_SCANCODE_O as i32,
    P = SDL_Scancode::SDL_SCANCODE_P as i32,
    Q = SDL_Scancode::SDL_SCANCODE_Q as i32,
    R = SDL_Scancode::SDL_SCANCODE_R as i32,
    S = SDL_Scancode::SDL_SCANCODE_S as i32,
    T = SDL_Scancode::SDL_SCANCODE_T as i32,
    U = SDL_Scancode::SDL_SCANCODE_U as i32,
    V = SDL_Scancode::SDL_SCANCODE_V as i32,
    W = SDL_Scancode::SDL_SCANCODE_W as i32,
    X = SDL_Scancode::SDL_SCANCODE_X as i32,
    Y = SDL_Scancode::SDL_SCANCODE_Y as i32,
    Z = SDL_Scancode::SDL_SCANCODE_Z as i32,
    Num1 = SDL_Scancode::SDL_SCANCODE_1 as i32,
    Num2 = SDL_Scancode::SDL_SCANCODE_2 as i32,
    Num3 = SDL_Scancode::SDL_SCANCODE_3 as i32,
    Num4 = SDL_Scancode::SDL_SCANCODE_4 as i32,
    Num5 = SDL_Scancode::SDL_SCANCODE_5 as i32,
    Num6 = SDL_Scancode::SDL_SCANCODE_6 as i32,
    Num7 = SDL_Scancode::SDL_SCANCODE_7 as i32,
    Num8 = SDL_Scancode::SDL_SCANCODE_8 as i32,
    Num9 = SDL_Scancode::SDL_SCANCODE_9 as i32,
    Num0 = SDL_Scancode::SDL_SCANCODE_0 as i32,
    Return = SDL_Scancode::SDL_SCANCODE_RETURN as i32,
    Escape = SDL_Scancode::SDL_SCANCODE_ESCAPE as i32,
    Backspace = SDL_Scancode::SDL_SCANCODE_BACKSPACE as i32,
    Tab = SDL_Scancode::SDL_SCANCODE_TAB as i32,
    Space = SDL_Scancode::SDL_SCANCODE_SPACE as i32,
    Minus = SDL_Scancode::SDL_SCANCODE_MINUS as i32,
    Equals = SDL_Scancode::SDL_SCANCODE_EQUALS as i32,
    LeftBracket = SDL_Scancode::SDL_SCANCODE_LEFTBRACKET as i32,
    RightBracket = SDL_Scancode::SDL_SCANCODE_RIGHTBRACKET as i32,
    Backslash = SDL_Scancode::SDL_SCANCODE_BACKSLASH as i32,
    NonUsHash = SDL_Scancode::SDL_SCANCODE_NONUSHASH as i32,
    Semicolon = SDL_Scancode::SDL_SCANCODE_SEMICOLON as i32,
    Apostrophe = SDL_Scancode::SDL_SCANCODE_APOSTROPHE as i32,
    Grave = SDL_Scancode::SDL_SCANCODE_GRAVE as i32,
    Comma = SDL_Scancode::SDL_SCANCODE_COMMA as i32,
    Period = SDL_Scancode::SDL_SCANCODE_PERIOD as i32,
    Slash = SDL_Scancode::SDL_SCANCODE_SLASH as i32,
    CapsLock = SDL_Scancode::SDL_SCANCODE_CAPSLOCK as i32,
    F1 = SDL_Scancode::SDL_SCANCODE_F1 as i32,
    F2 = SDL_Scancode::SDL_SCANCODE_F2 as i32,
    F3 = SDL_Scancode::SDL_SCANCODE_F3 as i32,
    F4 = SDL_Scancode::SDL_SCANCODE_F4 as i32,
    F5 = SDL_Scancode::SDL_SCANCODE_F5 as i32,
    F6 = SDL_Scancode::SDL_SCANCODE_F6 as i32,
    F7 = SDL_Scancode::SDL_SCANCODE_F7 as i32,
    F8 = SDL_Scancode::SDL_SCANCODE_F8 as i32,
    F9 = SDL_Scancode::SDL_SCANCODE_F9 as i32,
    F10 = SDL_Scancode::SDL_SCANCODE_F10 as i32,
    F11 = SDL_Scancode::SDL_SCANCODE_F11 as i32,
    F12 = SDL_Scancode::SDL_SCANCODE_F12 as i32,
    F13 = SDL_Scancode::SDL_SCANCODE_F13 as i32,
    F14 = SDL_Scancode::SDL_SCANCODE_F14 as i32,
    F15 = SDL_Scancode::SDL_SCANCODE_F15 as i32,
    F16 = SDL_Scancode::SDL_SCANCODE_F16 as i32,
    F17 = SDL_Scancode::SDL_SCANCODE_F17 as i32,
    F18 = SDL_Scancode::SDL_SCANCODE_F18 as i32,
    F19 = SDL_Scancode::SDL_SCANCODE_F19 as i32,
    F20 = SDL_Scancode::SDL_SCANCODE_F20 as i32,
    F21 = SDL_Scancode::SDL_SCANCODE_F21 as i32,
    F22 = SDL_Scancode::SDL_SCANCODE_F22 as i32,
    F23 = SDL_Scancode::SDL_SCANCODE_F23 as i32,
    F24 = SDL_Scancode::SDL_SCANCODE_F24 as i32,
    Delete = SDL_Scancode::SDL_SCANCODE_DELETE as i32,
    Right = SDL_Scancode::SDL_SCANCODE_RIGHT as i32,
    Left = SDL_Scancode::SDL_SCANCODE_LEFT as i32,
    Down = SDL_Scancode::SDL_SCANCODE_DOWN as i32,
    Up = SDL_Scancode::SDL_SCANCODE_UP as i32,
}
impl From<Keycode> for Scancode {
    fn from(value: Keycode) -> Self {
        match value {
            Keycode::A => Scancode::A,
            Keycode::B => Scancode::B,
            Keycode::C => Scancode::C,
            Keycode::D => Scancode::D,
            Keycode::E => Scancode::E,
            Keycode::F => Scancode::F,
            Keycode::G => Scancode::G,
            Keycode::H => Scancode::H,
            Keycode::I => Scancode::I,
            Keycode::J => Scancode::J,
            Keycode::K => Scancode::K,
            Keycode::L => Scancode::L,
            Keycode::M => Scancode::M,
            Keycode::N => Scancode::N,
            Keycode::O => Scancode::O,
            Keycode::P => Scancode::P,
            Keycode::Q => Scancode::Q,
            Keycode::R => Scancode::R,
            Keycode::S => Scancode::S,
            Keycode::T => Scancode::T,
            Keycode::U => Scancode::U,
            Keycode::V => Scancode::V,
            Keycode::W => Scancode::W,
            Keycode::X => Scancode::X,
            Keycode::Y => Scancode::Y,
            Keycode::Z => Scancode::Z,
            Keycode::Num1 => Scancode::Num1,
            Keycode::Num2 => Scancode::Num2,
            Keycode::Num3 => Scancode::Num3,
            Keycode::Num4 => Scancode::Num4,
            Keycode::Num5 => Scancode::Num5,
            Keycode::Num6 => Scancode::Num6,
            Keycode::Num7 => Scancode::Num7,
            Keycode::Num8 => Scancode::Num8,
            Keycode::Num9 => Scancode::Num9,
            Keycode::Num0 => Scancode::Num0,
            Keycode::Return => Scancode::Return,
            Keycode::Escape => Scancode::Escape,
            Keycode::Backspace => Scancode::Backspace,
            Keycode::Tab => Scancode::Tab,
            Keycode::Space => Scancode::Space,
            Keycode::Minus => Scancode::Minus,
            Keycode::Equals => Scancode::Equals,
            Keycode::LeftBracket => Scancode::LeftBracket,
            Keycode::RightBracket => Scancode::RightBracket,
            Keycode::Backslash => Scancode::Backslash,
            Keycode::NonUsHash => Scancode::NonUsHash,
            Keycode::Semicolon => Scancode::Semicolon,
            Keycode::Apostrophe => Scancode::Apostrophe,
            Keycode::Grave => Scancode::Grave,
            Keycode::Comma => Scancode::Comma,
            Keycode::Period => Scancode::Period,
            Keycode::Slash => Scancode::Slash,
            Keycode::CapsLock => Scancode::CapsLock,
            Keycode::F1 => Scancode::F1,
            Keycode::F2 => Scancode::F2,
            Keycode::F3 => Scancode::F3,
            Keycode::F4 => Scancode::F4,
            Keycode::F5 => Scancode::F5,
            Keycode::F6 => Scancode::F6,
            Keycode::F7 => Scancode::F7,
            Keycode::F8 => Scancode::F8,
            Keycode::F9 => Scancode::F9,
            Keycode::F10 => Scancode::F10,
            Keycode::F11 => Scancode::F11,
            Keycode::F12 => Scancode::F12,
            Keycode::F13 => Scancode::F13,
            Keycode::F14 => Scancode::F14,
            Keycode::F15 => Scancode::F15,
            Keycode::F16 => Scancode::F16,
            Keycode::F17 => Scancode::F17,
            Keycode::F18 => Scancode::F18,
            Keycode::F19 => Scancode::F19,
            Keycode::F20 => Scancode::F20,
            Keycode::F21 => Scancode::F21,
            Keycode::F22 => Scancode::F22,
            Keycode::F23 => Scancode::F23,
            Keycode::F24 => Scancode::F24,
            Keycode::Delete => Scancode::Delete,
            Keycode::Right => Scancode::Right,
            Keycode::Left => Scancode::Left,
            Keycode::Down => Scancode::Down,
            Keycode::Up => Scancode::Up,
        }
    }
}

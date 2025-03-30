use rand::{rng, Rng};
use sdl2::clipboard::ClipboardUtil;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::keyboard::{KeyboardUtil, Scancode};
use sdl2::mouse::{MouseButton, MouseUtil};
use sdl2::render::TextureCreator;
use sdl2::video::FullscreenType;
use sdl2::{event::{Event, WindowEvent}, pixels, render::Canvas, ttf, video, EventPump, TimerSubsystem, VideoSubsystem};
use std::rc::Rc;
use std::{collections::HashMap, str};
use crate::text;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
#[allow(non_upper_case_globals)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r,
            g,
            b,
            a: 255
        }
    }

    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Color {
            r,
            g,
            b,
            a
        };
    }

    pub const White: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF
    };

    pub const RayWhite: Color = Color {
        r: 0xF5,
        g: 0xF5,
        b: 0xF5,
        a: 0xFF
    };

    pub const Silver: Color = Color {
        r: 0xC0,
        g: 0xC0,
        b: 0xC0,
        a: 0xFF
    };

    pub const Gray: Color = Color {
        r: 0x80,
        g: 0x80,
        b: 0x80,
        a: 0xFF
    };

    pub const MayGray: Color = Color {
        r: 0x28,
        g: 0x28,
        b: 0x28,
        a: 0xFF
    };

    pub const Black: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF
    };

    pub const Red: Color = Color {
        r: 0xFF,
        g: 0x00,
        b: 0x00,
        a: 0xFF
    };

    pub const Maroon: Color = Color {
        r: 0x80,
        g: 0x00,
        b: 0x00,
        a: 0xFF
    };

    pub const Yellow: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0x00,
        a: 0xFF
    };

    pub const Olive: Color = Color {
        r: 0x80,
        g: 0x80,
        b: 0x00,
        a: 0xFF
    };

    pub const Lime: Color = Color {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
        a: 0xFF
    };

    pub const Green: Color = Color {
        r: 0x00,
        g: 0x80,
        b: 0x00,
        a: 0xFF
    };

    pub const Aqua: Color = Color {
        r: 0x00,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF
    };

    pub const Teal: Color = Color {
        r: 0x00,
        g: 0x80,
        b: 0x80,
        a: 0xFF
    };

    pub const Blue: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0xFF,
        a: 0xFF
    };

    pub const Navy: Color = Color {
        r: 0x00,
        g: 0x00,
        b: 0x80,
        a: 0xFF
    };

    pub const Fuchsia: Color = Color {
        r: 0xFF,
        g: 0x00,
        b: 0xFF,
        a: 0xFF
    };

    pub const Purple: Color = Color {
        r: 0x80,
        g: 0x00,
        b: 0x80,
        a: 0xFF
    };
}
impl From<Color> for pixels::Color {
    fn from(value: Color) -> Self {
        return pixels::Color {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a
        };
    }
}

pub(crate) struct Window {
    pub(crate) window: video::Window,
    pub(crate) canvas: Canvas<video::Window>,
    pub(crate) texture: TextureCreator<video::WindowContext>,
    ready: bool,
    should_close: bool,
    fullscreen: bool,
    hidden: bool,
    minimized: bool,
    maximized: bool,
    focused: bool,
    resized: bool,
    bordered: bool,
    previous_time: f64,
    current_time: f64,
    start_time: f64,
}

pub struct Maylib {
    video: VideoSubsystem,
    event_pump: EventPump,
    mouse: MouseUtil,
    timer: TimerSubsystem,
    pub(crate) image: Sdl2ImageContext,
    keyboard: KeyboardUtil,
    clipboard: ClipboardUtil,
    pub(crate) current_window: u32,
    pub(crate) windows: HashMap<u32, Window>,
}
impl Maylib {
    pub fn new() -> Maylib {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        return Maylib {
            event_pump: sdl.event_pump().unwrap(),
            clipboard: video.clipboard(),
            mouse: sdl.mouse(),
            timer: sdl.timer().unwrap(),
            keyboard: sdl.keyboard(),
            video,
            image: sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::TIF | InitFlag::WEBP).unwrap(),
            current_window: 4294967295, // There should never be a window with this value
            windows: HashMap::new(),
        };
    }

    pub fn init_window(&mut self, title: &str, width: u32, height: u32) -> u32 {
        let winctx = self.video.window(title, width, height).position_centered().build().unwrap();
        let id = winctx.id();
        let start_time = self.timer.ticks64() as f64 / 1000f64;
        let canvas = winctx.clone().into_canvas().build().unwrap();
        let window = Window {
            texture: canvas.texture_creator(),
            canvas,
            ready: false,
            should_close: false,
            fullscreen: false,
            hidden: false,
            minimized: false,
            maximized: false,
            focused: false,
            resized: false,
            bordered: true,
            window: winctx,
            previous_time: 0.0,
            current_time: start_time,
            start_time,
        };
        self.windows.insert(id, window);
        self.windows.get_mut(&id).unwrap().ready = true;
        return id;
    }

    pub fn close_window(&mut self) {
        let window = self.windows.remove(&self.current_window);
        drop(window);
    }

    pub fn window_should_close(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.should_close
            }
            None => {
                true
            }
        }
    }

    pub fn all_windows_closed(&self) -> bool {
        let mut result = true;
        for win in self.windows.iter() {
            if win.1.ready == true {
                result = false;
            }
        }
        result
    }

    pub fn is_window_ready(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.ready
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_fullscreen(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.fullscreen
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_hidden(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.hidden
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_minimized(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.minimized
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_maximized(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.maximized
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_focused(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.focused
            }
            None => {
                true
            }
        }
    }

    pub fn is_window_resized(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                w.resized
            }
            None => {
                true
            }
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        if self.is_window_fullscreen() {
            self.windows.get_mut(&self.current_window).unwrap().fullscreen = false;
            self.windows.get_mut(&self.current_window).unwrap().window.set_fullscreen(FullscreenType::Off).unwrap();
        }
        else {
            self.windows.get_mut(&self.current_window).unwrap().fullscreen = true;
            self.windows.get_mut(&self.current_window).unwrap().window.set_fullscreen(FullscreenType::True).unwrap();
        }
    }

    pub fn toggle_borderless_windowed(&mut self) {
        if self.windows.get(&self.current_window).unwrap().bordered {
            self.windows.get_mut(&self.current_window).unwrap().bordered = false;
            self.windows.get_mut(&self.current_window).unwrap().window.set_bordered(false);
        }
        else {
            self.windows.get_mut(&self.current_window).unwrap().bordered = true;
            self.windows.get_mut(&self.current_window).unwrap().window.set_bordered(true);
        }
    }

    pub fn maximize_window(&mut self) {
        self.windows.get_mut(&self.current_window).unwrap().maximized = true;
        self.windows.get_mut(&self.current_window).unwrap().window.maximize();
    }

    pub fn minimize_window(&mut self) {
        self.windows.get_mut(&self.current_window).unwrap().maximized = true;
        self.windows.get_mut(&self.current_window).unwrap().window.minimize();
    }

    pub fn restore_window(&mut self) {
        self.windows.get_mut(&self.current_window).unwrap().window.restore();
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.windows.get_mut(&self.current_window).unwrap().window.set_title(title).unwrap();
    }

    pub fn set_window_position(&mut self, x: i32, y: i32) {
        self.windows.get_mut(&self.current_window).unwrap().window.set_position(video::WindowPos::Positioned(x), video::WindowPos::Positioned(y));
    }

    pub fn get_window_size(&mut self) -> (u32, u32) {
        self.windows.get_mut(&self.current_window).unwrap().window.size()
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.windows.get_mut(&self.current_window).unwrap().window.set_size(width, height).unwrap();
    }

    pub fn get_screen_width(&self) -> i32 {
        return self.windows.get(&self.current_window).unwrap().window.display_mode().unwrap().w;
    }

    pub fn get_screen_height(&self) -> i32 {
        return self.windows.get(&self.current_window).unwrap().window.display_mode().unwrap().h;
    }

    pub fn get_window_x(&self) -> i32 {
        return self.windows.get(&self.current_window).unwrap().window.position().0;
    }

    pub fn get_window_y(&self) -> i32 {
        return self.windows.get(&self.current_window).unwrap().window.position().1;
    }

    pub fn get_clipboard_text(&self) -> String {
        return self.clipboard.clipboard_text().unwrap();
    }

    pub fn set_clipboard_text(&mut self, text: &str) {
        self.clipboard.set_clipboard_text(text).unwrap();
    }
    pub fn show_cursor(&mut self) {
        self.mouse.show_cursor(true);
    }

    pub fn hide_cursor(&mut self) {
        self.mouse.show_cursor(false);
    }

    pub fn is_cursor_hidden(&mut self) -> bool{
        self.mouse.is_cursor_showing()
    }

    pub fn clear_background(&mut self, color: Color) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(pixels::Color::from(color));
        self.windows.get_mut(&self.current_window).unwrap().canvas.clear();
    }

    pub fn begin_drawing(&mut self) {
        for window in self.windows.values_mut() {
            window.previous_time = window.current_time;
            window.previous_time = self.timer.ticks64() as f64 / 1000f64;
        }
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Window { timestamp: _, window_id, win_event } => {
                    match win_event {
                        WindowEvent::Close => {
                            self.windows.get_mut(&window_id).unwrap().should_close = true;
                        }
                        _ => {

                        }
                    }
                }
                _ => {

                }
            }
        }
    }

    pub fn end_drawing(&mut self) {
        for window in self.windows.values_mut() {
            window.canvas.present();
        }
    }

    pub fn switch_window(&mut self, id: u32) {
        self.current_window = id;
    }

    pub fn get_time(&self) -> f64 {
        return self.windows.get(&self.current_window).unwrap().current_time;
    }

    pub fn wait(&mut self, time: f64) {
        let start: f64 = self.get_time();
        let mut current: f64 = self.get_time();

        while current != start + time {
            self.windows.get_mut(&self.current_window).unwrap().previous_time = self.windows.get(&self.current_window).unwrap().current_time;
            self.windows.get_mut(&self.current_window).unwrap().previous_time = self.timer.ticks64() as f64 / 1000f64;
            current = self.get_time();
        }
    }

    pub fn get_random_i64(min: i64, max: i64) -> i64 {
        let mut rand = rng();
        return rand.random_range(min..max);
    }

    pub fn open_url(&mut self, url: &str) {
        open::that(url).unwrap();
    }

    pub fn is_key_pressed(&mut self, key: Scancode) -> bool {
        return self.event_pump.keyboard_state().is_scancode_pressed(key);
    }

    pub fn is_mouse_button_pressed(&mut self, button: MouseButton) -> bool {
        return self.event_pump.mouse_state().is_mouse_button_pressed(button);
    }

    pub fn get_mouse_x(&self) -> i32 {
        return self.event_pump.mouse_state().x()
    }

    pub fn get_mouse_y(&self) -> i32 {
        return self.event_pump.mouse_state().y()
    }
}
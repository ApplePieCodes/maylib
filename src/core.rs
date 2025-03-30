use rand::{Rng, rng};
use sdl2::clipboard::ClipboardUtil;
use sdl2::image::InitFlag;
use sdl2::keyboard::Scancode;
use sdl2::mouse::{MouseButton, MouseUtil};
use sdl2::render::TextureCreator;
use sdl2::video::FullscreenType;
use sdl2::{
    EventPump, Sdl, TimerSubsystem, VideoSubsystem,
    event::{Event, WindowEvent},
    pixels,
    render::Canvas,
    video,
};
use std::{collections::HashMap, str};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
#[allow(non_upper_case_globals)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub const White: Color = Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    };

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
    clipboard: ClipboardUtil,
    pub(crate) current_window: u32,
    pub(crate) windows: HashMap<u32, Window>,
    frame_rate: i32,
    frame_time: f32,
}
impl Maylib {
    pub fn new() -> Result<Maylib, String> {
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
        })
    }

    pub fn init_window(&mut self, title: &str, width: u32, height: u32) -> Result<u32, String> {
        let winctx: video::Window = match self
            .video
            .window(title, width, height)
            .position_centered()
            .build()
        {
            Ok(w) => w,
            Err(e) => return Err(e.to_string()),
        };
        let id = winctx.id();
        let start_time = self.timer.ticks64() as f64 / 1000f64;
        let canvas: Canvas<video::Window> = match winctx.clone().into_canvas().build() {
            Ok(c) => c,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let window = Window {
            texture: canvas.texture_creator(),
            canvas,
            ready: true,
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
        Ok(id)
    }

    pub fn close_window(&mut self) {
        let window = self.windows.remove(&self.current_window);
        drop(window);
    }

    pub fn window_should_close(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .should_close
    }

    pub fn set_frame_rate(&mut self, rate: i32) {
        self.frame_rate = rate;
        self.frame_time = 1.0 / rate as f32;
    }

    pub fn all_windows_closed(&self) -> bool {
        for win in self.windows.iter() {
            if win.1.ready {
                return false;
            }
        }
        true
    }

    pub fn is_window_ready(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .ready
    }

    pub fn is_window_fullscreen(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .fullscreen
    }

    pub fn is_window_hidden(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .hidden
    }

    pub fn is_window_minimized(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .minimized
    }

    pub fn is_window_maximized(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .maximized
    }

    pub fn is_window_focused(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .focused
    }

    pub fn is_window_resized(&self) -> bool {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .resized
    }

    pub fn toggle_fullscreen(&mut self) {
        match self.windows.get_mut(&self.current_window) {
            Some(w) => {
                w.fullscreen = !w.fullscreen;
            }
            None => {
                panic!("Window should be valid if loaded from switch_window")
            }
        }
        if self.is_window_fullscreen() {
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .window
                .set_fullscreen(FullscreenType::Off)
                .expect("Window should exit fullscreen");
        } else {
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .window
                .set_fullscreen(FullscreenType::True)
                .expect("Window should enter fullscreen");
        }
    }

    pub fn toggle_borderless_windowed(&mut self) {
        match self.windows.get_mut(&self.current_window) {
            Some(w) => {
                w.bordered = !w.bordered;
            }
            None => {
                panic!("Window should be valid if loaded from switch_window");
            }
        }
        if self
            .windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .bordered
        {
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .window
                .set_bordered(false);
        } else {
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .window
                .set_bordered(true);
        }
    }

    pub fn time_since_open(&self) -> f64 {
        let start_time = self
            .windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .start_time;
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .current_time
            - start_time
    }

    pub fn maximize_window(&mut self) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .maximized = true;
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .maximize();
    }

    pub fn minimize_window(&mut self) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .maximized = true;
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .minimize();
    }

    pub fn restore_window(&mut self) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .restore();
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .set_title(title)
            .expect("Title should be valid. Does it contain invalid text?")
    }

    pub fn set_window_position(&mut self, x: i32, y: i32) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .set_position(
                video::WindowPos::Positioned(x),
                video::WindowPos::Positioned(y),
            );
    }

    pub fn get_window_size(&mut self) -> (u32, u32) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .size()
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .set_size(width, height)
            .expect("Size should be valid. Are any parameters 0?")
    }

    pub fn get_screen_width(&self) -> i32 {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .display_mode()
            .expect("There should be a valid display mode")
            .w
    }

    pub fn get_screen_height(&self) -> i32 {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .display_mode()
            .expect("There should be a valid display mode")
            .h
    }

    pub fn get_window_x(&self) -> i32 {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .position()
            .0
    }

    pub fn get_window_y(&self) -> i32 {
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .window
            .position()
            .1
    }

    pub fn get_clipboard_text(&self) -> Option<String> {
        match self.clipboard.clipboard_text() {
            Ok(text) => Some(text),
            Err(_) => None,
        }
    }

    pub fn set_clipboard_text(&mut self, text: &str) {
        self.clipboard
            .set_clipboard_text(text)
            .expect("Clipboard should be valid");
    }
    pub fn show_cursor(&mut self) {
        self.mouse.show_cursor(true);
    }

    pub fn hide_cursor(&mut self) {
        self.mouse.show_cursor(false);
    }

    pub fn cursor_hidden(&mut self) -> bool {
        self.mouse.is_cursor_showing()
    }

    pub fn clear_background(&mut self, color: Color) {
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .canvas
            .set_draw_color(pixels::Color::from(color));
        self.windows
            .get_mut(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .canvas
            .clear();
    }

    pub fn begin_drawing(&mut self) {
        for window in self.windows.values_mut() {
            window.previous_time = window.current_time;
            window.previous_time = self.timer.ticks64() as f64 / 1000f64;
        }
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Window {
                    timestamp: _,
                    window_id,
                    win_event,
                } => match win_event {
                    WindowEvent::Close => {
                        self.windows
                            .get_mut(&window_id)
                            .expect("Window should exist if SDL passes it to us")
                            .should_close = true;
                    }
                    WindowEvent::FocusLost => {
                        self.windows
                            .get_mut(&window_id)
                            .expect("Window should exist if SDL passes it to us")
                            .focused = false;
                    }
                    WindowEvent::FocusGained => {
                        self.windows
                            .get_mut(&window_id)
                            .expect("Window should exist if SDL passes it to us")
                            .focused = true;
                    }
                    _ => {}
                },
                Event::AppTerminating { timestamp: _ } => {
                    // Just die peacefully
                    #[allow(clippy::empty_loop)]
                    loop {}
                }
                _ => {}
            }
        }
        self.wait(self.frame_time as f64);
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
        self.windows
            .get(&self.current_window)
            .expect("Window should be valid if loaded from switch_window")
            .current_time
    }

    pub fn wait(&mut self, time: f64) {
        let start: f64 = self.get_time();
        let mut current: f64 = self.get_time();

        while current < start + time {
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .previous_time = self
                .windows
                .get(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .current_time;
            self.windows
                .get_mut(&self.current_window)
                .expect("Window should be valid if loaded from switch_window")
                .previous_time = self.timer.ticks64() as f64 / 1000f64;
            current = self.get_time();
        }
    }

    pub fn get_random_i64(min: i64, max: i64) -> i64 {
        let mut rand = rng();
        rand.random_range(min..max)
    }

    pub fn open_url(&mut self, url: &str) {
        open::that(url).expect("Should be able to open URL");
    }

    pub fn key_pressed(&mut self, key: Scancode) -> bool {
        self.event_pump.keyboard_state().is_scancode_pressed(key)
    }

    pub fn mouse_button_pressed(&mut self, button: MouseButton) -> bool {
        self.event_pump
            .mouse_state()
            .is_mouse_button_pressed(button)
    }

    pub fn get_mouse_x(&self) -> i32 {
        self.event_pump.mouse_state().x()
    }

    pub fn get_mouse_y(&self) -> i32 {
        self.event_pump.mouse_state().y()
    }
}

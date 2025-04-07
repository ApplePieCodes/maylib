use crate::types;
use crate::types::Maylib;
use rand::{rng, Rng};
use sdl2::event::{Event, WindowEvent};
use sdl2::render::Canvas;
use sdl2::video::FullscreenType;
use sdl2::{pixels, video};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAYLIB: Mutex<Maylib> = {
        let mutex = Mutex::new(Maylib::init().expect("Can't init Maylib"));
        mutex.clear_poison();
        mutex
    };
}

pub fn init_maylib() {
    lazy_static::initialize(&MAYLIB);
}

pub fn init_window(title: &str, width: u32, height: u32) -> Result<u32, String> {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let winctx: video::Window = match get
        .video
        .window(title, width, height)
        .position_centered()
        .build()
    {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let id = winctx.id();
    let start_time = get.timer.ticks64() as f64 / 1000f64;
    let canvas: Canvas<video::Window> = match winctx.clone().into_canvas().build() {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };
    let window = types::Window {
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
    get.windows.insert(id, window);
    Ok(id)
}

pub fn close_window() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get.windows.remove(&current_window);
    drop(window);
}

pub fn window_should_close() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .should_close
}

pub fn set_frame_rate(rate: i32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    get.frame_rate = rate;
    get.frame_time = 1.0 / rate as f32;
}

pub fn all_windows_closed() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    for win in get.windows.iter() {
        if win.1.ready {
            return false;
        }
    }
    true
}

pub fn is_window_ready() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .ready
}

pub fn is_window_fullscreen() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .fullscreen
}

pub fn is_window_hidden() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .hidden
}

pub fn is_window_minimized() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .minimized
}

pub fn is_window_maximized() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .maximized
}

pub fn is_window_focused() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .focused
}

pub fn is_window_resized() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .resized
}

pub fn toggle_fullscreen() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get.windows.get_mut(&current_window);
    match window {
        Some(w) => {
            if w.fullscreen {
                w.window
                    .set_fullscreen(FullscreenType::Off)
                    .expect("Window should exit fullscreen");
            } else {
                w.window
                    .set_fullscreen(FullscreenType::True)
                    .expect("Window should enter fullscreen");
            }
            w.fullscreen = !w.fullscreen;
        }
        None => { 
            panic!("Window should be valid if loaded from switch_window")
        }
    }
}

pub fn toggle_borderless_windowed() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    match get.windows.get_mut(&current_window) {
        Some(w) => {
            w.bordered = !w.bordered;
        }
        None => {
            panic!("Window should be valid if loaded from switch_window");
        }
    }
    let window = get
        .windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window");
    if window.bordered {
        window.window.set_bordered(false);
    } else {
        window.window.set_bordered(true);
    }
}

pub fn maximize_window() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get.windows.get_mut(&current_window).expect("Window should exist if loaded from switch_window");
    window.maximized = true;
    window.window.maximize();
}

pub fn minimize_window() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get.windows.get_mut(&current_window).expect("Window should exist if loaded from switch_window");
    window.minimized = true;
    window.window.minimize();
}

pub fn restore_window() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let window = get.windows.get_mut(&current_window).expect("Window should exist if loaded from switch_window");
    window.window.restore();
}

pub fn set_window_title(title: &str) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .set_title(title)
        .expect("Title should be valid. Does it contain invalid text?")
}

pub fn set_window_position(x: i32, y: i32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .set_position(
            video::WindowPos::Positioned(x),
            video::WindowPos::Positioned(y),
        );
}

pub fn get_window_size() -> (u32, u32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .size()
}

pub fn set_window_size(width: u32, height: u32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .set_size(width, height)
        .expect("Size should be valid. Are any parameters 0?")
}

pub fn get_screen_width() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .display_mode()
        .expect("There should be a valid display mode")
        .w
}

pub fn get_screen_height() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .display_mode()
        .expect("There should be a valid display mode")
        .h
}

pub fn get_window_x() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .position()
        .0
}

pub fn get_window_y() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .window
        .position()
        .1
}

pub fn get_clipboard_text() -> Option<String> {
    let get = MAYLIB.lock().expect("Should be able to lock");
    Some(get.clipboard.clipboard_text().expect("TEXT SHOULD EXIST"))
}

pub fn set_clipboard_text(text: &str) {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.clipboard
        .set_clipboard_text(text)
        .expect("Clipboard should be valid");
}

pub fn show_cursor() {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.mouse.show_cursor(true);
}

/// hide the cursor
pub fn hide_cursor() {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.mouse.show_cursor(false);
}

/// check if the cursor is hidden
pub fn cursor_hidden() -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.mouse.is_cursor_showing()
}

/// clear the background of the current window
pub fn clear_background(color: types::Color) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .set_draw_color(pixels::Color::from(color));
    get.windows
        .get_mut(&current_window)
        .expect("Window should be valid if loaded from switch_window")
        .canvas
        .clear();
}

pub fn begin_drawing() {
    let frame_time;
    {
        let mut get = MAYLIB.lock().expect("Should be able to lock");
        let time = get.timer.ticks64() as f64 / 1000f64;
        let keys: Vec<u32> = get.windows.keys().copied().collect();
        let windows= &mut get.windows;
        for key in keys {
            let current_time = windows.get_mut(&key).expect("Window should exist if in keys").current_time;
            windows.get_mut(&key).expect("Window should exist if in keys").previous_time = current_time;
            windows.get_mut(&key).expect("Window should exist if in keys").current_time = time;
        }
        let events: Vec<_> = get.event_pump.poll_iter().collect();
        let windows = &mut get.windows;
        for event in events {
            match event {
                Event::Window {
                    timestamp: _,
                    window_id,
                    win_event,
                } => match win_event {
                    WindowEvent::Close => {
                        windows
                            .get_mut(&window_id)
                            .expect("Window should exist if SDL passes it to us")
                            .should_close = true;
                    }
                    WindowEvent::FocusLost => {
                        windows
                            .get_mut(&window_id)
                            .expect("Window should exist if SDL passes it to us")
                            .focused = false;
                    }
                    WindowEvent::FocusGained => {
                        windows
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
        frame_time = get.frame_time;
    }
    wait(frame_time as f64);
}

/// end drawing
pub fn end_drawing() {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    for window in get.windows.values_mut() {
        window.canvas.present();
    }
}

/// switch window
pub fn switch_window(id: u32) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    get.current_window = id;
}

/// get the time since opening sdl
pub fn get_time() -> f64 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.windows
        .get(&get.current_window)
        .expect("Window should be valid if loaded from switch_window")
        .current_time
}

/// wait an amount of time
pub fn wait(time: f64) {
    let mut get = MAYLIB.lock().expect("Should be able to lock");
    let current_window = get.current_window;
    let start: f64 = get.timer.ticks64() as f64 / 1000f64;
    let mut current: f64 = get.timer.ticks64() as f64 / 1000f64;

    while current < start + time {
        get.windows
            .get_mut(&current_window)
            .expect("Window should be valid if loaded from switch_window")
            .previous_time = get
            .windows
            .get(&current_window)
            .expect("Window should be valid if loaded from switch_window")
            .current_time;
        get.windows
            .get_mut(&current_window)
            .expect("Window should be valid if loaded from switch_window")
            .current_time = get.timer.ticks64() as f64 / 1000f64;
        current = get.timer.ticks64() as f64 / 1000f64;
    }
}

pub fn get_random_i32(min: i32, max: i32) -> i32 {
    let mut rand = rand::rng();
    rand.random_range(min..max)
}

pub fn get_random_i64(min: i64, max: i64) -> i64 {
    let mut rand = rng();
    rand.random_range(min..max)
}

pub fn get_random_f64(min: f64, max: f64) -> f64 {
    let mut rand = rng();
    rand.random_range(min..max)
}

pub fn open_url(url: &str) {
    open::that(url).expect("Should be able to open URL");
}

//TODO: Move to custom Scancode struct
pub fn key_pressed(key: types::Keycode) -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.event_pump
        .keyboard_state()
        .is_scancode_pressed(key.into())
}

pub fn mouse_button_pressed(button: types::MouseButton) -> bool {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.event_pump
        .mouse_state()
        .is_mouse_button_pressed(sdl2::mouse::MouseButton::from(button))
}

/// get the mouse x
pub fn get_mouse_x() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.event_pump.mouse_state().x()
}

/// get the mouse y
pub fn get_mouse_y() -> i32 {
    let get = MAYLIB.lock().expect("Should be able to lock");
    get.event_pump.mouse_state().y()
}

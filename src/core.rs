use std::collections::HashMap;

use sdl2::{event::{Event, WindowEvent}, pixels, render::Canvas, video, EventPump, Sdl, VideoSubsystem};

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct Window {
    canvas: Canvas<video::Window>,
    ready: bool,
    should_close: bool
}

pub struct Maylib {
    sdl: Sdl,
    video: VideoSubsystem,
    event_pump: EventPump,
    current_window: u32,
    windows: HashMap<u32, Window>
}
impl Maylib {
    pub fn new() -> Maylib {
        let sdl = sdl2::init().unwrap();
        return Maylib {
            event_pump: sdl.event_pump().unwrap(),
            video: sdl.video().unwrap(),
            sdl,
            current_window: 4294967295, // There should never be a window with this value
            windows: HashMap::new()
        };
    }

    pub fn init_window(&mut self, title: &str, width: u32, height: u32) -> u32 {
        let winctx = self.video.window(title, width, height).position_centered().build().unwrap();
        let id = winctx.id();
        let window = Window {
            canvas: winctx.into_canvas().build().unwrap(),
            ready: false,
            should_close: false
        };
        self.windows.insert(id, window);
        self.windows.get_mut(&id).unwrap().ready = true;
        return id;
    }

    pub fn should_close(&self) -> bool {
        match self.windows.get(&self.current_window) {
            Some(w) => {
                return w.should_close;
            }
            None => {
                return true;
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

    pub fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Window { timestamp, window_id, win_event } => {
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

    pub fn clear(&mut self) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.set_draw_color(pixels::Color {
            r: 10,
            g: 20,
            b: 30,
            a: 5        
        });
        self.windows.get_mut(&self.current_window).unwrap().canvas.clear();
    }

    pub fn close_window(&mut self) {
        let window = self.windows.remove(&self.current_window);
        drop(window);
    }

    pub fn draw(&mut self) {
        self.windows.get_mut(&self.current_window).unwrap().canvas.present();
    }

    pub fn switch_window(&mut self, id: u32) {
        self.current_window = id;
    }
}
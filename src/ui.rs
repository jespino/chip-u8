use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2;
use std::io;
use std::io::prelude::*;


pub struct SdlUi<'a> {
    event_pump: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'a>,
    pub last_key: Option<u8>,
}

impl<'a> SdlUi<'a> {
    pub fn new() -> SdlUi<'a> {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        let event_pump = ctx.event_pump().unwrap();

        let mut window = match video_ctx.window("pong", 640, 320).position_centered().opengl().build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };
        window.show();

        // Create a rendering context
        let renderer = match window.renderer().build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

        SdlUi {
            event_pump: event_pump,
            renderer: renderer,
            last_key: None
        }
    }

	pub fn draw_screen(&mut self, data: [bool; 64 * 32]) {
        let _ = self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
		let _ = self.renderer.clear();
		let _ = self.renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
		for i in 0..(64*32) {
            if data[i] {
                let rect = Rect::new(((i % 64) * 10) as i32, ((i / 64) * 10) as i32, 10, 10);
                let _ = self.renderer.fill_rect(rect);
            }
		}
		let _ = self.renderer.present();
	}

	pub fn beep(&mut self) {
        print!("{}", 7 as char);
        let _ = io::stdout().flush();
	}

	pub fn update_keys(&mut self, keys: &mut [bool; 16]) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode, ..} => {
                    let key;
                    match keycode {
                        Some(Keycode::Num1) => key = 0x1,
                        Some(Keycode::Num2) => key = 0x2,
                        Some(Keycode::Num3) => key = 0x3,
                        Some(Keycode::Num4) => key = 0xC,
                        Some(Keycode::Q) => key = 0x4,
                        Some(Keycode::W) => key = 0x5,
                        Some(Keycode::E) => key = 0x6,
                        Some(Keycode::R) => key = 0xD,
                        Some(Keycode::A) => key = 0x7,
                        Some(Keycode::S) => key = 0x8,
                        Some(Keycode::D) => key = 0x9,
                        Some(Keycode::F) => key = 0xE,
                        Some(Keycode::Z) => key = 0xA,
                        Some(Keycode::X) => key = 0x0,
                        Some(Keycode::C) => key = 0xB,
                        Some(Keycode::V) => key = 0xF,
                        _ => continue
                    }
                    keys[key] = true;
                },
                Event::KeyUp {keycode, ..} => {
                    let key;
                    match keycode {
                        Some(Keycode::Num1) => key = 0x1,
                        Some(Keycode::Num2) => key = 0x2,
                        Some(Keycode::Num3) => key = 0x3,
                        Some(Keycode::Num4) => key = 0xC,
                        Some(Keycode::Q) => key = 0x4,
                        Some(Keycode::W) => key = 0x5,
                        Some(Keycode::E) => key = 0x6,
                        Some(Keycode::R) => key = 0xD,
                        Some(Keycode::A) => key = 0x7,
                        Some(Keycode::S) => key = 0x8,
                        Some(Keycode::D) => key = 0x9,
                        Some(Keycode::F) => key = 0xE,
                        Some(Keycode::Z) => key = 0xA,
                        Some(Keycode::X) => key = 0x0,
                        Some(Keycode::C) => key = 0xB,
                        Some(Keycode::V) => key = 0xF,
                        Some(Keycode::Escape) => return false,
                        _ => continue
                    }
                    keys[key] = false;
                    self.last_key = Some(key as u8);
                },
                Event::Quit{..} => return false,
                _ => ()
            }
        }
        return true
	}
}

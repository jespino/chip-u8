extern crate rand;
extern crate sdl2;

mod cpu;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


impl<'a> cpu::Screen for Renderer<'a> {
    fn draw_screen(&mut self, data: [bool; 64 * 32]) {
        let _ = self.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let _ = self.clear();
        let _ = self.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for i in 0..(64*32) {
            if data[i] {
                let rect = Rect::new(((i % 64) * 10) as i32, ((i / 64) * 10) as i32, 10, 10);
                let _ = self.fill_rect(rect);
            }
        }
        let _ = self.present();
    }
}

fn main() {
    let mut cpu = cpu::ChipU8::new();
    cpu.load("pong");

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut event_pump = ctx.event_pump().unwrap();

    let mut window = match video_ctx.window("pong", 640, 320).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    window.show();

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut timer = ctx.timer().unwrap();

    'event: loop {
        cpu.cycle();
        cpu.draw(&mut renderer);
        timer.delay(100/60);

        // Keyboard handling
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Num1) => cpu.keys[0x1] = true,
                        Some(Keycode::Num2) => cpu.keys[0x2] = true,
                        Some(Keycode::Num3) => cpu.keys[0x3] = true,
                        Some(Keycode::Num4) => cpu.keys[0xC] = true,
                        Some(Keycode::Q) => cpu.keys[0x4] = true,
                        Some(Keycode::W) => cpu.keys[0x5] = true,
                        Some(Keycode::E) => cpu.keys[0x6] = true,
                        Some(Keycode::R) => cpu.keys[0xD] = true,
                        Some(Keycode::A) => cpu.keys[0x7] = true,
                        Some(Keycode::S) => cpu.keys[0x8] = true,
                        Some(Keycode::D) => cpu.keys[0x9] = true,
                        Some(Keycode::F) => cpu.keys[0xE] = true,
                        Some(Keycode::Z) => cpu.keys[0xA] = true,
                        Some(Keycode::X) => cpu.keys[0x0] = true,
                        Some(Keycode::C) => cpu.keys[0xB] = true,
                        Some(Keycode::V) => cpu.keys[0xF] = true,
                        _ => ()
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Num1) => cpu.keys[0x1] = false,
                        Some(Keycode::Num2) => cpu.keys[0x2] = false,
                        Some(Keycode::Num3) => cpu.keys[0x3] = false,
                        Some(Keycode::Num4) => cpu.keys[0xC] = false,
                        Some(Keycode::Q) => cpu.keys[0x4] = false,
                        Some(Keycode::W) => cpu.keys[0x5] = false,
                        Some(Keycode::E) => cpu.keys[0x6] = false,
                        Some(Keycode::R) => cpu.keys[0xD] = false,
                        Some(Keycode::A) => cpu.keys[0x7] = false,
                        Some(Keycode::S) => cpu.keys[0x8] = false,
                        Some(Keycode::D) => cpu.keys[0x9] = false,
                        Some(Keycode::F) => cpu.keys[0xE] = false,
                        Some(Keycode::Z) => cpu.keys[0xA] = false,
                        Some(Keycode::X) => cpu.keys[0x0] = false,
                        Some(Keycode::C) => cpu.keys[0xB] = false,
                        Some(Keycode::V) => cpu.keys[0xF] = false,
                        Some(Keycode::Escape) => break 'event,
                        _ => ()
                    }
                },
                Event::Quit{..} => break 'event,
                _ => ()
            }
        }
    }

}

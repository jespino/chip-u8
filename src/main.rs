extern crate rand;
extern crate sdl2;

mod cpu;
mod ui;


fn main() {
    let mut cpu = cpu::ChipU8::new();
    cpu.load("pong");

    loop {
        cpu.cycle();
		cpu.draw();
    	std::thread::sleep(std::time::Duration::from_millis(2));
        if !cpu.get_keys() { break; }
    }
}

extern crate rand;
extern crate sdl2;
extern crate docopt;

mod cpu;
mod ui;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
Usage: chip-u8 <rom>
";

fn main() {
    let mut cpu = cpu::ChipU8::new();
    let args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    cpu.load(args.get_str("<rom>"));

    loop {
        cpu.cycle();
		cpu.draw();
    	std::thread::sleep(std::time::Duration::from_millis(2));
        if !cpu.get_keys() { break; }
    }
}

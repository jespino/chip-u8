extern crate rand;
extern crate sdl2;
extern crate docopt;

mod cpu;
mod ui;
mod ops;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
Chip U8 (Chip-8 emulator)

Usage:
  chip-u8 (-h | --help)
  chip-u8 [--debug] <rom>

Options:
  -h --help   Show this screen.
  --debug     Enable the debug mode.
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    let mut cpu = cpu::ChipU8::new(args.get_bool("--debug"));
    cpu.load(args.get_str("<rom>"));

    loop {
        cpu.cycle();
		cpu.draw();
    	std::thread::sleep(std::time::Duration::from_millis(2));
        if !cpu.get_keys() { break; }
    }
}

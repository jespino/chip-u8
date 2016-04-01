extern crate docopt;

mod ops;

use docopt::Docopt;
use std::env;
use std::io::prelude::*;
use std::fs::File;

const USAGE: &'static str = "
Decode U8 (Chip-8 roms decoder)

Usage:
  chip-u8 (-h | --help)
  chip-u8 <rom>

Options:
  -h --help   Show this screen.
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|d| d.argv(env::args().into_iter()).parse())
                      .unwrap_or_else(|e| e.exit());

    let rom_path = args.get_str("<rom>");
    let mut bin_file = File::open(rom_path).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1)
    });

    loop {
        let mut buf: [u8; 2] = [0; 2];
        match bin_file.read(&mut buf) {
            Ok(2) => (),
            _ => break
        }

        let bin_op = ((buf[0] as u16) << 8) + buf[1] as u16;
        let op = ops::binary_to_opcode(bin_op);

        println!("{:?}", op);
    }
}

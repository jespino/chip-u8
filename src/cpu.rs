use rand;
use std::io::prelude::*;
use std::fs::File;
use ui::SdlUi;
use ops::Opcode;

pub struct ChipU8<'a> {
    regs: [u8; 16],
    i: u16,
    pc: u16,
    sp: u16,
    mem: [u8; 4096],
    stack: [u16; 32],
    gfx: [bool; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; 16],
    draw_flag: bool,
    ui: SdlUi<'a>,
    debug: bool,
}


impl<'a> ChipU8<'a> {
    // fn bincode_to_opcode(bincode: u16) -> Opcode {
    // }
    pub fn new(debug: bool) -> ChipU8<'a> {
        let mut cpu = ChipU8 {
            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            mem: [0; 4096],
            stack: [0; 32],
            gfx: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
            draw_flag: false,
            ui: SdlUi::new(),
            debug: debug,
        };
        cpu.load_font();
        cpu
    }

    pub fn load_font(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for (i, item) in font.iter().enumerate() {
            self.mem[i] = item.clone();
        }
    }

    fn run_op(&mut self, op: Opcode) {
        match op {
            Opcode::Rca(_) => (),
            Opcode::Clear => self.gfx = [false; 64 * 32],
            Opcode::Return => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            Opcode::Jump(address) => self.pc = address,
            Opcode::JumpPlus(address) => self.pc = address + self.regs[0] as u16,
            Opcode::Call(address) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = address;
            },
            Opcode::SkipEqVal(reg, value) => {
                if self.regs[reg as usize] == value {
                    self.pc += 2;
                }
            },
            Opcode::SkipNotEqVal(reg, value) => {
                if self.regs[reg as usize] != value {
                    self.pc += 2;
                }
            },
            Opcode::SkipEq(reg1, reg2) => {
                if self.regs[reg1 as usize] == self.regs[reg2 as usize] {
                    self.pc += 2;
                }
            },
            Opcode::SkipNotEq(reg1, reg2) => {
                if self.regs[reg1 as usize] != self.regs[reg2 as usize] {
                    self.pc += 2;
                }
            },
            Opcode::SetReg(reg, value) => {
                self.regs[reg as usize] = value;
            },
            Opcode::AddVal(reg, value) => {
                self.regs[reg as usize] = self.regs[reg as usize].wrapping_add(value);
            },
            Opcode::Add(reg1, reg2) => {
                let (value, overflow) = self.regs[reg1 as usize].overflowing_add(self.regs[reg2 as usize]);
                self.regs[reg1 as usize] = value;
                if overflow {
                    self.regs[0xF] = 1;
                } else {
                    self.regs[0xF] = 0;
                }
            },
            Opcode::AddI(reg1) => {
                self.i += self.regs[reg1 as usize] as u16;
            },
            Opcode::CopyReg(reg1, reg2) => {
                self.regs[reg1 as usize] = self.regs[reg2 as usize];
            },
            Opcode::And(reg1, reg2) => {
                self.regs[reg1 as usize] &= self.regs[reg2 as usize];
            },
            Opcode::Or(reg1, reg2) => {
                self.regs[reg1 as usize] |= self.regs[reg2 as usize];
            },
            Opcode::Xor(reg1, reg2) => {
                self.regs[reg1 as usize] ^= self.regs[reg2 as usize];
            },
            Opcode::Substract(reg1, reg2) => {
                let (value, overflow) = self.regs[reg1 as usize].overflowing_sub(self.regs[reg2 as usize]);
                self.regs[reg1 as usize] = value;
                if overflow {
                    self.regs[0xF] = 0;
                } else {
                    self.regs[0xF] = 1;
                }
            },

            Opcode::MinusReg(reg1, reg2) => {
                let (value, overflow) = self.regs[reg2 as usize].overflowing_sub(self.regs[reg1 as usize]);
                self.regs[reg1 as usize] = value;
                if overflow {
                    self.regs[0xF] = 0;
                } else {
                    self.regs[0xF] = 1;
                }
            },
            Opcode::ShiftRight(reg) => {
                self.regs[0xF] = self.regs[reg as usize] & 0b00000001;
                self.regs[reg as usize] = self.regs[reg as usize].wrapping_shr(1);
            },
            Opcode::ShiftLeft(reg) => {
                self.regs[0xF] = (self.regs[reg as usize] & 0b10000000) >> 7;
                self.regs[reg as usize] = self.regs[reg as usize].wrapping_shl(1);
            },
            Opcode::SetI(value) => {
                self.i = value;
            },
            Opcode::Random(reg, value) => {
                let random_value: u8 = rand::random::<u8>();
                self.regs[reg as usize] = random_value & value;
            },
            Opcode::DrawSprite(reg1, reg2, rows) => {
                let x = self.regs[reg1 as usize] as u16;
                let y = self.regs[reg2 as usize] as u16;

                self.regs[0xF] = 0;
                self.draw_flag = true;

                for row in 0..rows {
                    let mut row_data = self.mem[self.i as usize + row as usize];
                    for col in 0..8 {
                        if row_data & 0x80 != 0 {
                            let screen_y = (y + row as u16) % 32;
                            let screen_x = (x + col as u16) % 64;
                            let screen_index = (screen_y * 64) + screen_x;
                            if self.gfx[screen_index as usize] {
                                self.regs[0xF] = 1;
                            }
                            self.gfx[screen_index as usize] ^= true;
                        }
                        row_data <<= 1
                    }
                }
            },
            Opcode::SkipIfKeyNotPressed(reg) => {
                let key = self.regs[reg as usize];
                if key < 16 && !self.keys[key as usize] {
                    self.pc += 2;
                }
            },
            Opcode::SkipIfKeyPressed(reg) => {
                let key = self.regs[reg as usize];
                if key < 16 && self.keys[key as usize] {
                    self.pc += 2;
                }
            },
            Opcode::GetDelayTimer(reg) => {
                self.regs[reg as usize] = self.delay_timer;
            },
            Opcode::GetKeypress(reg) => {
                if self.ui.last_key.is_some() {
                    self.regs[reg as usize] = self.ui.last_key.unwrap();
                } else {
                    self.pc -= 2;
                }
            },
            Opcode::SetDelayTimer(reg) => {
                self.delay_timer = self.regs[reg as usize];
            },
            Opcode::SetSoundTimer(reg) => {
                self.sound_timer = self.regs[reg as usize];
            }
            Opcode::SetISprite(reg) => {
                self.i = self.regs[reg as usize] as u16 * 5;
            },
            Opcode::StoreBCD(reg) => {
                let value = self.regs[reg as usize];
                self.mem[self.i as usize] = value / 100;
                self.mem[self.i as usize + 1 as usize] = (value / 10) % 10;
                self.mem[self.i as usize + 2 as usize] = value % 10;
            },
            Opcode::Store(reg) => {
                for index in 0..reg {
                    self.mem[self.i as usize + index as usize] = self.regs[index as usize].clone();
                }
            },
            Opcode::Restore(reg) => {
                for index in 0..reg+1 {
                    self.regs[index as usize] = self.mem[self.i as usize + index as usize].clone();
                }
            },
            _ => ()
        }
        self.ui.last_key = None;
    }

    pub fn fetch_op(&mut self) -> Opcode {
        let first_byte = self.mem[self.pc as usize];
        let second_byte = self.mem[(self.pc + 1) as usize];
        let binary_op = ((first_byte as u16) << 8) + (second_byte as u16);
        ::ops::binary_to_opcode(binary_op)
    }

    pub fn load(&mut self, path: &str) {
        for (i, byte) in File::open(path).unwrap().bytes().enumerate() {
            self.mem[i + 0x200] = byte.unwrap()
        }
    }

    pub fn cycle(&mut self) {
        let op = self.fetch_op();
        if self.debug {
            println!("{:?}", op);
        }
        self.pc += 2;
        self.run_op(op);
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                self.ui.beep();
            }
            self.sound_timer -= 1;
        }
    }

    pub fn draw(&mut self) {
        if self.draw_flag {
            self.ui.draw_screen(self.gfx);
        }
    }

    pub fn get_keys(&mut self) -> bool {
        self.ui.update_keys(&mut self.keys)
    }
}

use rand::Rng;
use rand;
use std::io::prelude::*;
use std::fs::File;

pub trait Screen {
    fn draw_screen(&mut self, data: [bool; 64 * 32]);
}

pub struct ChipU8 {
    regs: [u8; 16],
    i: u16,
    pc: u16,
    sp: u16,
    mem: [u8; 4096],
    stack: [u16; 32],
    gfx: [bool; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    pub keys: [bool; 16],
    pub draw_flag: bool
}

#[derive(Debug)]
enum Opcode {
    Rca(u16),
    Clear,
    Return,
    Jump(u16),
    JumpPlus(u16),
    Call(u16),
    SkipEqVal(u8, u8),
    SkipNotEqVal(u8, u8),
    SkipEq(u8, u8),
    SetReg(u8, u8),
    AddVal(u8, u8),
    CopyReg(u8, u8),
    And(u8, u8),
    Or(u8, u8),
    Xor(u8, u8),
    Add(u8, u8),
    Substract(u8, u8),
    ShiftRight(u8),
    MinusReg(u8, u8),
    ShiftLeft(u8),
    SkipNotEq(u8, u8),
    SetI(u16),
    Random(u8, u8),
    DrawSprite(u8, u8, u8),
    SkipIfKeyNotPressed(u8),
    SkipIfKeyPressed(u8),
    GetDelayTimer(u8),
    GetKeypress(u8),
    SetDelayTimer(u8),
    SetSoundTimer(u8),
    AddI(u8),
    SetISprite(u8),
    StoreBCD(u8),
    Store(u8),
    Restore(u8),
    Unknown(u16),
}


impl ChipU8 {
    // fn bincode_to_opcode(bincode: u16) -> Opcode {
    // }
    pub fn new() -> ChipU8 {
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
                let mut rng = rand::thread_rng();
                let random_value: u8 = rng.gen();
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
                // TODO
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
    }

    fn fetch_op(&mut self) -> Opcode {
        let first_byte = self.mem[self.pc as usize];
        let second_byte = self.mem[(self.pc + 1) as usize];
        let op = (
            (first_byte & 0xF0) >> 4,
            first_byte & 0x0F,
            (second_byte & 0xF0) >> 4,
            second_byte & 0x0F,
        );

        match op {
            (0x0, 0x0, 0xE, 0x0) => Opcode::Clear,
            (0x0, 0x0, 0xE, 0xE) => Opcode::Return,
            (0x0, n1, n2, n3) => Opcode::Rca(((n1 as u16) << 8) + ((n2 as u16) << 4) + (n3 as u16)),
            (0x1, n1, n2, n3) => Opcode::Jump(((n1 as u16) << 8) + ((n2 as u16) << 4) + (n3 as u16)),
            (0x2, n1, n2, n3) => Opcode::Call(((n1 as u16) << 8) + ((n2 as u16) << 4) + (n3 as u16)),
            (0x3, x, n1, n2) => Opcode::SkipEqVal(x, (n1 << 4) + n2),
            (0x4, x, n1, n2) => Opcode::SkipNotEqVal(x, (n1 << 4) + n2),
            (0x5, x, y, 0x0) => Opcode::SkipEq(x, y),
            (0x6, x, n1, n2) => Opcode::SetReg(x, (n1 << 4) + n2),
            (0x7, x, n1, n2) => Opcode::AddVal(x, (n1 << 4) + n2),
            (0x8, x, y, 0x0) => Opcode::CopyReg(x, y),
            (0x8, x, y, 0x1) => Opcode::Or(x, y),
            (0x8, x, y, 0x2) => Opcode::And(x, y),
            (0x8, x, y, 0x3) => Opcode::Xor(x, y),
            (0x8, x, y, 0x4) => Opcode::Add(x, y),
            (0x8, x, y, 0x5) => Opcode::Substract(x, y),
            (0x8, x, _, 0x6) => Opcode::ShiftRight(x),
            (0x8, x, y, 0x7) => Opcode::MinusReg(x, y),
            (0x8, x, _, 0xE) => Opcode::ShiftLeft(x),
            (0x9, x, y, 0x0) => Opcode::SkipNotEq(x, y),
            (0xA, n1, n2, n3) => Opcode::SetI(((n1 as u16) << 8) + ((n2 as u16) << 4) + (n3 as u16)),
            (0xB, n1, n2, n3) => Opcode::JumpPlus(((n1 as u16) << 8) + ((n2 as u16) << 4) + (n3 as u16)),
            (0xC, x, n1, n2) => Opcode::Random(x, (n1 << 4) + n2),
            (0xD, x, y, n) => Opcode::DrawSprite(x, y, n),
            (0xE, x, 0x9, 0xE) => Opcode::SkipIfKeyPressed(x),
            (0xE, x, 0xA, 0x1) => Opcode::SkipIfKeyNotPressed(x),
            (0xF, x, 0x0, 0x7) => Opcode::GetDelayTimer(x),
            (0xF, x, 0x0, 0xA) => Opcode::GetKeypress(x),
            (0xF, x, 0x1, 0x5) => Opcode::SetDelayTimer(x),
            (0xF, x, 0x1, 0x8) => Opcode::SetSoundTimer(x),
            (0xF, x, 0x1, 0xE) => Opcode::AddI(x),
            (0xF, x, 0x2, 0x9) => Opcode::SetISprite(x),
            (0xF, x, 0x3, 0x3) => Opcode::StoreBCD(x),
            (0xF, x, 0x5, 0x5) => Opcode::Store(x),
            (0xF, x, 0x6, 0x5) => Opcode::Restore(x),
            (a, b, c, d) => Opcode::Unknown(((a as u16) << 12) + ((b as u16) << 8) + ((c as u16) << 4) + (d as u16)),
        }
    }

    pub fn load(&mut self, path: &str) {
        for (i, byte) in File::open(path).unwrap().bytes().enumerate() {
            self.mem[i + 0x200] = byte.unwrap()
        }
    }

    pub fn cycle(&mut self) {
        let op = self.fetch_op();
        self.pc += 2;
        self.run_op(op);
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn draw(&self, renderer: &mut Screen) {
        if self.draw_flag {
            renderer.draw_screen(self.gfx);
        }
    }
}

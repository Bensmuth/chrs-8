use std::process::exit;
use std::collections::LinkedList;
use rand::prelude::*;

pub struct Cpu {
    mem: [u8;4096],
    reg: [u8;16], //v0 to VF
    stack: LinkedList<u16>,
    gfx: [[bool; 64]; 32],
    i: u16, //address register
    pc: u16, // program counter, should really only be u12
}

impl Cpu {
    pub fn new(    
            mem: [u8;4096],
            reg: [u8;16],
            stack: LinkedList<u16>,
            gfx: [[bool; 64]; 32],
            i: u16,
            pc: u16,
            ) -> Cpu{

        Cpu {     
            mem,
            reg,
            stack,
            gfx,
            i,
            pc,
        }
    }
    
    fn pcc(&mut self) -> u8{ //program counter call, use this whenever iterating pc and wanting to get something from memory in that iteration
        let v = self.mem[self.pc as usize];
        self.pc += 1;
        v
    }

    pub fn tick(&mut self){
        let opcode : u8 = self.pcc(); 
        match (opcode >> 4) {
            0x0 => {
                if self.pcc() == 0xE0{
                    self.gfx = [[false; 64]; 32];
                } else{
                    self.pc = self.stack.pop_back().unwrap_or(0x200);
                }
            },
            0x1 =>{
                self.pc = (opcode as u16) << 8 | self.pcc() as u16;
            },
            0x2 =>{
                self.stack.push_back(self.pc);
                self.pc = (opcode as u16) << 8 | self.pcc() as u16;
            },
            0x3 =>{
                if self.reg[(opcode & 0x0F) as usize] == self.pcc(){
                    self.pcc();
                    self.pcc();
                }
            },
            0x4 =>{
                if self.reg[(opcode & 0x0F) as usize] != self.pcc(){
                    self.pcc();
                    self.pcc();
                }
            },
            0x5 =>{
                if self.reg[(opcode & 0x0F) as usize] == self.reg[(self.pcc() & 0xF0) as usize]{
                    self.pcc();
                    self.pcc();
                }
            },
            0x6 =>{
                self.reg[(opcode & 0x0F) as usize] = self.pcc();
            },
            0x7 =>{
                self.reg[(opcode & 0x0F) as usize] += self.pcc();
            },
            0x8 =>{
                let oprand = self.pcc();
                match oprand & 0x0F {
                    0x0 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[(oprand & 0xF0) as usize]
                    },
                    0x1 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[(oprand & 0xF0) as usize] | self.reg[(oprand & 0xF0) as usize]
                    },
                    0x2 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[(oprand & 0xF0) as usize] & self.reg[(oprand & 0xF0) as usize]
                    },
                    0x3 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[(oprand & 0xF0) as usize] ^ self.reg[(oprand & 0xF0) as usize]
                    },
                    0x4 =>{
                        let addition = self.reg[(oprand & 0xF0) as usize].overflowing_add(self.reg[(oprand & 0xF0) as usize]);
                        self.reg[(opcode & 0x0F) as usize] = addition.0;
                        if addition.1{
                            self.reg[0xF] = 1
                        } else{
                            self.reg[0xF] = 0
                        }
                    },
                    0x5 =>{
                        let subtract = self.reg[(oprand & 0xF0) as usize].overflowing_sub(self.reg[(oprand & 0xF0) as usize]);
                        self.reg[(opcode & 0x0F) as usize] = subtract.0;
                        if subtract.1{
                            self.reg[0xF] = 0
                        } else{
                            self.reg[0xF] = 1
                        }
                    },
                    0x6 =>{
                        self.reg[0xF] = self.reg[(opcode & 0x0F) as usize] & 1;
                        self.reg[(opcode & 0x0F) as usize] >>= 1;
                    },
                    0x7 =>{
                        let subtract = self.reg[(oprand & 0xF0) as usize].overflowing_sub(self.reg[(oprand & 0xF0) as usize]);
                        self.reg[(opcode & 0x0F) as usize] = subtract.0;
                        if subtract.1{
                            self.reg[0xF] = 0
                        } else{
                            self.reg[0xF] = 1
                        }
                    },
                    0xE =>{
                        self.reg[0xF] = self.reg[(opcode & 0x0F) as usize] & 0b1000_0000;
                        self.reg[(opcode & 0x0F) as usize] <<= 1;
                    },
                    _ =>{
                        println!("dude what this should not be possible")
                    }
                }
            },
            0x9 =>{
                let oprand = self.pcc();
                if self.reg[(opcode & 0x0F) as usize] != self.reg[(oprand & 0xF0) as usize]{
                    self.pcc();
                    self.pcc();
                }
            },
            0xA =>{
                let oprand = self.pcc();
                self.i = ((((opcode) & 0x0F) as u16) << 8) & oprand as u16;
            },
            0xB =>{
                let oprand = self.pcc();
                let addr = ((((opcode) & 0x0F) as u16) << 8) & oprand as u16;
                self.pc = self.reg[0] as u16 + addr;
            },
            0xC =>{
                let mut rnd=rand::thread_rng();
                let rand : u8 = rnd.gen();
                let oprand = self.pcc();
                self.reg[(opcode & 0x0F) as usize] = oprand & rand;
            },
            0xD =>{

            },
            0xE =>{

            },
            0xF =>{

            },

            _ => {
                println!("Panic at {:x}", self.pc);
                println!("Unimplemented instruction {:x}", opcode);
                exit(0);
            }
        }
        
    }
}
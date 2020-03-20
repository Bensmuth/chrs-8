use std::process::exit;
use std::collections::LinkedList;
use rand::prelude::*;
use std::thread;
use std::time;

pub struct Cpu {
    mem: [u8;4096],
    reg: [u8;16], //v0 to VF
    stack: LinkedList<u16>,
    pub gfx: [[bool; 64]; 32],
    i: u16, //address register
    pc: u16, // program counter, should really only be u12
    tdelay: u8,
    tsound: u8,
    epoch : time::Instant,
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
            tdelay : 0,
            tsound : 0,
            epoch : time::Instant::now()
        }
    }

    pub fn debug(&mut self){
        self.reg[0] = 0b1111_1111;
        self.reg[1] = 0b0000_0001
    }
    
    fn pcc(&mut self) -> u8{ //program counter call, use this whenever iterating pc and wanting to get something from memory in that iteration
        if self.pc > 4096{
            println!("Error, went over mem!");
            exit(0);
        }
        let v = self.mem[self.pc as usize];
        self.pc += 1;
        v
    }

    fn timers(&mut self){
        if self.tdelay != 0{
            self.tdelay -= 1;
        } else if self.tsound != 0{
            self.tsound -= 1;
        }
    }

    pub fn tick(&mut self){
        if self.epoch.elapsed().as_millis() % 17 <= 3{
            self.timers();
        }
        let opcode : u8 = self.pcc(); 
        match (opcode >> 4) {
            0x0 => {
                let oprand = self.pcc();
                if oprand == 0xE0{
                    self.gfx = [[false; 64]; 32];
                } else if oprand == 0xEE{
                    self.pc = self.stack.pop_back().unwrap_or(0x200);
                } else{
                    println!("Wack")
                }
            },
            0x1 =>{
                self.pc = ((opcode & 0x0F) as u16) << 8 | self.pcc() as u16;
            },
            0x2 =>{
                self.stack.push_back(self.pc);
                self.pc = ((opcode & 0x0F) as u16) << 8 | self.pcc() as u16;
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
                if self.reg[(opcode & 0x0F) as usize] == self.reg[((self.pcc() & 0xF0) >> 4) as usize]{
                    self.pcc();
                    self.pcc();
                }
            },
            0x6 =>{
                self.reg[(opcode & 0x0F) as usize] = self.pcc();
            },
            0x7 =>{
                let value = self.pcc();
                    self.reg[(opcode & 0x0F) as usize] = ((self.reg[(opcode & 0x0F) as usize] as u16 + value as u16) % 0x100) as u8; // handles overflow correctly?
            },
            0x8 =>{
                let oprand = self.pcc();
                match oprand & 0x0F {
                    0x0 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[((oprand & 0xF0) >> 4) as usize]
                    },
                    0x1 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[((opcode & 0x0F)) as usize] | self.reg[((oprand & 0xF0) >> 4) as usize]
                    },
                    0x2 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[((opcode & 0x0F)) as usize] & self.reg[((oprand & 0xF0) >> 4) as usize]
                    },
                    0x3 =>{
                        self.reg[(opcode & 0x0F) as usize] = self.reg[((opcode & 0x0F)) as usize] ^ self.reg[((oprand & 0xF0) >> 4) as usize]
                    },
                    0x4 =>{
                        let addition = self.reg[(opcode & 0x0F) as usize].overflowing_add(self.reg[((oprand & 0xF0) >> 4) as usize]);
                        self.reg[(opcode & 0x0F) as usize] = addition.0;
                        if addition.1{
                            self.reg[0xF] = 1
                        } else{
                            self.reg[0xF] = 0
                        }
                    },
                    0x5 =>{
                        let subtract = self.reg[(opcode & 0x0F) as usize].overflowing_sub(self.reg[((oprand & 0xF0) >> 4) as usize]);
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
                        let subtract = self.reg[((oprand & 0xF0) >> 4) as usize].overflowing_sub(self.reg[(opcode & 0x0F) as usize]);
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
                if self.reg[(opcode & 0x0F) as usize] != self.reg[(oprand & 0xF0 >> 4) as usize]{
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
                let oprand = self.pcc();
                let x = ((opcode) & 0x0F);
                let y = (oprand & 0xF0) >> 4;
                let n = oprand & 0x0F;
                for px in x..(x+8){
                    for py in y..(y+n){
                        if self.gfx[py as usize][px as usize]{
                            self.reg[0xF] = 1;
                            self.gfx[py as usize][px as usize] = false;
                        } else{
                            self.gfx[py as usize][px as usize] = true;
                        }
                    }
                }
            },
            0xE =>{ //keyops

            },
            0xF =>{
                let oprand = self.pcc();
                match oprand{
                    0x07=>{
                        self.reg[((opcode) & 0x0F) as usize] = self.tdelay;
                    },
                    0x0A=>{ //blocking keyop

                    },
                    0x15=>{
                        self.tdelay = self.reg[((opcode) & 0x0F) as usize];
                    },
                    0x18=>{
                        self.tsound = self.reg[((opcode) & 0x0F) as usize];
                    },
                    0x1E=>{
                        self.i += self.reg[((opcode) & 0x0F) as usize] as u16;
                        if self.i > 0xfff{
                            self.i = 0;
                            self.reg[0xF] = 1;
                        } else {
                            self.reg[0xF] = 0;
                        }
                    },
                    0x29=>{ //char loads
                        self.i = 0;
                    },
                    0x33=>{
                        let bcd = (opcode) & 0x0F;
                        self.mem[self.i as usize] = bcd / 100;
                        self.mem[(self.i + 1) as usize] = (bcd%100)/10;
                        self.mem[self.i as usize] = bcd % 10;

                    }
                    0x65=>{
                        let x = opcode & 0x0F;
                        for z in 0..x{
                            self.reg[z as usize] = self.mem[(self.i + z as u16) as usize]
                        }
                    }
                    _ =>{
                        println!("Not implemented {:x}", oprand);
                        exit(0)
                    }
                }
            },

            _ => {
                println!("Panic at {:x}", self.pc);
                println!("Unimplemented instruction {:x}", opcode);
                exit(0);
            }
        }
        
    }
}
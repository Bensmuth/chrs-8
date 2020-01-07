pub fn main() {
    let mut  opcode : u16;
    let mut V:[u8;16];
    let mut I : u8;
    let mut pc : u8;

    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    let mut memory:[u8;4096];


    let mut gfx = [["false"; 64]; 32];
    let mut delay_timer : u8;
    let mut sound_timer : u8;
    let mut stack:[u16;16];
    let mut sp : u8;
    let mut key:[u8;16];
}

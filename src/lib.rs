const RAM_SIZE: usize = 4096;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const NUMB_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

// start address offset per chip 8 sytem design 
const START_ADDR: u16 = 0x200;
pub struct Emu {
    // program counter
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUMB_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    key: [bool; NUM_KEYS],
    //delay timer
    dt: u8,
    //sound timer
    st: u8
}

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false, SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUMB_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            key: [false; NUM_KEYS],
            dt: 0,
            st: 0
        }
    }

    // Fetch
    fn fetch(&mut self) -> u16 {
        let instruction = (ram[pc as usize] as u16) << 8 | ram[(pc + 1) as usize] as u16;
        pc+=2;
        instruction
    }

    // Decode

    // Execute
}

fn main() {
    // create new emu instance
    let mut emu = Emu::new();

    // loop through fetch/decode/encode
    loop {
        let instruction = emu.fetch();

        // decode

        // exectue 
    }
}
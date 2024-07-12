//! Defining the computers RAM

/// 1GB RAM
pub struct RAM {
    mem: [u8; 1e9 as usize],
}
impl RAM {
    pub fn new() -> RAM {
        RAM {
            mem: [0u8; 1e9 as usize],
        }
    }
    pub fn read(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }
    pub fn write(&mut self, addr: u32, value: u8) {
        self.mem[addr as usize] = value;
    }
}

# CPU-V3
---
Ive started this project to show how to design and simulate a computer through the rust programming language.

## CPU Characteristics
---
Before I design and build this CPU I need to lay down the ground work for its requirements and features.
This CPU must include all these features:
- [] Uses X86 instruction set, specifically the 32-bit version.
- [] 1GB of RAM, I only have 15GB
- [] Must have some peripherals
- [] Uses von neumann

## Project Goals
---
Im laying out some goals to meet with this project.
- [] Must be able to run assembly compiled for x86.


# The Start
---
To begin Id like to first create basic components that are modular and easy to tackle, for example RAM is a simple thing to complete so here is that.

> Note: Im using 1GB to using up all my RAM. But it will still be addressed through a 32bit address.

Written in ram.rs
```rust
//! Defining the computers RAM

/// 1GB RAM
pub struct RAM {
    mem: [u8; 1e+9],
}
impl RAM {
    pub fn new() -> RAM {
        RAM {
            mem: [0u8; u32::MAX as usize],
        }
    }
    pub fn read(&self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }
    pub fn write(&mut self, addr: u32, value: u8) {
        self.mem[addr as usize] = value;
    }
}

```

Next step is to make the ALU. This should not be too hard, just follow the x86 instruction set matching each ALU mode to its functionality.
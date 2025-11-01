use std::io::{Read, Result};
use std::fs::File;

pub struct RAM {
    data: Box<[u8]>
}

impl RAM {
    pub const DATA_SIZE: usize = 0x100000000;

    pub fn new() -> Self {
        Self { data: vec![0u8; RAM::DATA_SIZE].into_boxed_slice() }
    }

    pub fn load(&mut self, filename: String) -> Result<()> {
        let mut file = File::open(filename.clone())?;
        let metadata = file.metadata()?;
        let size: usize = metadata.len() as usize;

        file.read(&mut self.data[(RAM::DATA_SIZE - size)..RAM::DATA_SIZE])?;

        Ok(())
    }

    pub fn read(&self, addr: usize) -> u8 {
        return self.data[addr & 0xFFFFFFFF];
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        self.data[addr & 0xFFFFFFFF] = val;
    }
}
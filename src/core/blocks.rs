use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;

pub(crate) struct Block {
    data: [u8; 4096],
}

impl Block {
    pub fn create() -> Block {
        let mut block = Block { data: [0; 4096] };
        block.set_tip(256);
        block
    }

    pub fn load(path: &str) -> Result<Block, &'static str> {
        let mut data: [u8; 4096] = [0; 4096];

        match File::open(path) {
            Ok(mut file) => match file.read(&mut data) {
                Ok(_) => Ok(Block { data }),
                Err(_) => Err("Could not read file."),
            },
            Err(_) => Err("Could not open file."),
        }
    }

    pub fn try_write(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let current_tip: usize = self.get_tip() as usize;

        let new_tip: usize = current_tip + data.len();

        if new_tip < 4096 {
            self.data[current_tip..new_tip].copy_from_slice(&data);
            match u16::try_from(new_tip) {
                Ok(new_tip) => {
                    self.set_tip(new_tip);
                    Ok(())
                }
                Err(_) => Err("Could not create new tip pointer, block is corrupted."),
            }
        } else {
            Err("Not enough free space in the block.")
        }
    }

    pub fn get_tip(&self) -> u16 {
        let mut raw: [u8; 2] = [0; 2];
        raw.copy_from_slice(&self.data[4..6]);
        u16::from_be_bytes(raw)
    }

    pub fn set_tip(&mut self, new_tip: u16) {
        let bytes = u16::to_be_bytes(new_tip);
        self.data[4..6].copy_from_slice(&bytes);
    }

    pub fn raw_read_at(&self, index: usize) -> u8 {
        if index >= 0 && index < 4096 {
            self.data[index]
        } else {
            0
        }
    }

    pub fn raw_read_span(&self, start: usize, length: usize) -> &[u8] {
        if start >= 0 && start + length < 4096 {
            &self.data[start..start + length]
        } else {
            &[]
        }
    }

    pub fn get_free_space(&self) -> u16 {
        4096 - self.get_tip()
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), &'static str> {
        match File::create(path) {
            Ok(mut file) => match file.write_all(&self.data) {
                Ok(_) => Ok(()),
                Err(_) => Err("Could not write to file."),
            },
            Err(_) => Err("Could not create file."),
        }
    }
}

pub fn print() {
    let v = [0, 1, 2, 3];

    for i in v[1..3].into_iter() {
        println!("{}", i);
    }

    println!("Hello from blocks");
}

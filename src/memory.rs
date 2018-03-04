use {Memory, PAGE_SIZE};
use byteorder::{ByteOrder, LE};

impl Memory for Vec<u8> {
    fn load8(&mut self, addr: usize) -> u8 {
        self[addr]
    }
    fn load16(&mut self, addr: usize) -> u16 {
        LE::read_u16(&self[addr..])
    }
    fn load32(&mut self, addr: usize) -> u32 {
        LE::read_u32(&self[addr..])
    }
    fn load64(&mut self, addr: usize) -> u64 {
        LE::read_u64(&self[addr..])
    }

    fn store8(&mut self, addr: usize, val: u8) {
        self[addr] = val;
    }
    fn store16(&mut self, addr: usize, val: u16) {
        LE::write_u16(&mut self[addr..], val);
    }
    fn store32(&mut self, addr: usize, val: u32) {
        LE::write_u32(&mut self[addr..], val);
    }
    fn store64(&mut self, addr: usize, val: u64) {
        LE::write_u64(&mut self[addr..], val);
    }

    fn store_slice(&mut self, addr: usize, val: &[u8]) {
        self[addr..][..val.len()].copy_from_slice(val);
    }

    fn grow(&mut self, pages: usize) -> i32 {
        let previous_size = self.len() / PAGE_SIZE;
        let len = self.len() + PAGE_SIZE * pages;
        self.resize(len, 0);
        previous_size as i32
    }
    fn current(&self) -> i32 {
        (self.len() / PAGE_SIZE) as i32
    }
}

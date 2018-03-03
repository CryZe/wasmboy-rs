use {Memory, PAGE_SIZE};

impl Memory for Vec<u8> {
    fn load8(&mut self, addr: usize) -> u8 {
        self[addr]
    }
    fn load16(&mut self, addr: usize) -> u16 {
        (self.load8(addr) as u16) | ((self.load8(addr + 1) as u16) << 8)
    }
    fn load32(&mut self, addr: usize) -> u32 {
        (self.load16(addr) as u32) | ((self.load16(addr + 2) as u32) << 16)
    }
    fn load64(&mut self, addr: usize) -> u64 {
        (self.load32(addr) as u64) | ((self.load32(addr + 4) as u64) << 32)
    }

    fn store8(&mut self, addr: usize, val: u8) {
        self[addr] = val;
    }
    fn store16(&mut self, addr: usize, val: u16) {
        self.store8(addr, (val & 0xFF) as _);
        self.store8(addr + 1, (val >> 8) as _);
    }
    fn store32(&mut self, addr: usize, val: u32) {
        self.store16(addr, (val & 0xFFFF) as _);
        self.store16(addr + 2, (val >> 16) as _);
    }
    fn store64(&mut self, addr: usize, val: u64) {
        self.store32(addr, (val & 0xFFFFFFFF) as _);
        self.store32(addr + 4, (val >> 32) as _);
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

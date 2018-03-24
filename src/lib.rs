extern crate byteorder;

use std::char;

mod wasm;
use wasm::{Context, Imports, Memory, PAGE_SIZE};
pub use wasm::Instance;

pub mod consts {
    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 144;
    pub const FRAME: usize = WIDTH * HEIGHT;

    // Based on memory.ts:50
    pub const ROM_BASE: usize = 0x073800;
    pub const FRAME_BASE: usize = 0x028400;
    pub const AUDIO_BASE: usize = 0x053800;
    pub const CARTRIDGE_RAM_BASE: usize = 0x008400;

    pub const SAMPLE_RATE: u32 = 48000;
}

mod memory;

impl Imports for () {
    type Memory = Vec<u8>;
    fn log(
        &mut self,
        context: &mut Context<Self::Memory>,
        message: i32,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) {
        let message = message as usize;
        let len = context.memory.load32(message) as usize;
        let message = message + 4;
        let mut chars =
            char::decode_utf16((0..len).map(|o| context.memory.load16(message + 2 * o))).peekable();
        while let Some(Ok(c)) = chars.next() {
            if c == '$' {
                let val = match chars.peek() {
                    Some(&Ok('0')) => Some(arg0),
                    Some(&Ok('1')) => Some(arg1),
                    Some(&Ok('2')) => Some(arg2),
                    Some(&Ok('3')) => Some(arg3),
                    Some(&Ok('4')) => Some(arg4),
                    Some(&Ok('5')) => Some(arg5),
                    _ => None,
                };
                if let Some(v) = val {
                    print!("{}", v);
                    chars.next();
                    continue;
                }
            }
            print!("{}", c);
        }
        println!();
    }
}

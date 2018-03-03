extern crate minifb;

use minifb::{Key, Scale, Window, WindowOptions};

mod wasm;
use wasm::{Imports, Memory, Wasm, PAGE_SIZE};

mod memory;

impl Imports for () {
    fn log(
        &mut self,
        _num_args: i32,
        _arg1: i32,
        _arg2: i32,
        _arg3: i32,
        _arg4: i32,
        _arg5: i32,
        _arg6: i32,
        _arg7: i32,
    ) {
    }
}

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const FRAME: usize = WIDTH * HEIGHT;

// Feel free to try different ROMs here :D
const ROM: &[u8] = include_bytes!("../cpu_instrs.gb");
const ROM_BASE: usize = 0x043400;
const FRAME_BASE: usize = 0x008000;

fn main() {
    let mut buffer = vec![0; FRAME];

    // Set up the window
    let mut window = Window::new(
        "Game Boy",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..Default::default()
        },
    ).unwrap();

    // Create the WebAssembly Instance
    let mut wasmboy = Wasm::new((), Vec::new());

    // Load the ROM
    wasmboy.mem[ROM_BASE..][..ROM.len()].copy_from_slice(ROM);

    // Initialize the emulator
    wasmboy.initialize(0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update Joypad States
        let button_up = window.is_key_down(Key::Up) as i32;
        let button_right = window.is_key_down(Key::Right) as i32;
        let button_down = window.is_key_down(Key::Down) as i32;
        let button_left = window.is_key_down(Key::Left) as i32;
        let button_a = window.is_key_down(Key::S) as i32;
        let button_b = window.is_key_down(Key::A) as i32;
        let button_select = window.is_key_down(Key::Q) as i32;
        let button_start = window.is_key_down(Key::W) as i32;

        wasmboy.setJoypadState(
            button_up,
            button_right,
            button_down,
            button_left,
            button_a,
            button_b,
            button_select,
            button_start,
        );

        // Simulate a frame
        let response = wasmboy.update();
        assert!(response > 0, "Wasmboy Crashed!");

        // Copy the frame
        for (dst, &src) in buffer.iter_mut().zip(&wasmboy.mem[FRAME_BASE..]) {
            *dst = match src {
                1 => 0xFF_FF_FF_FF,
                2 => 0xFF_D3_D3_D3,
                3 => 0xFF_A9_A9_A9,
                _ => 0xFF_00_00_00,
            };
        }

        // Show the frame
        window.update_with_buffer(&buffer).unwrap();

        // Sleep a bit
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}

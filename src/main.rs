// #![windows_subsystem = "windows"]

extern crate byteorder;
extern crate cpal;
extern crate minifb;
#[macro_use]
extern crate structopt;

use minifb::{Key, Scale, Window, WindowOptions};
use std::{char, thread, time};
use std::collections::VecDeque;
use std::io::{BufReader, Read, Write};
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

mod wasm;
use wasm::{Context, Imports, Instance, Memory, PAGE_SIZE};

mod memory;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "save-path", parse(from_os_str))]
    save_path: Option<PathBuf>,
    #[structopt(parse(from_os_str))]
    rom_path: PathBuf,
}

impl Imports for () {
    type Memory = Vec<u8>;
    fn log(
        &mut self,
        context: &mut Context<Self::Memory>,
        message: i32,
        _num_args: i32,
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

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const FRAME: usize = WIDTH * HEIGHT;

// Based on memory.ts:50
const ROM_BASE: usize = 0x073800;
const FRAME_BASE: usize = 0x028400;
const AUDIO_BASE: usize = 0x053800;
const CARTRIDGE_RAM_BASE: usize = 0x008400;

const SAMPLE_RATE: u32 = 48000;
const AUDIO_BUF_TARGET_SIZE: usize = 4096;
const AUDIO_BUF_MAX_CAP: usize = 2 * AUDIO_BUF_TARGET_SIZE;

fn main() {
    let opt = Opt::from_args();

    // Read the ROM file
    let rom_path = opt.rom_path;
    let save_path = opt.save_path
        .unwrap_or_else(|| rom_path.with_extension("sav"));
    let mut reader = BufReader::new(File::open(rom_path).expect("Couldn't open ROM file"));
    let mut rom = Vec::new();
    reader
        .read_to_end(&mut rom)
        .expect("Couldn't read ROM file");
    drop(reader);

    // Parse the game name
    let mut window_title = String::from("Game Boy - ");
    for &c in &rom[0x134..][..15] {
        if c == 0 {
            break;
        }
        window_title.push(c.into());
    }

    // Parse the RAM size
    let ram_size = match rom[0x149] {
        0 => 0,
        1 => 2 << 10,
        2 => 8 << 10,
        3 => 32 << 10,
        4 => 128 << 10,
        5 => 64 << 10,
        _ => panic!("Invalid RAM size"),
    };

    // Set up the window
    let mut buffer = [0; FRAME];
    let mut window = Window::new(
        &window_title,
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..Default::default()
        },
    ).unwrap();

    // Set up the audio
    let event_loop = cpal::EventLoop::new();
    let device = cpal::default_output_device().expect("No audio device available");
    let audio_stream = event_loop
        .build_output_stream(
            &device,
            &cpal::Format {
                channels: 2,
                sample_rate: cpal::SampleRate(SAMPLE_RATE),
                data_type: cpal::SampleFormat::F32,
            },
        )
        .unwrap();
    event_loop.play_stream(audio_stream);
    let audio_buf = Arc::new(Mutex::new(VecDeque::<u8>::with_capacity(AUDIO_BUF_MAX_CAP)));
    let audio_buf_clone = audio_buf.clone();

    thread::spawn(move || {
        event_loop.run(|_, mut stream_data| {
            if let cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(ref mut dst),
            } = stream_data
            {
                let mut src = audio_buf_clone.lock().unwrap();
                let mut len = src.len();
                let dst: &mut [f32] = &mut *dst;
                if dst.len() < len {
                    len = dst.len();
                }
                for (dst, src) in dst.iter_mut().zip(src.drain(..len)) {
                    *dst = (src as f32 - 1.0) / 127.0 - 1.0;
                }
            }
        });
    });

    // Create the WebAssembly instance
    let mut wasmboy = Instance::new((), Vec::new());

    // Load the ROM
    wasmboy.context.memory[ROM_BASE..][..rom.len()].copy_from_slice(&rom);
    drop(rom);

    // Try to load the Cartridge RAM
    File::open(&save_path)
        .and_then(|mut f| {
            f.read_exact(&mut wasmboy.context.memory[CARTRIDGE_RAM_BASE..][..ram_size])
        })
        .ok();

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
        assert!(response > 0, "Wasmboy crashed!");

        // Update the audio buffer
        let audio_queue_index = wasmboy.getAudioQueueIndex();
        let audio_buf_fill;
        let buf_len = 2 * audio_queue_index as usize;
        {
            let mut audio_buf = audio_buf.lock().unwrap();
            audio_buf.extend(&wasmboy.context.memory[AUDIO_BASE..][..buf_len]);
            let len = audio_buf.len();
            if len > AUDIO_BUF_MAX_CAP {
                audio_buf.drain(..len - AUDIO_BUF_MAX_CAP);
            }
            audio_buf_fill = audio_buf.len() as f32 / AUDIO_BUF_TARGET_SIZE as f32;
        }
        wasmboy.resetAudioQueue();

        // Copy the frame
        for (dst, &src) in buffer.iter_mut().zip(&wasmboy.context.memory[FRAME_BASE..]) {
            *dst = match src {
                1 => 0xFF_FF_FF_FF,
                2 => 0xFF_D3_D3_D3,
                3 => 0xFF_A9_A9_A9,
                _ => 0xFF_00_00_00,
            };
        }

        // Show the frame
        window.update_with_buffer(&buffer).unwrap();

        if !window.is_key_down(Key::D) {
            // Sleep a bit based on the fill of the audio buffer
            let sleep_time =
                time::Duration::new(0, ((1_000_000_000.0 / 60.0) * audio_buf_fill) as u32);
            thread::sleep(sleep_time);
        }
    }

    // Save the Cartridge RAM
    let mut file = File::create(save_path).expect("Couldn't create Cartridge RAM file");
    file.write(&wasmboy.context.memory[CARTRIDGE_RAM_BASE..][..ram_size])
        .expect("Couldn't save Cartridge RAM file");
}

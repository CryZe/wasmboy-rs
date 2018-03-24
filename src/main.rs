// #![windows_subsystem = "windows"]

extern crate cpal;
extern crate gilrs;
extern crate minifb;
#[macro_use]
extern crate structopt;
extern crate wasmboy;

use minifb::{Key, Scale, Window, WindowOptions};
use std::{thread, time};
use std::collections::VecDeque;
use std::io::{BufReader, Read, Write};
use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use wasmboy::Instance;
use wasmboy::consts::*;
use gilrs::{Axis, Button, Gilrs};

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "save-path", parse(from_os_str))]
    save_path: Option<PathBuf>,
    #[structopt(parse(from_os_str))]
    rom_path: PathBuf,
}

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

    // Set up gamepads
    let mut gilrs = Gilrs::new().unwrap();

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
                    let mut val = (src as f32 - 1.0) / 127.0 - 1.0;
                    if val.abs() < 0.008 {
                        // Should probably be 0
                        val = 0.0;
                    }
                    // Volume, it's way too loud
                    *dst = val / 2.5;
                }
            }
        });
    });

    // Create the WebAssembly instance
    let mut wasmboy = Instance::new((), Vec::new());

    // Configure the instance
    let audio_batch_processing = 1;
    let graphics_batch_processing = 1;
    let timers_batch_processing = 1;
    wasmboy.config(
        audio_batch_processing,
        graphics_batch_processing,
        timers_batch_processing,
    );

    // Load the ROM
    wasmboy.context.memory[ROM_BASE..][..rom.len()].copy_from_slice(&rom);
    drop(rom);

    // Initialize the emulator
    wasmboy.initialize(0);

    // Try to load the Cartridge RAM
    File::open(&save_path)
        .and_then(|mut f| {
            f.read_exact(&mut wasmboy.context.memory[CARTRIDGE_RAM_BASE..][..ram_size])
        })
        .ok();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update Joypad States
        while let Some(_) = gilrs.next_event() {}

        let button_up = (window.is_key_down(Key::Up) || gilrs.gamepads().any(|(_, g)| {
            g.is_pressed(Button::DPadUp)
                || g.axis_data(Axis::LeftStickY)
                    .map_or(false, |c| c.value() > 0.5)
        })) as i32;
        let button_right = (window.is_key_down(Key::Right) || gilrs.gamepads().any(|(_, g)| {
            g.is_pressed(Button::DPadRight)
                || g.axis_data(Axis::LeftStickX)
                    .map_or(false, |c| c.value() > 0.5)
        })) as i32;
        let button_down = (window.is_key_down(Key::Down) || gilrs.gamepads().any(|(_, g)| {
            g.is_pressed(Button::DPadDown)
                || g.axis_data(Axis::LeftStickY)
                    .map_or(false, |c| c.value() < -0.5)
        })) as i32;
        let button_left = (window.is_key_down(Key::Left) || gilrs.gamepads().any(|(_, g)| {
            g.is_pressed(Button::DPadLeft)
                || g.axis_data(Axis::LeftStickX)
                    .map_or(false, |c| c.value() < -0.5)
        })) as i32;
        let button_a = (window.is_key_down(Key::S)
            || gilrs.gamepads().any(|(_, g)| g.is_pressed(Button::East)))
            as i32;
        let button_b = (window.is_key_down(Key::A)
            || gilrs.gamepads().any(|(_, g)| g.is_pressed(Button::South)))
            as i32;
        let button_select = (window.is_key_down(Key::Q)
            || gilrs.gamepads().any(|(_, g)| g.is_pressed(Button::Select)))
            as i32;
        let button_start = (window.is_key_down(Key::W)
            || gilrs.gamepads().any(|(_, g)| g.is_pressed(Button::Start)))
            as i32;

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
                2 => 0xFF_A0_A0_A0,
                3 => 0xFF_58_58_58,
                _ => 0xFF_00_00_00,
            };
        }

        // Show the frame
        window.update_with_buffer(&buffer).unwrap();

        if !window.is_key_down(Key::D)
            && gilrs
                .gamepads()
                .all(|(_, g)| !g.is_pressed(Button::RightTrigger))
        {
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

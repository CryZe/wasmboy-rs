#![feature(test)]

extern crate test;
extern crate wasmboy;

use test::Bencher;

use wasmboy::{consts::*, Context, Imports, Instance};

static BACK_TO_COLOR: &[u8] = include_bytes!("back-to-color.gbc");
static TOBU_TOBU_GIRL: &[u8] = include_bytes!("tobutobugirl.gb");

struct DontPrint;

impl Imports for DontPrint {
    type Memory = Vec<u8>;
    fn log(
        &mut self,
        _context: &mut Context<Self::Memory>,
        _message: i32,
        _arg0: i32,
        _arg1: i32,
        _arg2: i32,
        _arg3: i32,
        _arg4: i32,
        _arg5: i32,
    ) {
    }
}

#[derive(Copy, Clone, Default)]
struct Config {
    audio_batch_processing: bool,
    graphics_batch_processing: bool,
    timers_batch_processing: bool,
    graphics_disable_scanline_rendering: bool,
    audio_accumulate_samples: bool,
    tile_rendering: bool,
    tile_caching: bool,
}

fn setup_wasmboy(rom: &[u8], config: Config) -> Instance<DontPrint, Vec<u8>> {
    let mut instance = Instance::new(DontPrint, Vec::new());

    instance.config(
        config.audio_batch_processing as i32,
        config.graphics_batch_processing as i32,
        config.timers_batch_processing as i32,
        config.graphics_disable_scanline_rendering as i32,
        config.audio_accumulate_samples as i32,
        config.tile_rendering as i32,
        config.tile_caching as i32,
    );

    instance.context.memory[ROM_BASE..][..rom.len()].copy_from_slice(&rom);
    instance.initialize(1, 0);

    instance
}

macro_rules! bench_single {
    ($name:ident, $game:ident, {$($option:ident),*}) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut instance = setup_wasmboy(
                $game,
                Config {
                    $(
                        $option: true,
                    )*
                    ..Default::default()
                },
            );

            b.iter(|| {
                instance.update();
            })
        }
    };
}

macro_rules! bench {
    ($($game:ident = [$($name:ident => $option:tt),*]),*) => {
        $(
            #[allow(non_snake_case)]
            mod $game {
                use super::*;

                $(
                    bench_single!($name, $game, $option);
                )*
            }
        )*
    }
}

bench! {
    BACK_TO_COLOR = [
        no_performance_options => { },
        all => {
            audio_batch_processing,
            graphics_batch_processing,
            timers_batch_processing,
            graphics_disable_scanline_rendering,
            audio_accumulate_samples,
            tile_rendering,
            tile_caching
        },
        audio_batch_processing => { audio_batch_processing },
        graphics_batch_processing => { graphics_batch_processing },
        timers_batch_processing => { timers_batch_processing },
        graphics_disable_scanline_rendering => { graphics_disable_scanline_rendering },
        audio_accumulate_samples => { audio_accumulate_samples },
        tile_rendering => { tile_rendering },
        tile_caching => { tile_caching }
    ],

    TOBU_TOBU_GIRL = [
        no_performance_options => { },
        all => {
            audio_batch_processing,
            graphics_batch_processing,
            timers_batch_processing,
            graphics_disable_scanline_rendering,
            audio_accumulate_samples,
            tile_rendering,
            tile_caching
        },
        audio_batch_processing => { audio_batch_processing },
        graphics_batch_processing => { graphics_batch_processing },
        timers_batch_processing => { timers_batch_processing },
        graphics_disable_scanline_rendering => { graphics_disable_scanline_rendering },
        audio_accumulate_samples => { audio_accumulate_samples },
        tile_rendering => { tile_rendering },
        tile_caching => { tile_caching }
    ]
}

#![allow(
    unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case,
    non_upper_case_globals, unconditional_recursion, path_statements
)]

pub const PAGE_SIZE: usize = 64 << 10;

pub trait Imports {
    type Memory: Memory;
    fn log(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32);
}

pub trait Memory {
    fn load8(&mut self, addr: usize) -> u8;
    fn load16(&mut self, addr: usize) -> u16;
    fn load32(&mut self, addr: usize) -> u32;
    fn load64(&mut self, addr: usize) -> u64;

    fn store8(&mut self, addr: usize, val: u8);
    fn store16(&mut self, addr: usize, val: u16);
    fn store32(&mut self, addr: usize, val: u32);
    fn store64(&mut self, addr: usize, val: u64);

    fn store_slice(&mut self, addr: usize, val: &'static [u8]);

    fn grow(&mut self, pages: usize) -> i32;
    fn size(&mut self) -> i32;
}

pub struct Instance<I: Imports<Memory = M>, M: Memory> {
    pub imports: I,
    pub context: Context<M>,
}

pub struct Context<M: Memory> {
    pub memory: M,
    global14: i32,
    global15: i32,
    global16: i32,
    global17: i32,
    global18: i32,
    global19: i32,
    global20: i32,
    global21: i32,
    global22: i32,
    global23: i32,
    global24: i32,
    global25: i32,
    global26: i32,
    global27: i32,
    global28: i32,
    global29: i32,
    global30: i32,
    global31: i32,
    global32: i32,
    global33: i32,
    global34: i32,
    global35: i32,
    global36: i32,
    global37: i32,
    global38: i32,
    global39: i32,
    global40: i32,
    global41: i32,
    global42: i32,
    global43: i32,
    global44: i32,
    global45: i32,
    global46: i32,
    global47: i32,
    global48: i32,
    global49: i32,
    global50: i32,
    global51: i32,
    global52: i32,
    global53: i32,
    global54: i32,
    global55: i32,
    global56: i32,
    global57: i32,
    global58: i32,
    global59: i32,
    global60: i32,
    global61: i32,
    global62: i32,
    global63: i32,
    global64: i32,
    global65: i32,
    global66: i32,
    global67: i32,
    global68: i32,
    global69: i32,
    global70: i32,
    global71: i32,
    global72: i32,
    global73: i32,
    global74: i32,
    global75: i32,
    global76: i32,
    global77: i32,
    global78: i32,
    global79: i32,
    global80: i32,
    global81: i32,
    global82: i32,
    global83: i32,
    global84: i32,
    global85: i32,
    global86: i32,
    global87: i32,
    global88: i32,
    global89: i32,
    global90: i32,
    global91: i32,
    global92: i32,
    global93: i32,
    global94: i32,
    global95: i32,
    global96: i32,
    global97: i32,
    global98: i32,
    global99: i32,
    global100: i32,
    global101: i32,
    global102: i32,
    global103: i32,
    global104: i32,
    global105: i32,
    global106: i32,
    global107: i32,
    global108: i32,
    global109: i32,
    global110: i32,
    global111: i32,
    global112: i32,
    global113: i32,
    global114: i32,
    global115: i32,
    global116: i32,
    global117: i32,
    global118: i32,
    global119: i32,
    global120: i32,
    global121: i32,
    global122: i32,
    global123: i32,
    global124: i32,
    global125: i32,
    global126: i32,
    global127: i32,
    global128: i32,
    global129: i32,
    global130: i32,
    global131: i32,
    global132: i32,
    global133: i32,
    global134: i32,
    global135: i32,
    global136: i32,
    global137: i32,
    global138: i32,
    global139: i32,
    global140: i32,
    global141: i32,
    global142: i32,
    global143: i32,
    global144: i32,
    global145: i32,
    global146: i32,
    global147: i32,
    global148: i32,
    global149: i32,
    global150: i32,
    global151: i32,
    global152: i32,
    global153: i32,
    global154: i32,
    global155: i32,
    global156: i32,
    global157: i32,
    global158: i32,
    global159: i32,
    global160: i32,
    global161: i32,
    global162: i32,
    global163: i32,
    global164: i32,
    global165: i32,
    global166: i32,
    global167: i32,
    global168: i32,
    global169: i32,
    global170: i32,
    global171: i32,
    global172: i32,
    global173: i32,
    global174: i32,
    global175: i32,
    global176: i32,
    global177: i32,
    global178: i32,
    global179: i32,
    global180: i32,
    global181: i32,
    global182: i32,
    global183: i32,
    global184: i32,
    global185: i32,
    global186: i32,
    global187: i32,
    global188: i32,
    global189: i32,
    global190: i32,
    global191: i32,
    global192: i32,
    global193: i32,
    global194: i32,
    global195: i32,
    global196: i32,
    global197: i32,
    global198: i32,
    global199: i32,
    global200: i32,
    global201: i32,
    global202: i32,
    global203: i32,
    global204: i32,
    global205: i32,
    global206: i32,
    global207: i32,
    global208: i32,
    global209: i32,
    global210: i32,
    global211: i32,
    global212: i32,
    global213: i32,
    global214: i32,
    global215: i32,
}

pub mod consts {
    pub const wasmMemorySize: i32 = 9109504;
    pub const wasmBoyInternalStateLocation: i32 = 1024;
    pub const wasmBoyInternalStateSize: i32 = 1024;
    pub const gameBoyInternalMemoryLocation: i32 = 2048;
    pub const gameBoyInternalMemorySize: i32 = 65535;
    pub const videoOutputLocation: i32 = 67584;
    pub const gameboyColorPaletteLocation: i32 = 67584;
    pub const gameboyColorPaletteSize: i32 = 512;
    pub const frameInProgressVideoOutputLocation: i32 = 93184;
    pub const backgroundMapLocation: i32 = 232448;
    pub const tileDataMap: i32 = 429056;
    pub const soundOutputLocation: i32 = 588800;
    pub const gameRamBanksLocation: i32 = 719872;
    pub const gameBytesLocation: i32 = 850944;
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        let current_pages = memory.size() as usize;
        if current_pages < 1 {
            memory.grow(1 - current_pages);
            assert_eq!(memory.size(), 1, "Not enough memory pages allocated");
        }
        memory.store_slice(4, b" \0\0\0i\0n\0i\0t\0i\0a\0l\0i\0z\0i\0n\0g\0 \0(\0i\0n\0c\0l\0u\0d\0e\0B\0o\0o\0t\0R\0o\0m\0=\0$\00\0)");
        let mut instance = Self {
            imports,
            context: Context {
                memory,
                global14: 0,
                global15: 0,
                global16: 0,
                global17: 0,
                global18: 0,
                global19: 0,
                global20: 0,
                global21: 0,
                global22: 0,
                global23: 0,
                global24: 0,
                global25: 0,
                global26: 0,
                global27: 0,
                global28: 0,
                global29: 0,
                global30: 0,
                global31: 0,
                global32: 0,
                global33: 0,
                global34: 0,
                global35: 0,
                global36: 0,
                global37: 65359,
                global38: 0,
                global39: 65392,
                global40: 0,
                global41: 0,
                global42: 0,
                global43: 0,
                global44: 0,
                global45: 0,
                global46: 0,
                global47: 0,
                global48: 0,
                global49: 0,
                global50: 0,
                global51: 0,
                global52: 0,
                global53: 0,
                global54: 0,
                global55: 1,
                global56: 0,
                global57: 0,
                global58: 0,
                global59: 0,
                global60: 0,
                global61: 0,
                global62: 0,
                global63: 0,
                global64: 0,
                global65: 0,
                global66: 0,
                global67: 256,
                global68: 0,
                global69: 0,
                global70: 0,
                global71: 0,
                global72: 1,
                global73: 0,
                global74: 0,
                global75: 0,
                global76: 0,
                global77: 0,
                global78: 0,
                global79: 0,
                global80: 0,
                global81: 0,
                global82: 0,
                global83: 0,
                global84: 0,
                global85: 0,
                global86: 0,
                global87: 0,
                global88: 0,
                global89: 0,
                global90: 0,
                global91: 0,
                global92: 0,
                global93: 0,
                global94: 0,
                global95: 0,
                global96: 0,
                global97: 0,
                global98: 0,
                global99: 0,
                global100: 0,
                global101: 0,
                global102: 0,
                global103: 0,
                global104: 0,
                global105: 0,
                global106: 0,
                global107: 0,
                global108: 0,
                global109: 0,
                global110: 0,
                global111: 0,
                global112: 0,
                global113: 0,
                global114: 0,
                global115: 0,
                global116: 0,
                global117: 0,
                global118: 0,
                global119: 0,
                global120: 0,
                global121: 0,
                global122: 0,
                global123: 0,
                global124: 0,
                global125: 0,
                global126: 15,
                global127: 0,
                global128: 0,
                global129: 15,
                global130: 0,
                global131: 0,
                global132: 0,
                global133: 15,
                global134: 0,
                global135: 0,
                global136: 0,
                global137: 15,
                global138: 0,
                global139: 0,
                global140: 0,
                global141: 0,
                global142: 0,
                global143: 0,
                global144: 48000,
                global145: 1,
                global146: 1,
                global147: 1,
                global148: 1,
                global149: 1,
                global150: 1,
                global151: 1,
                global152: 1,
                global153: 0,
                global154: 0,
                global155: 127,
                global156: 127,
                global157: 131072,
                global158: 1,
                global159: 0,
                global160: 0,
                global161: 0,
                global162: 0,
                global163: 0,
                global164: 0,
                global165: 0,
                global166: 0,
                global167: 0,
                global168: 0,
                global169: 0,
                global170: 0,
                global171: 0,
                global172: 1,
                global173: 0,
                global174: 0,
                global175: 0,
                global176: 0,
                global177: 0,
                global178: 0,
                global179: 0,
                global180: 0,
                global181: 0,
                global182: 0,
                global183: 0,
                global184: 65365,
                global185: 0,
                global186: 65361,
                global187: 65362,
                global188: 65363,
                global189: 65364,
                global190: 0,
                global191: 0,
                global192: 0,
                global193: 0,
                global194: 65384,
                global195: 65387,
                global196: 65385,
                global197: 0,
                global198: 0,
                global199: 0,
                global200: 0,
                global201: 0,
                global202: 0,
                global203: 0,
                global204: 0,
                global205: 0,
                global206: 0,
                global207: 0,
                global208: 0,
                global209: 0,
                global210: 0,
                global211: -1,
                global212: -1,
                global213: 0,
                global214: 0,
                global215: 0,
            },
        };
        instance.context.start(&mut instance.imports);
        instance
    }
    pub fn config(&mut self, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32) {
        self.context.wasm_config_config(&mut self.imports, var0, var1, var2, var3, var4, var5, var6)
    }
    pub fn setJoypadState(&mut self, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.context.wasm_joypad_index_setJoypadState(&mut self.imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex(&mut self) -> i32 {
        self.context.wasm_sound_sound_getAudioQueueIndex(&mut self.imports)
    }
    pub fn resetAudioQueue(&mut self) {
        self.context.wasm_sound_sound_resetAudioQueue(&mut self.imports)
    }
    pub fn getWasmBoyOffsetFromGameBoyOffset(&mut self, var0: i32) -> i32 {
        self.context.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(&mut self.imports, var0)
    }
    pub fn getRegisterA(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterA(&mut self.imports)
    }
    pub fn getRegisterB(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterB(&mut self.imports)
    }
    pub fn getRegisterC(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterC(&mut self.imports)
    }
    pub fn getRegisterD(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterD(&mut self.imports)
    }
    pub fn getRegisterE(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterE(&mut self.imports)
    }
    pub fn getRegisterH(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterH(&mut self.imports)
    }
    pub fn getRegisterL(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterL(&mut self.imports)
    }
    pub fn getRegisterF(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getRegisterF(&mut self.imports)
    }
    pub fn getProgramCounter(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getProgramCounter(&mut self.imports)
    }
    pub fn getStackPointer(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getStackPointer(&mut self.imports)
    }
    pub fn getOpcodeAtProgramCounter(&mut self) -> i32 {
        self.context.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(&mut self.imports)
    }
    pub fn drawBackgroundMapToWasmMemory(&mut self, var0: i32) {
        self.context.wasm_debug_debug_graphics_drawBackgroundMapToWasmMemory(&mut self.imports, var0)
    }
    pub fn drawTileDataToWasmMemory(&mut self) {
        self.context.wasm_debug_debug_graphics_drawTileDataToWasmMemory(&mut self.imports)
    }
    pub fn hasCoreStarted(&mut self) -> i32 {
        self.context.wasm_index_hasCoreStarted(&mut self.imports)
    }
    pub fn initialize(&mut self, var0: i32, var1: i32) {
        self.context.wasm_index_initialize(&mut self.imports, var0, var1)
    }
    pub fn emulationStep(&mut self) -> i32 {
        self.context.wasm_index_emulationStep(&mut self.imports)
    }
    pub fn update(&mut self) -> i32 {
        self.context.wasm_index_update(&mut self.imports)
    }
    pub fn saveState(&mut self) {
        self.context.wasm_index_saveState(&mut self.imports)
    }
    pub fn loadState(&mut self) {
        self.context.wasm_index_loadState(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn config<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32) {
        self.wasm_config_config(imports, var0, var1, var2, var3, var4, var5, var6)
    }
    pub fn setJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.wasm_joypad_index_setJoypadState(imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_sound_sound_getAudioQueueIndex(imports)
    }
    pub fn resetAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_sound_sound_resetAudioQueue(imports)
    }
    pub fn getWasmBoyOffsetFromGameBoyOffset<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32) -> i32 {
        self.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(imports, var0)
    }
    pub fn getRegisterA<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterA(imports)
    }
    pub fn getRegisterB<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterB(imports)
    }
    pub fn getRegisterC<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterC(imports)
    }
    pub fn getRegisterD<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterD(imports)
    }
    pub fn getRegisterE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterE(imports)
    }
    pub fn getRegisterH<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterH(imports)
    }
    pub fn getRegisterL<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterL(imports)
    }
    pub fn getRegisterF<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getRegisterF(imports)
    }
    pub fn getProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getProgramCounter(imports)
    }
    pub fn getStackPointer<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getStackPointer(imports)
    }
    pub fn getOpcodeAtProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports)
    }
    pub fn drawBackgroundMapToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32) {
        self.wasm_debug_debug_graphics_drawBackgroundMapToWasmMemory(imports, var0)
    }
    pub fn drawTileDataToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_debug_debug_graphics_drawTileDataToWasmMemory(imports)
    }
    pub fn hasCoreStarted<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_index_hasCoreStarted(imports)
    }
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32) {
        self.wasm_index_initialize(imports, var0, var1)
    }
    pub fn emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_index_emulationStep(imports)
    }
    pub fn update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_index_update(imports)
    }
    pub fn saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_index_saveState(imports)
    }
    pub fn loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_index_loadState(imports)
    }
    fn wasm_config_config<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) {
        if (var0 > 0i32) as i32 != 0 {
            self.global14 = 1i32;
        } else {
            self.global14 = 0i32;
        }
        if (var1 > 0i32) as i32 != 0 {
            self.global15 = 1i32;
        } else {
            self.global15 = 0i32;
        }
        if (var2 > 0i32) as i32 != 0 {
            self.global16 = 1i32;
        } else {
            self.global16 = 0i32;
        }
        if (var3 > 0i32) as i32 != 0 {
            self.global17 = 1i32;
        } else {
            self.global17 = 0i32;
        }
        if (var4 > 0i32) as i32 != 0 {
            self.global18 = 1i32;
        } else {
            self.global18 = 0i32;
        }
        if (var5 > 0i32) as i32 != 0 {
            self.global19 = 1i32;
        } else {
            self.global19 = 0i32;
        }
        if (var6 > 0i32) as i32 != 0 {
            self.global20 = 1i32;
        } else {
            self.global20 = 0i32;
        }
    }
    fn wasm_joypad_index__getJoypadButtonStateFromButtonId<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                match var0 {
                                                    0 => break 'label8,
                                                    1 => break 'label7,
                                                    2 => break 'label6,
                                                    3 => break 'label5,
                                                    4 => break 'label4,
                                                    5 => break 'label3,
                                                    6 => break 'label2,
                                                    7 => break 'label1,
                                                    _ => break 'label9,
                                                }
                                                break;
                                            }
                                            break 'label0;
                                            break;
                                        }
                                        let var1 = self.global22;
                                        return var1;
                                        break;
                                    }
                                    let var2 = self.global23;
                                    return var2;
                                    break;
                                }
                                let var3 = self.global24;
                                return var3;
                                break;
                            }
                            let var4 = self.global25;
                            return var4;
                            break;
                        }
                        let var5 = self.global26;
                        return var5;
                        break;
                    }
                    let var6 = self.global27;
                    return var6;
                    break;
                }
                let var7 = self.global28;
                return var7;
                break;
            }
            let var8 = self.global29;
            return var8;
            break;
        }
        0i32
    }
    fn wasm_joypad_index__setJoypadButtonStateFromButtonId<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                match var0 {
                                                    0 => break 'label8,
                                                    1 => break 'label7,
                                                    2 => break 'label6,
                                                    3 => break 'label5,
                                                    4 => break 'label4,
                                                    5 => break 'label3,
                                                    6 => break 'label2,
                                                    7 => break 'label1,
                                                    _ => break 'label9,
                                                }
                                                break;
                                            }
                                            break 'label0;
                                            break;
                                        }
                                        self.global22 = var1;
                                        break 'label0;
                                        break;
                                    }
                                    self.global23 = var1;
                                    break 'label0;
                                    break;
                                }
                                self.global24 = var1;
                                break 'label0;
                                break;
                            }
                            self.global25 = var1;
                            break 'label0;
                            break;
                        }
                        self.global26 = var1;
                        break 'label0;
                        break;
                    }
                    self.global27 = var1;
                    break 'label0;
                    break;
                }
                self.global28 = var1;
                break 'label0;
                break;
            }
            self.global29 = var1;
            break;
        }
    }
    fn wasm_memory_banking_getRomBankAddress<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global34;
        var1 = var3;
        let var4 = self.global35;
        var2 = (var4 == 0) as i32;
        let var5: i32;
        if var2 != 0 {
            var5 = (var1 == 0) as i32;
        } else {
            var5 = var2;
        }
        if var5 & 1i32 != 0 {
            var1 = 1i32;
        }
        var1.wrapping_mul(16384i32).wrapping_add(var0.wrapping_sub(16384i32))
    }
    fn wasm_memory_banking_getRamBankAddress<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global38;
        var1.wrapping_mul(8192i32).wrapping_add(var0.wrapping_sub(40960i32))
    }
    fn wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        match var0.wrapping_shr(12i32 as u32) {
                                            0 => break 'label6,
                                            1 => break 'label6,
                                            2 => break 'label6,
                                            3 => break 'label6,
                                            4 => break 'label5,
                                            5 => break 'label5,
                                            6 => break 'label5,
                                            7 => break 'label5,
                                            8 => break 'label4,
                                            9 => break 'label4,
                                            10 => break 'label3,
                                            11 => break 'label3,
                                            12 => break 'label2,
                                            13 => break 'label1,
                                            _ => break 'label7,
                                        }
                                        break;
                                    }
                                    break 'label0;
                                    break;
                                }
                                return var0.wrapping_add(850944i32);
                                break;
                            }
                            let var2 = self.wasm_memory_banking_getRomBankAddress(imports, var0);
                            return var2.wrapping_add(850944i32);
                            break;
                        }
                        let var3 = self.global36;
                        if var3 != 0 {
                            let var4 = self.global37;
                            let var5 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4);
                            var1 = var5 & 1i32;
                        }
                        return var0.wrapping_add(-30720i32).wrapping_add(var1.wrapping_mul(8192i32));
                        break;
                    }
                    let var6 = self.wasm_memory_banking_getRamBankAddress(imports, var0);
                    return var6.wrapping_add(719872i32);
                    break;
                }
                return var0.wrapping_add(-30720i32);
                break;
            }
            let var7 = self.global36;
            if var7 != 0 {
                let var8 = self.global39;
                let var9 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var8);
                var1 = var9 & 7i32;
            }
            if (var1 < 1i32) as i32 != 0 {
                var1 = 1i32;
            }
            return var0.wrapping_add(var1.wrapping_mul(4096i32)).wrapping_add(-34816i32);
            break;
        }
        var0.wrapping_add(-6144i32)
    }
    fn wasm_memory_load_eightBitLoadFromGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(imports, var0);
        let var2 = self.memory.load8(var1 as usize) as i32;
        var2
    }
    fn wasm_helpers_index_setBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1 | 1i32.wrapping_shl(var0 as u32)
    }
    fn wasm_memory_store_eightBitStoreIntoGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(imports, var0);
        self.memory.store8(var2 as usize, var1 as u8);
    }
    fn wasm_interrupts_index__requestInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65295i32);
        let var3 = self.wasm_helpers_index_setBitOnByte(imports, var0, var2);
        var1 = var3;
        self.global40 = var1;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65295i32, var1);
    }
    fn wasm_interrupts_index_requestJoypadInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global33 = 1i32;
        self.wasm_interrupts_index__requestInterrupt(imports, 4i32);
    }
    fn wasm_joypad_index__pressJoypadButton<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        self.global21 = 0i32;
        let var2 = self.wasm_joypad_index__getJoypadButtonStateFromButtonId(imports, var0);
        if (var2 == 0) as i32 != 0 {
            var1 = 1i32;
        }
        self.wasm_joypad_index__setJoypadButtonStateFromButtonId(imports, var0, 1i32);
        if var1 != 0 {
            var1 = 0i32;
            if (var0 <= 3i32) as i32 != 0 {
                var1 = 1i32;
            }
            var0 = 0i32;
            let var3 = self.global31;
            let var4: i32;
            if var3 != 0 {
                var4 = var1;
            } else {
                let var5 = self.global31;
                var4 = var5;
            }
            if var4 & 1i32 != 0 {
                var0 = 1i32;
            }
            let var6 = self.global32;
            let var7: i32;
            if var6 != 0 {
                var7 = (var1 == 0) as i32;
            } else {
                let var8 = self.global32;
                var7 = var8;
            }
            if var7 & 1i32 != 0 {
                var0 = 1i32;
            }
            if var0 != 0 {
                self.wasm_interrupts_index_requestJoypadInterrupt(imports);
            }
        }
    }
    fn wasm_joypad_index__releaseJoypadButton<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.wasm_joypad_index__setJoypadButtonStateFromButtonId(imports, var0, 0i32);
    }
    fn wasm_joypad_index_setJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32) {
        if (var0 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 0i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 0i32);
        }
        if (var1 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 1i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 1i32);
        }
        if (var2 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 2i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 2i32);
        }
        if (var3 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 3i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 3i32);
        }
        if (var4 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 4i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 4i32);
        }
        if (var5 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 5i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 5i32);
        }
        if (var6 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 6i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 6i32);
        }
        if (var7 > 0i32) as i32 != 0 {
            self.wasm_joypad_index__pressJoypadButton(imports, 7i32);
        } else {
            self.wasm_joypad_index__releaseJoypadButton(imports, 7i32);
        }
    }
    fn wasm_sound_sound_getAudioQueueIndex<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global41;
        var0
    }
    fn wasm_sound_sound_resetAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global41 = 0i32;
    }
    fn wasm_debug_debug_cpu_getRegisterA<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global42;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterB<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterC<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global44;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterD<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global45;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global46;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterH<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global47;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterL<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global48;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterF<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global49;
        var0
    }
    fn wasm_debug_debug_cpu_getProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global50;
        var0
    }
    fn wasm_debug_debug_cpu_getStackPointer<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global51;
        var0
    }
    fn wasm_debug_debug_cpu_getOpcodeAtProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global50;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
        var1 & 255i32
    }
    fn wasm_memory_memory_loadFromVramBank<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = self.memory.load8(var0.wrapping_add(-30720i32).wrapping_add((var1 & 1i32).wrapping_mul(8192i32)) as usize) as i32;
        var2
    }
    fn wasm_helpers_index_checkBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var1 & 1i32.wrapping_shl(var0 as u32) != 0i32) as i32
    }
    fn wasm_graphics_renderUtils_getTileDataAddress<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        if (var0 == 34816i32) as i32 != 0 {
            var2 = var1.wrapping_add(128i32);
            let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
            if var3 != 0 {
                var2 = var1.wrapping_sub(128i32);
            }
            return var0.wrapping_add(var2.wrapping_mul(16i32));
        }
        var0.wrapping_add(var1.wrapping_mul(16i32))
    }
    fn wasm_memory_memory_loadPaletteByteFromWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = var0 & 63i32;
        if var1 != 0 {
            var2 = var2.wrapping_add(64i32);
        }
        let var3 = self.memory.load8(var2.wrapping_add(67584i32) as usize) as i32;
        var3
    }
    fn wasm_helpers_index_concatenateBytes<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var0 & 255i32).wrapping_shl(8i32 as u32) | var1 & 255i32
    }
    fn wasm_graphics_palette_getRgbColorFromPalette<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        var3 = var0.wrapping_mul(8i32).wrapping_add(var1.wrapping_mul(2i32));
        let var4 = self.wasm_memory_memory_loadPaletteByteFromWasmMemory(imports, var3.wrapping_add(1i32), var2);
        let var5 = self.wasm_memory_memory_loadPaletteByteFromWasmMemory(imports, var3, var2);
        let var6 = self.wasm_helpers_index_concatenateBytes(imports, var4, var5);
        var6
    }
    fn wasm_graphics_palette_getColorComponentFromRgb<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var1 & 31i32.wrapping_shl(var0.wrapping_mul(5i32) as u32)).wrapping_shr(var0.wrapping_mul(5i32) as u32).wrapping_mul(8i32)
    }
    fn wasm_graphics_palette_getMonochromeColorFromPalette<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        if (var2 == 0) as i32 != 0 {
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var1);
            var0 = var3.wrapping_shr(var0.wrapping_mul(2i32) as u32) & 3i32;
        }
        var1 = 242i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            match var0 {
                                0 => break 'label0,
                                1 => break 'label3,
                                2 => break 'label2,
                                3 => break 'label1,
                                _ => break 'label4,
                            }
                            break;
                        }
                        break 'label0;
                        break;
                    }
                    var1 = 160i32;
                    break 'label0;
                    break;
                }
                var1 = 88i32;
                break 'label0;
                break;
            }
            var1 = 8i32;
            break;
        }
        var1
    }
    fn wasm_debug_debug_graphics_drawBackgroundMapToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        var8 = 34816i32;
        let var12 = self.global52;
        if var12 != 0 {
            var8 = 32768i32;
        }
        var9 = 38912i32;
        let var13 = self.global53;
        if var13 != 0 {
            var9 = 39936i32;
        }
        'label0: loop {
            if (var3 < 256i32) as i32 != 0 {
                var4 = 0i32;
                'label1: loop {
                    if (var4 < 256i32) as i32 != 0 {
                        var1 = var9.wrapping_add(var3.wrapping_shr(3i32 as u32).wrapping_mul(32i32)).wrapping_add(var4.wrapping_shr(3i32 as u32));
                        let var14 = self.wasm_memory_memory_loadFromVramBank(imports, var1, 0i32);
                        let var15 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var8, var14);
                        var10 = var15;
                        var2 = var3.wrapping_rem(8i32);
                        var6 = 7i32.wrapping_sub(var4.wrapping_rem(8i32));
                        var5 = 0i32;
                        let var16 = self.global36;
                        let var17: i32;
                        if var16 != 0 {
                            var17 = (var0 > 0i32) as i32;
                        } else {
                            let var18 = self.global36;
                            var17 = var18;
                        }
                        if var17 & 1i32 != 0 {
                            let var19 = self.wasm_memory_memory_loadFromVramBank(imports, var1, 1i32);
                            var5 = var19;
                        }
                        let var20 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var5);
                        if var20 != 0 {
                            var2 = 7i32.wrapping_sub(var2);
                        }
                        var7 = 0i32;
                        let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var5);
                        if var21 != 0 {
                            var7 = 1i32;
                        }
                        let var22 = self.wasm_memory_memory_loadFromVramBank(imports, var10.wrapping_add(var2.wrapping_mul(2i32)), var7);
                        var11 = var22;
                        var1 = 0i32;
                        let var23 = self.wasm_memory_memory_loadFromVramBank(imports, var10.wrapping_add(var2.wrapping_mul(2i32)).wrapping_add(1i32), var7);
                        let var24 = self.wasm_helpers_index_checkBitOnByte(imports, var6, var23);
                        if var24 != 0 {
                            var1 = 2i32;
                        }
                        let var25 = self.wasm_helpers_index_checkBitOnByte(imports, var6, var11);
                        if var25 != 0 {
                            var1 = var1.wrapping_add(1i32);
                        }
                        var6 = var3.wrapping_mul(256i32).wrapping_add(var4).wrapping_mul(3i32);
                        let var26 = self.global36;
                        let var27: i32;
                        if var26 != 0 {
                            var27 = (var0 > 0i32) as i32;
                        } else {
                            let var28 = self.global36;
                            var27 = var28;
                        }
                        if var27 & 1i32 != 0 {
                            let var29 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var5 & 7i32, var1, 0i32);
                            var1 = var29;
                            let var30 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var1);
                            var5 = var30;
                            let var31 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var1);
                            var7 = var31;
                            let var32 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var1);
                            var2 = var32;
                            var1 = var6.wrapping_add(232448i32);
                            self.memory.store8(var1 as usize, var5 as u8);
                            self.memory.store8(var1.wrapping_add(1i32) as usize, var7 as u8);
                            self.memory.store8(var1.wrapping_add(2i32) as usize, var2 as u8);
                        } else {
                            let var33 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var1, 65351i32, 0i32);
                            var1 = var33;
                            var2 = 0i32;
                            'label2: loop {
                                if (var2 < 3i32) as i32 != 0 {
                                    self.memory.store8(var6.wrapping_add(232448i32).wrapping_add(var2) as usize, var1 as u8);
                                    var2 = var2.wrapping_add(1i32);
                                    continue 'label2;
                                }
                                break;
                            }
                        }
                        var4 = var4.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                var3 = var3.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_tiles_getTilePixelStart<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        var1.wrapping_mul(var2).wrapping_add(var0).wrapping_mul(3i32)
    }
    fn wasm_graphics_priority_getPixelStart<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1.wrapping_mul(160i32).wrapping_add(var0)
    }
    fn wasm_graphics_priority_addPriorityforPixel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) {
        let mut var4: i32 = 0;
        var4 = var2 & 3i32;
        if var3 != 0 {
            let var5 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var4);
            var4 = var5;
        }
        let var6 = self.wasm_graphics_priority_getPixelStart(imports, var0, var1);
        self.memory.store8(var6.wrapping_add(69632i32) as usize, var4 as u8);
    }
    fn wasm_graphics_tiles_drawPixelsFromLineOfTile<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32, mut var8: i32, mut var9: i32, mut var10: i32, mut var11: i32, mut var12: i32) -> i32 {
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        let mut var16: i32 = 0;
        let mut var17: i32 = 0;
        let mut var18: i32 = 0;
        let var19 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var1, var0);
        var0 = var19;
        let var20 = self.wasm_memory_memory_loadFromVramBank(imports, var0.wrapping_add(var5.wrapping_mul(2i32)), var2);
        var17 = var20;
        let var21 = self.wasm_memory_memory_loadFromVramBank(imports, var0.wrapping_add(var5.wrapping_mul(2i32)).wrapping_add(1i32), var2);
        var18 = var21;
        var0 = var3;
        'label0: loop {
            if (var0 <= var4) as i32 != 0 {
                var15 = var6.wrapping_add(var0.wrapping_sub(var3));
                if (var15 < var8) as i32 != 0 {
                    var1 = var0;
                    var13 = (var12 < 0i32) as i32;
                    let var22: i32;
                    if var13 != 0 {
                        var22 = var13;
                    } else {
                        let var23 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var12);
                        var22 = (var23 == 0) as i32;
                    }
                    if var22 & 1i32 != 0 {
                        var1 = 7i32.wrapping_sub(var1);
                    }
                    var13 = 0i32;
                    let var24 = self.wasm_helpers_index_checkBitOnByte(imports, var1, var18);
                    if var24 != 0 {
                        var13 = 2i32;
                    }
                    let var25 = self.wasm_helpers_index_checkBitOnByte(imports, var1, var17);
                    if var25 != 0 {
                        var13 = var13.wrapping_add(1i32);
                    }
                    let var26: i32;
                    if (var12 >= 0i32) as i32 != 0 {
                        let var27 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var12 & 7i32, var13, 0i32);
                        var2 = var27;
                        let var28 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var2);
                        var16 = var28;
                        let var29 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var2);
                        var1 = var29;
                        let var30 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var2);
                        var26 = var30;
                    } else {
                        if (var11 <= 0i32) as i32 != 0 {
                            var11 = 65351i32;
                        }
                        let var31 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var13, var11, var10);
                        var2 = var31;
                        var16 = var2;
                        var1 = var2;
                        var26 = var1;
                    }
                    var5 = var26;
                    let var32 = self.wasm_graphics_tiles_getTilePixelStart(imports, var15, var7, var8);
                    var2 = var32;
                    self.memory.store8(var9.wrapping_add(var2) as usize, var16 as u8);
                    self.memory.store8(var9.wrapping_add(var2).wrapping_add(1i32) as usize, var1 as u8);
                    self.memory.store8(var9.wrapping_add(var2).wrapping_add(2i32) as usize, var5 as u8);
                    var1 = 0i32;
                    if (var12 >= 0i32) as i32 != 0 {
                        let var33 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var12);
                        var1 = var33;
                    }
                    self.wasm_graphics_priority_addPriorityforPixel(imports, var15, var7, var13, var1);
                    var14 = var14.wrapping_add(1i32);
                }
                var0 = var0.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
        var14
    }
    fn wasm_debug_debug_graphics_drawTileDataToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        'label0: loop {
            if (var2 < 23i32) as i32 != 0 {
                var0 = 0i32;
                'label1: loop {
                    if (var0 < 31i32) as i32 != 0 {
                        var4 = 0i32;
                        if (var0 > 15i32) as i32 != 0 {
                            var4 = 1i32;
                        }
                        var1 = var2;
                        if (var2 > 15i32) as i32 != 0 {
                            var1 = var1.wrapping_sub(15i32);
                        }
                        var1 = var1.wrapping_shl(4i32 as u32);
                        let var6: i32;
                        if (var0 > 15i32) as i32 != 0 {
                            var6 = var1.wrapping_add(var0.wrapping_sub(15i32));
                        } else {
                            var6 = var1.wrapping_add(var0);
                        }
                        var1 = var6;
                        var5 = 32768i32;
                        if (var2 > 15i32) as i32 != 0 {
                            var5 = 34816i32;
                        }
                        var3 = 0i32;
                        'label2: loop {
                            if (var3 < 8i32) as i32 != 0 {
                                let var7 = self.wasm_graphics_tiles_drawPixelsFromLineOfTile(imports, var1, var5, var4, 0i32, 7i32, var3, var0.wrapping_mul(8i32), var2.wrapping_mul(8i32).wrapping_add(var3), 248i32, 429056i32, 1i32, 0i32, -1i32);
                                var7;
                                var3 = var3.wrapping_add(1i32);
                                continue 'label2;
                            }
                            break;
                        }
                        var0 = var0.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                var2 = var2.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_index_hasCoreStarted<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global54;
        if var0 != 0 {
            return 1i32;
        }
        0i32
    }
    fn wasm_helpers_index_log<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) {
        imports.log(self, var0, var1, var2, var3, var4, var5, var6);
    }
    fn wasm_memory_memory_initializeCartridge<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 327i32);
        var0 = var2;
        self.global55 = 0i32;
        self.global56 = 0i32;
        self.global57 = 0i32;
        self.global58 = 0i32;
        self.global35 = 0i32;
        if var0 != 0 {
            var1 = (var0 >= 1i32) as i32;
            let var3: i32;
            if var1 != 0 {
                var3 = (var0 <= 3i32) as i32;
            } else {
                var3 = var1;
            }
            if var3 & 1i32 != 0 {
                self.global56 = 1i32;
            } else {
                var1 = (var0 >= 5i32) as i32;
                let var4: i32;
                if var1 != 0 {
                    var4 = (var0 <= 6i32) as i32;
                } else {
                    var4 = var1;
                }
                if var4 & 1i32 != 0 {
                    self.global57 = 1i32;
                } else {
                    var1 = (var0 >= 15i32) as i32;
                    let var5: i32;
                    if var1 != 0 {
                        var5 = (var0 <= 19i32) as i32;
                    } else {
                        var5 = var1;
                    }
                    if var5 & 1i32 != 0 {
                        self.global58 = 1i32;
                    } else {
                        var1 = (var0 >= 25i32) as i32;
                        let var6: i32;
                        if var1 != 0 {
                            var6 = (var0 <= 30i32) as i32;
                        } else {
                            var6 = var1;
                        }
                        if var6 & 1i32 != 0 {
                            self.global35 = 1i32;
                        }
                    }
                }
            }
        } else {
            self.global55 = 1i32;
        }
        self.global34 = 1i32;
        self.global38 = 0i32;
    }
    fn wasm_graphics_graphics_initializeGraphics<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global36;
        if var0 != 0 {
            self.global59 = 145i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65344i32, 145i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65345i32, 129i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65348i32, 144i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65351i32, 252i32);
        } else {
            self.global59 = 145i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65344i32, 145i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65345i32, 133i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65350i32, 255i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65351i32, 252i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65352i32, 255i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65353i32, 255i32);
        }
    }
    fn wasm_sound_channel1_Channel1_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65296i32, 128i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65297i32, 191i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65298i32, 243i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65299i32, 193i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65300i32, 191i32);
    }
    fn wasm_sound_channel2_Channel2_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65301i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65302i32, 63i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65303i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65304i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65305i32, 184i32);
    }
    fn wasm_sound_channel3_Channel3_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65306i32, 127i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65307i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65308i32, 159i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65309i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65310i32, 184i32);
        self.global60 = 1i32;
    }
    fn wasm_sound_channel4_Channel4_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65311i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65312i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65313i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65314i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65315i32, 191i32);
    }
    fn wasm_sound_sound_initializeSound<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_sound_channel1_Channel1_initialize(imports);
        self.wasm_sound_channel2_Channel2_initialize(imports);
        self.wasm_sound_channel3_Channel3_initialize(imports);
        self.wasm_sound_channel4_Channel4_initialize(imports);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65316i32, 119i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65317i32, 243i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65318i32, 241i32);
        self.global61 = 1i32;
        self.global62 = 1i32;
    }
    fn wasm_timers_index_getFrequencyFromInputClockSelect<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        var0 = 256i32;
        let var1 = self.global68;
        if var1 != 0 {
            var0 = 512i32;
        }
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var2 = self.global65;
                            match var2 {
                                0 => break 'label3,
                                1 => break 'label2,
                                2 => break 'label1,
                                _ => break 'label4,
                            }
                            break;
                        }
                        break 'label0;
                        break;
                    }
                    var0 = 1024i32;
                    let var3 = self.global68;
                    if var3 != 0 {
                        var0 = 2048i32;
                    }
                    return var0;
                    break;
                }
                var0 = 16i32;
                let var4 = self.global68;
                if var4 != 0 {
                    var0 = 32i32;
                }
                return var0;
                break;
            }
            var0 = 64i32;
            let var5 = self.global68;
            if var5 != 0 {
                var0 = 126i32;
            }
            return var0;
            break;
        }
        var0
    }
    fn wasm_timers_index_Timers_updateTimerControl<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global64 = var1;
        let var2 = self.global64;
        if (var2 == 0) as i32 != 0 {
            return;
        }
        self.global65 = var0 & 3i32;
        self.global66 = 0i32;
        let var3 = self.wasm_timers_index_getFrequencyFromInputClockSelect(imports);
        self.global67 = var3;
    }
    fn wasm_timers_index_initializeTimers<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global36;
        if var0 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65284i32, 47i32);
            self.global63 = 47i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65287i32, 248i32);
            self.wasm_timers_index_Timers_updateTimerControl(imports, 248i32);
        } else {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65284i32, 171i32);
            self.global63 = 171i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65287i32, 248i32);
            self.wasm_timers_index_Timers_updateTimerControl(imports, 248i32);
        }
    }
    fn wasm_cpu_cpu_initializeCpu<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 323i32);
        var3 = var4;
        var2 = (var3 == 192i32) as i32;
        let var5: i32;
        if var2 != 0 {
            var5 = var2;
        } else {
            var2 = (var0 > 0i32) as i32;
            let var6: i32;
            if var2 != 0 {
                var6 = (var3 == 128i32) as i32;
            } else {
                var6 = var2;
            }
            var5 = var6 & 1i32;
        }
        if var5 & 1i32 != 0 {
            self.global36 = 1i32;
        }
        self.wasm_helpers_index_log(imports, 4i32, 1i32, var1, -9999i32, -9999i32, -9999i32, -9999i32);
        if (var1 <= 0i32) as i32 != 0 {
            let var7 = self.global36;
            if var7 != 0 {
                self.global42 = 17i32;
                self.global49 = 128i32;
                self.global43 = 0i32;
                self.global44 = 0i32;
                self.global45 = 255i32;
                self.global46 = 86i32;
                self.global47 = 0i32;
                self.global48 = 13i32;
                self.global50 = 256i32;
                self.global51 = 65534i32;
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65392i32, 248i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65359i32, 254i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65357i32, 126i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65280i32, 207i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65282i32, 124i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65295i32, 225i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65384i32, 192i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65385i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65386i32, 193i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65387i32, 13i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65359i32, 0i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65392i32, 1i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65361i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65362i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65363i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65364i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65365i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65388i32, 254i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65397i32, 143i32);
            } else {
                self.global42 = 1i32;
                self.global49 = 176i32;
                self.global43 = 0i32;
                self.global44 = 19i32;
                self.global45 = 0i32;
                self.global46 = 216i32;
                self.global47 = 1i32;
                self.global48 = 77i32;
                self.global50 = 256i32;
                self.global51 = 65534i32;
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65392i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65359i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65357i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65280i32, 207i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65282i32, 126i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65295i32, 225i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65384i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65385i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65386i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65387i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65359i32, 0i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65392i32, 1i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65361i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65362i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65363i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65364i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65365i32, 255i32);
            }
            self.wasm_memory_memory_initializeCartridge(imports);
            self.wasm_graphics_graphics_initializeGraphics(imports);
            self.wasm_sound_sound_initializeSound(imports);
            self.wasm_timers_index_initializeTimers(imports);
        }
    }
    fn wasm_index_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        self.wasm_cpu_cpu_initializeCpu(imports, var0, var1);
        self.global54 = 0i32;
    }
    fn wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 140448i32;
        }
        70224i32
    }
    fn wasm_cpu_opcodes_getDataByteTwo<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global50;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0.wrapping_add(1i32) & 65535i32);
        var1 & 255i32
    }
    fn wasm_cpu_opcodes_getConcatenatedDataByte<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_cpu_opcodes_getDataByteTwo(imports);
        let var1 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
        let var2 = self.wasm_helpers_index_concatenateBytes(imports, var0, var1);
        var2 & 65535i32
    }
    fn wasm_helpers_index_splitHighByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        (var0 & 65280i32).wrapping_shr(8i32 as u32)
    }
    fn wasm_helpers_index_splitLowByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        var0 & 255i32
    }
    fn wasm_memory_banking_handleBanking<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global55;
        if var4 != 0 {
            return;
        }
        if (var0 <= 8191i32) as i32 != 0 {
            let var5 = self.global57;
            let var6: i32;
            if var5 != 0 {
                let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var1 & 255i32);
                var6 = (var7 == 0) as i32;
            } else {
                let var8 = self.global57;
                var6 = var8;
            }
            if (var6 & 1i32 == 0) as i32 != 0 {
                var2 = var1 & 15i32;
                if var2 != 0 {
                    if (var2 == 10i32) as i32 != 0 {
                        self.global71 = 1i32;
                    }
                } else {
                    self.global71 = 0i32;
                }
            }
        } else {
            if (var0 <= 16383i32) as i32 != 0 {
                let var9 = self.global35;
                var2 = (var9 == 0) as i32;
                let var10: i32;
                if var2 != 0 {
                    var10 = var2;
                } else {
                    var10 = (var0 <= 12287i32) as i32;
                }
                if var10 & 1i32 != 0 {
                    let var11 = self.global57;
                    if var11 != 0 {
                        self.global34 = var1 & 15i32;
                    }
                    var2 = var1;
                    let var12 = self.global56;
                    if var12 != 0 {
                        var2 = var2 & 31i32;
                        let var13 = self.global34;
                        self.global34 = var13 & 224i32;
                    } else {
                        let var14 = self.global58;
                        if var14 != 0 {
                            var2 = var2 & 127i32;
                            let var15 = self.global34;
                            self.global34 = var15 & 128i32;
                        } else {
                            let var16 = self.global35;
                            if var16 != 0 {
                                let var17 = self.global34;
                                self.global34 = var17 & 0i32;
                            }
                        }
                    }
                    let var18 = self.global34;
                    self.global34 = var18 | var2;
                } else {
                    var2 = 0i32;
                    let var19 = self.global34;
                    let var20 = self.wasm_helpers_index_splitLowByte(imports, var19);
                    var3 = var20;
                    if (var1 > 0i32) as i32 != 0 {
                        var2 = 1i32;
                    }
                    let var21 = self.wasm_helpers_index_concatenateBytes(imports, var2, var3);
                    self.global34 = var21;
                }
            } else {
                let var22 = self.global57;
                var3 = (var22 == 0) as i32;
                let var23: i32;
                if var3 != 0 {
                    var23 = (var0 <= 24575i32) as i32;
                } else {
                    var23 = var3;
                }
                if var23 & 1i32 != 0 {
                    let var24 = self.global56;
                    let var25: i32;
                    if var24 != 0 {
                        let var26 = self.global72;
                        var25 = var26;
                    } else {
                        let var27 = self.global56;
                        var25 = var27;
                    }
                    if var25 & 1i32 != 0 {
                        let var28 = self.global34;
                        self.global34 = var28 & 31i32;
                        let var29 = self.global34;
                        self.global34 = var29 | var1 & 224i32;
                        return;
                    }
                    let var30 = self.global58;
                    if var30 != 0 {
                        var3 = (var1 >= 8i32) as i32;
                        let var31: i32;
                        if var3 != 0 {
                            var31 = (var1 <= 12i32) as i32;
                        } else {
                            var31 = var3;
                        }
                        var31;
                    }
                    var3 = var1;
                    let var32 = self.global35;
                    let var33: i32;
                    if var32 != 0 {
                        var33 = var3 & 15i32;
                    } else {
                        var33 = var3 & 3i32;
                    }
                    var3 = var33;
                    self.global38 = var3;
                } else {
                    let var34 = self.global57;
                    var3 = (var34 == 0) as i32;
                    let var35: i32;
                    if var3 != 0 {
                        var35 = (var0 <= 32767i32) as i32;
                    } else {
                        var35 = var3;
                    }
                    if var35 & 1i32 != 0 {
                        let var36 = self.global56;
                        if var36 != 0 {
                            let var37 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var1 & 255i32);
                            if var37 != 0 {
                                self.global72 = 1i32;
                            } else {
                                self.global72 = 0i32;
                            }
                        }
                    }
                }
            }
        }
    }
    fn wasm_sound_sound_Sound_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 174i32;
        }
        87i32
    }
    fn wasm_sound_sound_Sound_maxFrameSequenceCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 16384i32;
        }
        8192i32
    }
    fn wasm_sound_channel1_Channel1_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global77;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.global78;
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global77;
            self.global77 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global77;
        if (var5 == 0) as i32 != 0 {
            self.global79 = 0i32;
        }
    }
    fn wasm_sound_channel2_Channel2_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global80;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.global81;
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global80;
            self.global80 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global80;
        if (var5 == 0) as i32 != 0 {
            self.global82 = 0i32;
        }
    }
    fn wasm_sound_channel3_Channel3_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global83;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.global84;
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global83;
            self.global83 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global83;
        if (var5 == 0) as i32 != 0 {
            self.global85 = 0i32;
        }
    }
    fn wasm_sound_channel4_Channel4_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global86;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.global87;
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global86;
            self.global86 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global86;
        if (var5 == 0) as i32 != 0 {
            self.global88 = 0i32;
        }
    }
    fn wasm_sound_channel1_getNewFrequencyFromSweep<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global92;
        let var2 = self.global93;
        var0 = var1.wrapping_shr(var2 as u32);
        let var3 = self.global94;
        let var4: i32;
        if var3 != 0 {
            let var5 = self.global92;
            var4 = var5.wrapping_sub(var0);
        } else {
            let var6 = self.global92;
            var4 = var6.wrapping_add(var0);
        }
        var0 = var4;
        var0
    }
    fn wasm_sound_channel1_Channel1_setFrequency<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65300i32);
        var1 = var0.wrapping_shr(8i32 as u32);
        var2 = var3 & 248i32 | var1;
        var0 = var0 & 255i32;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65299i32, var0);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65300i32, var2);
        self.global95 = var0;
        self.global96 = var1;
        let var4 = self.global96;
        let var5 = self.global95;
        self.global97 = var4.wrapping_shl(8i32 as u32) | var5;
    }
    fn wasm_sound_channel1_calculateSweepAndCheckOverflow<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.wasm_sound_channel1_getNewFrequencyFromSweep(imports);
        var0 = var2;
        var1 = (var0 <= 2047i32) as i32;
        let var3: i32;
        if var1 != 0 {
            let var4 = self.global93;
            var3 = (var4 > 0i32) as i32;
        } else {
            var3 = var1;
        }
        if var3 & 1i32 != 0 {
            self.global92 = var0;
            self.wasm_sound_channel1_Channel1_setFrequency(imports, var0);
            let var5 = self.wasm_sound_channel1_getNewFrequencyFromSweep(imports);
            var0 = var5;
        }
        if (var0 > 2047i32) as i32 != 0 {
            self.global79 = 0i32;
        }
    }
    fn wasm_sound_channel1_Channel1_updateSweep<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global89;
        self.global89 = var0.wrapping_sub(1i32);
        let var1 = self.global89;
        if (var1 <= 0i32) as i32 != 0 {
            let var2 = self.global90;
            self.global89 = var2;
            let var3 = self.global91;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.global90;
                var4 = (var5 > 0i32) as i32;
            } else {
                let var6 = self.global91;
                var4 = var6;
            }
            if var4 & 1i32 != 0 {
                self.wasm_sound_channel1_calculateSweepAndCheckOverflow(imports);
            }
        }
    }
    fn wasm_sound_channel1_Channel1_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global98;
        self.global98 = var1.wrapping_sub(1i32);
        let var2 = self.global98;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.global99;
            self.global98 = var3;
            let var4 = self.global98;
            if var4 != 0 {
                let var5 = self.global100;
                let var6: i32;
                if var5 != 0 {
                    let var7 = self.global101;
                    var6 = (var7 < 15i32) as i32;
                } else {
                    let var8 = self.global100;
                    var6 = var8;
                }
                if var6 & 1i32 != 0 {
                    let var9 = self.global101;
                    self.global101 = var9.wrapping_add(1i32);
                } else {
                    let var10 = self.global100;
                    var0 = (var10 == 0) as i32;
                    let var11: i32;
                    if var0 != 0 {
                        let var12 = self.global101;
                        var11 = (var12 > 0i32) as i32;
                    } else {
                        var11 = var0;
                    }
                    if var11 & 1i32 != 0 {
                        let var13 = self.global101;
                        self.global101 = var13.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_channel2_Channel2_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global102;
        self.global102 = var1.wrapping_sub(1i32);
        let var2 = self.global102;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.global103;
            self.global102 = var3;
            let var4 = self.global102;
            if var4 != 0 {
                let var5 = self.global104;
                let var6: i32;
                if var5 != 0 {
                    let var7 = self.global105;
                    var6 = (var7 < 15i32) as i32;
                } else {
                    let var8 = self.global104;
                    var6 = var8;
                }
                if var6 & 1i32 != 0 {
                    let var9 = self.global105;
                    self.global105 = var9.wrapping_add(1i32);
                } else {
                    let var10 = self.global104;
                    var0 = (var10 == 0) as i32;
                    let var11: i32;
                    if var0 != 0 {
                        let var12 = self.global105;
                        var11 = (var12 > 0i32) as i32;
                    } else {
                        var11 = var0;
                    }
                    if var11 & 1i32 != 0 {
                        let var13 = self.global105;
                        self.global105 = var13.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_channel4_Channel4_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global106;
        self.global106 = var1.wrapping_sub(1i32);
        let var2 = self.global106;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.global107;
            self.global106 = var3;
            let var4 = self.global106;
            if var4 != 0 {
                let var5 = self.global108;
                let var6: i32;
                if var5 != 0 {
                    let var7 = self.global109;
                    var6 = (var7 < 15i32) as i32;
                } else {
                    let var8 = self.global108;
                    var6 = var8;
                }
                if var6 & 1i32 != 0 {
                    let var9 = self.global109;
                    self.global109 = var9.wrapping_add(1i32);
                } else {
                    let var10 = self.global108;
                    var0 = (var10 == 0) as i32;
                    let var11: i32;
                    if var0 != 0 {
                        let var12 = self.global109;
                        var11 = (var12 > 0i32) as i32;
                    } else {
                        var11 = var0;
                    }
                    if var11 & 1i32 != 0 {
                        let var13 = self.global109;
                        self.global109 = var13.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_sound_updateFrameSequencer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global75;
        self.global75 = var1.wrapping_add(var0);
        let var2 = self.global75;
        let var3 = self.wasm_sound_sound_Sound_maxFrameSequenceCycles(imports);
        if (var2 >= var3) as i32 != 0 {
            let var4 = self.global75;
            let var5 = self.wasm_sound_sound_Sound_maxFrameSequenceCycles(imports);
            self.global75 = var4.wrapping_sub(var5);
            'label0: loop {
                'label1: loop {
                    'label2: loop {
                        'label3: loop {
                            'label4: loop {
                                'label5: loop {
                                    'label6: loop {
                                        let var6 = self.global76;
                                        match var6 {
                                            0 => break 'label5,
                                            1 => break 'label6,
                                            2 => break 'label4,
                                            3 => break 'label6,
                                            4 => break 'label3,
                                            5 => break 'label6,
                                            6 => break 'label2,
                                            7 => break 'label1,
                                            _ => break 'label6,
                                        }
                                        break;
                                    }
                                    break 'label0;
                                    break;
                                }
                                self.wasm_sound_channel1_Channel1_updateLength(imports);
                                self.wasm_sound_channel2_Channel2_updateLength(imports);
                                self.wasm_sound_channel3_Channel3_updateLength(imports);
                                self.wasm_sound_channel4_Channel4_updateLength(imports);
                                break 'label0;
                                break;
                            }
                            self.wasm_sound_channel1_Channel1_updateLength(imports);
                            self.wasm_sound_channel2_Channel2_updateLength(imports);
                            self.wasm_sound_channel3_Channel3_updateLength(imports);
                            self.wasm_sound_channel4_Channel4_updateLength(imports);
                            self.wasm_sound_channel1_Channel1_updateSweep(imports);
                            break 'label0;
                            break;
                        }
                        self.wasm_sound_channel1_Channel1_updateLength(imports);
                        self.wasm_sound_channel2_Channel2_updateLength(imports);
                        self.wasm_sound_channel3_Channel3_updateLength(imports);
                        self.wasm_sound_channel4_Channel4_updateLength(imports);
                        break 'label0;
                        break;
                    }
                    self.wasm_sound_channel1_Channel1_updateLength(imports);
                    self.wasm_sound_channel2_Channel2_updateLength(imports);
                    self.wasm_sound_channel3_Channel3_updateLength(imports);
                    self.wasm_sound_channel4_Channel4_updateLength(imports);
                    self.wasm_sound_channel1_Channel1_updateSweep(imports);
                    break 'label0;
                    break;
                }
                self.wasm_sound_channel1_Channel1_updateEnvelope(imports);
                self.wasm_sound_channel2_Channel2_updateEnvelope(imports);
                self.wasm_sound_channel4_Channel4_updateEnvelope(imports);
                break;
            }
            let var7 = self.global76;
            self.global76 = var7.wrapping_add(1i32);
            let var8 = self.global76;
            if (var8 >= 8i32) as i32 != 0 {
                self.global76 = 0i32;
            }
            return 1i32;
        }
        0i32
    }
    fn wasm_sound_channel1_Channel1_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global110;
        self.global110 = var1.wrapping_add(var0);
        let var2 = self.global111;
        let var3 = self.global110;
        if (var2.wrapping_sub(var3) > 0i32) as i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_sound_didChannelDacChange<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match var0.wrapping_sub(1i32) {
                                    0 => break 'label4,
                                    1 => break 'label3,
                                    2 => break 'label2,
                                    3 => break 'label1,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label0;
                            break;
                        }
                        let var1 = self.global112;
                        let var2 = self.global113;
                        if (var1 != var2) as i32 != 0 {
                            let var3 = self.global113;
                            self.global112 = var3;
                            return 1i32;
                        }
                        return 0i32;
                        break;
                    }
                    let var4 = self.global114;
                    let var5 = self.global115;
                    if (var4 != var5) as i32 != 0 {
                        let var6 = self.global115;
                        self.global114 = var6;
                        return 1i32;
                    }
                    return 0i32;
                    break;
                }
                let var7 = self.global116;
                let var8 = self.global117;
                if (var7 != var8) as i32 != 0 {
                    let var9 = self.global117;
                    self.global116 = var9;
                    return 1i32;
                }
                return 0i32;
                break;
            }
            let var10 = self.global118;
            let var11 = self.global119;
            if (var10 != var11) as i32 != 0 {
                let var12 = self.global119;
                self.global118 = var12;
                return 1i32;
            }
            return 0i32;
            break;
        }
        0i32
    }
    fn wasm_sound_channel2_Channel2_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global120;
        self.global120 = var1.wrapping_add(var0);
        let var2 = self.global121;
        let var3 = self.global120;
        if (var2.wrapping_sub(var3) > 0i32) as i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel3_Channel3_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global122;
        self.global122 = var2.wrapping_add(var0);
        let var3 = self.global123;
        let var4 = self.global122;
        var1 = (var3.wrapping_sub(var4) > 0i32) as i32;
        let var5: i32;
        if var1 != 0 {
            let var6 = self.global60;
            var5 = (var6 == 0) as i32;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel4_Channel4_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global124;
        self.global124 = var1.wrapping_add(var0);
        let var2 = self.global125;
        let var3 = self.global124;
        if (var2.wrapping_sub(var3) > 0i32) as i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel1_Channel1_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global97;
        self.global111 = 2048i32.wrapping_sub(var0).wrapping_mul(4i32);
        let var1 = self.global68;
        if var1 != 0 {
            let var2 = self.global111;
            self.global111 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            match var0.wrapping_sub(1i32) {
                                0 => break 'label3,
                                1 => break 'label2,
                                2 => break 'label1,
                                _ => break 'label4,
                            }
                            break;
                        }
                        break 'label0;
                        break;
                    }
                    let var2 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 129i32);
                    return var2;
                    break;
                }
                let var3 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 135i32);
                return var3;
                break;
            }
            let var4 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 126i32);
            return var4;
            break;
        }
        let var5 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 1i32);
        var5
    }
    fn wasm_sound_channel1_Channel1_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global111;
        self.global111 = var2.wrapping_sub(var0);
        let var3 = self.global111;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global111;
            var0 = var4;
            var0 = { let a = var0; let b = 0i32.wrapping_sub(var0); if (var0 > 0i32) as i32 != 0 { a } else { b } };
            self.wasm_sound_channel1_Channel1_resetTimer(imports);
            let var5 = self.global111;
            self.global111 = var5.wrapping_sub(var0);
            let var6 = self.global127;
            self.global127 = var6.wrapping_add(1i32);
            let var7 = self.global127;
            if (var7 >= 8i32) as i32 != 0 {
                self.global127 = 0i32;
            }
        }
        let var8 = self.global79;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.global113;
            var9 = var10;
        } else {
            let var11 = self.global79;
            var9 = var11;
        }
        if var9 & 1i32 != 0 {
            let var12 = self.global101;
            var0 = var12;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var13 = self.global128;
        let var14 = self.global127;
        let var15 = self.wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform(imports, var13, var14);
        if (var15 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        var1.wrapping_mul(var0).wrapping_add(15i32)
    }
    fn wasm_sound_channel1_Channel1_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global110;
        var0 = var1;
        self.global110 = 0i32;
        let var2 = self.wasm_sound_channel1_Channel1_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel2_Channel2_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global130;
        self.global121 = 2048i32.wrapping_sub(var0).wrapping_mul(4i32);
        let var1 = self.global68;
        if var1 != 0 {
            let var2 = self.global121;
            self.global121 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_channel2_Channel2_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global121;
        self.global121 = var2.wrapping_sub(var0);
        let var3 = self.global121;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global121;
            var0 = var4;
            var0 = { let a = var0; let b = 0i32.wrapping_sub(var0); if (var0 > 0i32) as i32 != 0 { a } else { b } };
            self.wasm_sound_channel2_Channel2_resetTimer(imports);
            let var5 = self.global121;
            self.global121 = var5.wrapping_sub(var0);
            let var6 = self.global131;
            self.global131 = var6.wrapping_add(1i32);
            let var7 = self.global131;
            if (var7 >= 8i32) as i32 != 0 {
                self.global131 = 0i32;
            }
        }
        let var8 = self.global82;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.global115;
            var9 = var10;
        } else {
            let var11 = self.global82;
            var9 = var11;
        }
        if var9 & 1i32 != 0 {
            let var12 = self.global105;
            var0 = var12;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var13 = self.global132;
        let var14 = self.global131;
        let var15 = self.wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform(imports, var13, var14);
        if (var15 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        var1.wrapping_mul(var0).wrapping_add(15i32)
    }
    fn wasm_sound_channel2_Channel2_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global120;
        var0 = var1;
        self.global120 = 0i32;
        let var2 = self.wasm_sound_channel2_Channel2_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel3_Channel3_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global134;
        self.global123 = 2048i32.wrapping_sub(var0).wrapping_mul(2i32);
        let var1 = self.global68;
        if var1 != 0 {
            let var2 = self.global123;
            self.global123 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_channel3_Channel3_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global123;
        self.global123 = var3.wrapping_sub(var0);
        let var4 = self.global123;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global123;
            var1 = var5;
            var1 = { let a = var1; let b = 0i32.wrapping_sub(var1); if (var1 > 0i32) as i32 != 0 { a } else { b } };
            self.wasm_sound_channel3_Channel3_resetTimer(imports);
            let var6 = self.global123;
            self.global123 = var6.wrapping_sub(var1);
            let var7 = self.global135;
            self.global135 = var7.wrapping_add(1i32);
            let var8 = self.global135;
            if (var8 >= 32i32) as i32 != 0 {
                self.global135 = 0i32;
            }
        }
        var1 = 0i32;
        let var9 = self.global136;
        var2 = var9;
        let var10 = self.global85;
        let var11: i32;
        if var10 != 0 {
            let var12 = self.global117;
            var11 = var12;
        } else {
            let var13 = self.global85;
            var11 = var13;
        }
        if var11 & 1i32 != 0 {
            let var14 = self.global60;
            if var14 != 0 {
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65308i32);
                var2 = var15.wrapping_shr(5i32 as u32) & 15i32;
                self.global136 = var2;
                self.global60 = 0i32;
            }
        } else {
            return 15i32;
        }
        let var16 = self.global135;
        let var17 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, (var16 / 2i32).wrapping_add(65328i32));
        var0 = var17;
        let var18 = self.global135;
        let var19: i32;
        if var18.wrapping_rem(2i32) != 0 {
            var19 = var0 & 15i32;
        } else {
            var19 = var0.wrapping_shr(4i32 as u32) & 15i32;
        }
        var0 = var19;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match var2 {
                                    0 => break 'label4,
                                    1 => break 'label3,
                                    2 => break 'label2,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label1;
                            break;
                        }
                        var0 = var0.wrapping_shr(4i32 as u32);
                        break 'label0;
                        break;
                    }
                    var1 = 1i32;
                    break 'label0;
                    break;
                }
                var0 = var0.wrapping_shr(1i32 as u32);
                var1 = 2i32;
                break 'label0;
                break;
            }
            var0 = var0.wrapping_shr(2i32 as u32);
            var1 = 4i32;
            break;
        }
        let var20: i32;
        if (var1 > 0i32) as i32 != 0 {
            var20 = var0 / var1;
        } else {
            var20 = 0i32;
        }
        var0 = var20;
        var0.wrapping_add(15i32)
    }
    fn wasm_sound_channel3_Channel3_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global122;
        var0 = var1;
        self.global122 = 0i32;
        let var2 = self.wasm_sound_channel3_Channel3_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global138;
        let var2 = self.global139;
        var0 = var1.wrapping_shl(var2 as u32);
        let var3 = self.global68;
        if var3 != 0 {
            var0 = var0.wrapping_mul(2i32);
        }
        var0
    }
    fn wasm_sound_channel4_Channel4_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global125;
        self.global125 = var2.wrapping_sub(var0);
        let var3 = self.global125;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global125;
            var0 = var4;
            var0 = { let a = var0; let b = 0i32.wrapping_sub(var0); if (var0 > 0i32) as i32 != 0 { a } else { b } };
            let var5 = self.wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod(imports);
            self.global125 = var5;
            let var6 = self.global125;
            self.global125 = var6.wrapping_sub(var0);
            let var7 = self.global140;
            let var8 = self.global140;
            var1 = var7 & 1i32 ^ var8.wrapping_shr(1i32 as u32) & 1i32;
            let var9 = self.global140;
            self.global140 = var9.wrapping_shr(1i32 as u32);
            let var10 = self.global140;
            self.global140 = var10 | var1.wrapping_shl(14i32 as u32);
            let var11 = self.global141;
            if var11 != 0 {
                let var12 = self.global140;
                self.global140 = var12 & -65i32;
                let var13 = self.global140;
                self.global140 = var13 | var1.wrapping_shl(6i32 as u32);
            }
        }
        let var14 = self.global88;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.global119;
            var15 = var16;
        } else {
            let var17 = self.global88;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global109;
            var1 = var18;
        } else {
            return 15i32;
        }
        let var19 = self.global140;
        let var20 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var19);
        let var21: i32;
        if var20 != 0 {
            var21 = -1i32;
        } else {
            var21 = 1i32;
        }
        var0 = var21;
        var0.wrapping_mul(var1).wrapping_add(15i32)
    }
    fn wasm_sound_channel4_Channel4_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global124;
        var0 = var1;
        self.global124 = 0i32;
        let var2 = self.wasm_sound_channel4_Channel4_getSample(imports, var0);
        var2
    }
    fn wasm_cpu_cpu_Cpu_CLOCK_SPEED<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 8388608i32;
        }
        4194304i32
    }
    fn wasm_sound_sound_Sound_maxDownSampleCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_cpu_cpu_Cpu_CLOCK_SPEED(imports);
        var0
    }
    fn wasm_sound_sound_getSampleAsUnsignedByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        if (var0 == 60i32) as i32 != 0 {
            return 127i32;
        }
        var2 = 100000i32;
        (var0.wrapping_sub(60i32).wrapping_mul(var2).wrapping_mul(var1) / 8i32 / 100000i32).wrapping_add(60i32).wrapping_mul(100000i32) / 47244i32
    }
    fn wasm_sound_sound_mixChannelSamples<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        self.global61 = 0i32;
        let var6 = self.global145;
        let var7: i32;
        if var6 != 0 {
            var7 = 0i32.wrapping_add(var0);
        } else {
            var7 = 15i32;
        }
        var4 = var7;
        let var8 = self.global146;
        let var9: i32;
        if var8 != 0 {
            var9 = var4.wrapping_add(var1);
        } else {
            var9 = var4.wrapping_add(15i32);
        }
        var4 = var9;
        let var10 = self.global147;
        let var11: i32;
        if var10 != 0 {
            var11 = var4.wrapping_add(var2);
        } else {
            var11 = var4.wrapping_add(15i32);
        }
        var4 = var11;
        let var12 = self.global148;
        let var13: i32;
        if var12 != 0 {
            var13 = var4.wrapping_add(var3);
        } else {
            var13 = var4.wrapping_add(15i32);
        }
        var4 = var13;
        let var14 = self.global149;
        let var15: i32;
        if var14 != 0 {
            var15 = 0i32.wrapping_add(var0);
        } else {
            var15 = 15i32;
        }
        var5 = var15;
        let var16 = self.global150;
        let var17: i32;
        if var16 != 0 {
            var17 = var5.wrapping_add(var1);
        } else {
            var17 = var5.wrapping_add(15i32);
        }
        var5 = var17;
        let var18 = self.global151;
        let var19: i32;
        if var18 != 0 {
            var19 = var5.wrapping_add(var2);
        } else {
            var19 = var5.wrapping_add(15i32);
        }
        var5 = var19;
        let var20 = self.global152;
        let var21: i32;
        if var20 != 0 {
            var21 = var5.wrapping_add(var3);
        } else {
            var21 = var5.wrapping_add(15i32);
        }
        var5 = var21;
        self.global62 = 0i32;
        self.global142 = 0i32;
        let var22 = self.global153;
        let var23 = self.wasm_sound_sound_getSampleAsUnsignedByte(imports, var4, var22.wrapping_add(1i32));
        var0 = var23;
        let var24 = self.global154;
        let var25 = self.wasm_sound_sound_getSampleAsUnsignedByte(imports, var5, var24.wrapping_add(1i32));
        var1 = var25;
        self.global155 = var0;
        self.global156 = var1;
        let var26 = self.wasm_helpers_index_concatenateBytes(imports, var0, var1);
        var26
    }
    fn wasm_memory_memory_setLeftAndRightOutputForAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        var3 = var2.wrapping_mul(2i32).wrapping_add(588800i32);
        self.memory.store8(var3 as usize, var0.wrapping_add(1i32) as u8);
        self.memory.store8(var3.wrapping_add(1i32) as usize, var1.wrapping_add(1i32) as u8);
    }
    fn wasm_sound_sound_accumulateSound<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.wasm_sound_channel1_Channel1_willChannelUpdate(imports, var0);
        var1 = var5;
        let var6: i32;
        if var1 != 0 {
            var6 = var1;
        } else {
            let var7 = self.wasm_sound_sound_didChannelDacChange(imports, 1i32);
            var6 = var7;
        }
        var1 = var6 & 1i32;
        let var8 = self.wasm_sound_channel2_Channel2_willChannelUpdate(imports, var0);
        var2 = var8;
        let var9: i32;
        if var2 != 0 {
            var9 = var2;
        } else {
            let var10 = self.wasm_sound_sound_didChannelDacChange(imports, 2i32);
            var9 = var10;
        }
        var2 = var9 & 1i32;
        let var11 = self.wasm_sound_channel3_Channel3_willChannelUpdate(imports, var0);
        var3 = var11;
        let var12: i32;
        if var3 != 0 {
            var12 = var3;
        } else {
            let var13 = self.wasm_sound_sound_didChannelDacChange(imports, 3i32);
            var12 = var13;
        }
        var3 = var12 & 1i32;
        let var14 = self.wasm_sound_channel4_Channel4_willChannelUpdate(imports, var0);
        var4 = var14;
        let var15: i32;
        if var4 != 0 {
            var15 = var4;
        } else {
            let var16 = self.wasm_sound_sound_didChannelDacChange(imports, 4i32);
            var15 = var16;
        }
        var4 = var15 & 1i32;
        if var1 != 0 {
            let var17 = self.wasm_sound_channel1_Channel1_getSampleFromCycleCounter(imports);
            self.global126 = var17;
        }
        if var2 != 0 {
            let var18 = self.wasm_sound_channel2_Channel2_getSampleFromCycleCounter(imports);
            self.global129 = var18;
        }
        if var3 != 0 {
            let var19 = self.wasm_sound_channel3_Channel3_getSampleFromCycleCounter(imports);
            self.global133 = var19;
        }
        if var4 != 0 {
            let var20 = self.wasm_sound_channel4_Channel4_getSampleFromCycleCounter(imports);
            self.global137 = var20;
        }
        let var21: i32;
        if var1 != 0 {
            var21 = var1;
        } else {
            var21 = var2;
        }
        var1 = var21 & 1i32;
        let var22: i32;
        if var1 != 0 {
            var22 = var1;
        } else {
            var22 = var3;
        }
        var1 = var22 & 1i32;
        let var23: i32;
        if var1 != 0 {
            var23 = var1;
        } else {
            var23 = var4;
        }
        if var23 & 1i32 != 0 {
            self.global142 = 1i32;
        }
        let var24 = self.global143;
        let var25 = self.global144;
        self.global143 = var24.wrapping_add(var0.wrapping_mul(var25));
        let var26 = self.global143;
        let var27 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
        if (var26 >= var27) as i32 != 0 {
            let var28 = self.global143;
            let var29 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
            self.global143 = var28.wrapping_sub(var29);
            let var30 = self.global142;
            let var31: i32;
            if var30 != 0 {
                let var32 = self.global142;
                var31 = var32;
            } else {
                let var33 = self.global61;
                var31 = var33;
            }
            var1 = var31 & 1i32;
            let var34: i32;
            if var1 != 0 {
                var34 = var1;
            } else {
                let var35 = self.global62;
                var34 = var35;
            }
            if var34 & 1i32 != 0 {
                let var36 = self.global126;
                let var37 = self.global129;
                let var38 = self.global133;
                let var39 = self.global137;
                let var40 = self.wasm_sound_sound_mixChannelSamples(imports, var36, var37, var38, var39);
                var40;
            }
            let var41 = self.global155;
            let var42 = self.global156;
            let var43 = self.global41;
            self.wasm_memory_memory_setLeftAndRightOutputForAudioQueue(imports, var41.wrapping_add(1i32), var42.wrapping_add(1i32), var43);
            let var44 = self.global41;
            self.global41 = var44.wrapping_add(1i32);
            let var45 = self.global41;
            let var46 = self.global157;
            if (var45 >= (var46 / 2i32).wrapping_sub(1i32)) as i32 != 0 {
                let var47 = self.global41;
                self.global41 = var47.wrapping_sub(1i32);
            }
        }
    }
    fn wasm_sound_sound_calculateSound<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.wasm_sound_channel1_Channel1_getSample(imports, var0);
        var1 = var5;
        let var6 = self.wasm_sound_channel2_Channel2_getSample(imports, var0);
        var2 = var6;
        let var7 = self.wasm_sound_channel3_Channel3_getSample(imports, var0);
        var3 = var7;
        let var8 = self.wasm_sound_channel4_Channel4_getSample(imports, var0);
        var4 = var8;
        self.global126 = var1;
        self.global129 = var2;
        self.global133 = var3;
        self.global137 = var4;
        let var9 = self.global143;
        let var10 = self.global144;
        self.global143 = var9.wrapping_add(var0.wrapping_mul(var10));
        let var11 = self.global143;
        let var12 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
        if (var11 >= var12) as i32 != 0 {
            let var13 = self.global143;
            let var14 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
            self.global143 = var13.wrapping_sub(var14);
            let var15 = self.wasm_sound_sound_mixChannelSamples(imports, var1, var2, var3, var4);
            var0 = var15;
            let var16 = self.wasm_helpers_index_splitHighByte(imports, var0);
            let var17 = self.wasm_helpers_index_splitLowByte(imports, var0);
            let var18 = self.global41;
            self.wasm_memory_memory_setLeftAndRightOutputForAudioQueue(imports, var16.wrapping_add(1i32), var17.wrapping_add(1i32), var18);
            let var19 = self.global41;
            self.global41 = var19.wrapping_add(1i32);
            let var20 = self.global41;
            let var21 = self.global157;
            if (var20 >= (var21 / 2i32).wrapping_sub(1i32)) as i32 != 0 {
                let var22 = self.global41;
                self.global41 = var22.wrapping_sub(1i32);
            }
        }
    }
    fn wasm_sound_sound_updateSound<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.wasm_sound_sound_updateFrameSequencer(imports, var0);
        var1 = var2;
        let var3 = self.global18;
        let var4: i32;
        if var3 != 0 {
            var4 = (var1 == 0) as i32;
        } else {
            let var5 = self.global18;
            var4 = var5;
        }
        if var4 & 1i32 != 0 {
            self.wasm_sound_sound_accumulateSound(imports, var0);
        } else {
            self.wasm_sound_sound_calculateSound(imports, var0);
        }
    }
    fn wasm_sound_sound_batchProcessAudio<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global74;
        let var1 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
        if (var0 < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global74;
            let var3 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
                self.wasm_sound_sound_updateSound(imports, var4);
                let var5 = self.global74;
                let var6 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
                self.global74 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_sound_channel1_Channel1_updateNRx0<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global90 = (var0 & 112i32).wrapping_shr(4i32 as u32);
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global94 = var1;
        self.global93 = var0 & 7i32;
    }
    fn wasm_sound_channel3_Channel3_updateNRx0<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        self.global117 = var1;
    }
    fn wasm_sound_channel1_Channel1_updateNRx1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global128 = var0.wrapping_shr(6i32 as u32) & 3i32;
        self.global159 = var0 & 63i32;
        let var1 = self.global159;
        self.global77 = 64i32.wrapping_sub(var1);
    }
    fn wasm_sound_channel2_Channel2_updateNRx1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global132 = var0.wrapping_shr(6i32 as u32) & 3i32;
        self.global160 = var0 & 63i32;
        let var1 = self.global160;
        self.global80 = 64i32.wrapping_sub(var1);
    }
    fn wasm_sound_channel3_Channel3_updateNRx1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global161 = var0;
        let var1 = self.global161;
        self.global83 = 256i32.wrapping_sub(var1);
    }
    fn wasm_sound_channel4_Channel4_updateNRx1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global162 = var0 & 63i32;
        let var1 = self.global162;
        self.global86 = 64i32.wrapping_sub(var1);
    }
    fn wasm_sound_channel1_Channel1_updateNRx2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global163 = var0.wrapping_shr(4i32 as u32) & 15i32;
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global100 = var1;
        self.global99 = var0 & 7i32;
        self.global113 = (var0 & 248i32 > 0i32) as i32;
    }
    fn wasm_sound_channel2_Channel2_updateNRx2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global164 = var0.wrapping_shr(4i32 as u32) & 15i32;
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global104 = var1;
        self.global103 = var0 & 7i32;
        self.global115 = (var0 & 248i32 > 0i32) as i32;
    }
    fn wasm_sound_channel3_Channel3_updateNRx2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global165 = var0.wrapping_shr(5i32 as u32) & 15i32;
    }
    fn wasm_sound_channel4_Channel4_updateNRx2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global166 = var0.wrapping_shr(4i32 as u32) & 15i32;
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global108 = var1;
        self.global107 = var0 & 7i32;
        self.global119 = (var0 & 248i32 > 0i32) as i32;
    }
    fn wasm_sound_channel1_Channel1_updateNRx3<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global95 = var0;
        let var1 = self.global96;
        let var2 = self.global95;
        self.global97 = var1.wrapping_shl(8i32 as u32) | var2;
    }
    fn wasm_sound_channel2_Channel2_updateNRx3<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global167 = var0;
        let var1 = self.global168;
        let var2 = self.global167;
        self.global130 = var1.wrapping_shl(8i32 as u32) | var2;
    }
    fn wasm_sound_channel3_Channel3_updateNRx3<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global169 = var0;
        let var1 = self.global170;
        let var2 = self.global169;
        self.global134 = var1.wrapping_shl(8i32 as u32) | var2;
    }
    fn wasm_sound_channel4_Channel4_updateNRx3<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global139 = var0.wrapping_shr(4i32 as u32);
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global141 = var1;
        self.global171 = var0 & 7i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                let var2 = self.global171;
                                                match var2 {
                                                    0 => break 'label8,
                                                    1 => break 'label7,
                                                    2 => break 'label6,
                                                    3 => break 'label5,
                                                    4 => break 'label4,
                                                    5 => break 'label3,
                                                    6 => break 'label2,
                                                    7 => break 'label1,
                                                    _ => break 'label9,
                                                }
                                                break;
                                            }
                                            break 'label0;
                                            break;
                                        }
                                        self.global138 = 8i32;
                                        return;
                                        break;
                                    }
                                    self.global138 = 16i32;
                                    return;
                                    break;
                                }
                                self.global138 = 32i32;
                                return;
                                break;
                            }
                            self.global138 = 48i32;
                            return;
                            break;
                        }
                        self.global138 = 64i32;
                        return;
                        break;
                    }
                    self.global138 = 80i32;
                    return;
                    break;
                }
                self.global138 = 96i32;
                return;
                break;
            }
            self.global138 = 112i32;
            break;
        }
    }
    fn wasm_sound_channel1_Channel1_updateNRx4<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global78 = var1;
        self.global96 = var0 & 7i32;
        let var2 = self.global96;
        let var3 = self.global95;
        self.global97 = var2.wrapping_shl(8i32 as u32) | var3;
    }
    fn wasm_sound_channel1_Channel1_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        self.global79 = 1i32;
        let var1 = self.global77;
        if (var1 == 0) as i32 != 0 {
            self.global77 = 64i32;
        }
        self.wasm_sound_channel1_Channel1_resetTimer(imports);
        let var2 = self.global99;
        self.global98 = var2;
        let var3 = self.global163;
        self.global101 = var3;
        let var4 = self.global97;
        self.global92 = var4;
        let var5 = self.global90;
        self.global89 = var5;
        let var6 = self.global90;
        var0 = (var6 > 0i32) as i32;
        let var7: i32;
        if var0 != 0 {
            let var8 = self.global93;
            var7 = (var8 > 0i32) as i32;
        } else {
            var7 = var0;
        }
        if var7 & 1i32 != 0 {
            self.global91 = 1i32;
        } else {
            self.global91 = 0i32;
        }
        let var9 = self.global93;
        if (var9 > 0i32) as i32 != 0 {
            self.wasm_sound_channel1_calculateSweepAndCheckOverflow(imports);
        }
        let var10 = self.global113;
        if (var10 == 0) as i32 != 0 {
            self.global79 = 0i32;
        }
    }
    fn wasm_sound_channel2_Channel2_updateNRx4<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global81 = var1;
        self.global168 = var0 & 7i32;
        let var2 = self.global168;
        let var3 = self.global167;
        self.global130 = var2.wrapping_shl(8i32 as u32) | var3;
    }
    fn wasm_sound_channel2_Channel2_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global82 = 1i32;
        let var0 = self.global80;
        if (var0 == 0) as i32 != 0 {
            self.global80 = 64i32;
        }
        self.wasm_sound_channel2_Channel2_resetTimer(imports);
        let var1 = self.global103;
        self.global102 = var1;
        let var2 = self.global164;
        self.global105 = var2;
        let var3 = self.global115;
        if (var3 == 0) as i32 != 0 {
            self.global82 = 0i32;
        }
    }
    fn wasm_sound_channel3_Channel3_updateNRx4<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global84 = var1;
        self.global170 = var0 & 7i32;
        let var2 = self.global170;
        let var3 = self.global169;
        self.global134 = var2.wrapping_shl(8i32 as u32) | var3;
    }
    fn wasm_sound_channel3_Channel3_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global85 = 1i32;
        let var0 = self.global83;
        if (var0 == 0) as i32 != 0 {
            self.global83 = 256i32;
        }
        self.wasm_sound_channel3_Channel3_resetTimer(imports);
        self.global135 = 0i32;
        let var1 = self.global117;
        if (var1 == 0) as i32 != 0 {
            self.global85 = 0i32;
        }
    }
    fn wasm_sound_channel4_Channel4_updateNRx4<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global87 = var1;
    }
    fn wasm_sound_channel4_Channel4_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global88 = 1i32;
        let var0 = self.global86;
        if (var0 == 0) as i32 != 0 {
            self.global86 = 64i32;
        }
        let var1 = self.wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod(imports);
        self.global125 = var1;
        let var2 = self.global107;
        self.global106 = var2;
        let var3 = self.global166;
        self.global109 = var3;
        self.global140 = 32767i32;
        let var4 = self.global119;
        if (var4 == 0) as i32 != 0 {
            self.global88 = 0i32;
        }
    }
    fn wasm_sound_sound_Sound_updateNR50<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global153 = var0.wrapping_shr(4i32 as u32) & 7i32;
        self.global154 = var0 & 7i32;
    }
    fn wasm_sound_sound_Sound_updateNR51<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        self.global148 = var1;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global147 = var2;
        let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var0);
        self.global146 = var3;
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
        self.global145 = var4;
        let var5 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global152 = var5;
        let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global151 = var6;
        let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var0);
        self.global150 = var7;
        let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
        self.global149 = var8;
    }
    fn wasm_sound_sound_Sound_updateNR52<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        self.global158 = var1;
    }
    fn wasm_sound_registers_SoundRegisterWriteTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = (var0 != 65318i32) as i32;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.global158;
            var3 = (var4 == 0) as i32;
        } else {
            var3 = var2;
        }
        if var3 & 1i32 != 0 {
            return 0i32;
        }
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                'label18: loop {
                                                                                    'label19: loop {
                                                                                        'label20: loop {
                                                                                            'label21: loop {
                                                                                                'label22: loop {
                                                                                                    match var0.wrapping_sub(65296i32) {
                                                                                                        0 => break 'label21,
                                                                                                        1 => break 'label19,
                                                                                                        2 => break 'label15,
                                                                                                        3 => break 'label11,
                                                                                                        4 => break 'label7,
                                                                                                        5 => break 'label22,
                                                                                                        6 => break 'label18,
                                                                                                        7 => break 'label14,
                                                                                                        8 => break 'label10,
                                                                                                        9 => break 'label6,
                                                                                                        10 => break 'label20,
                                                                                                        11 => break 'label17,
                                                                                                        12 => break 'label13,
                                                                                                        13 => break 'label9,
                                                                                                        14 => break 'label5,
                                                                                                        15 => break 'label22,
                                                                                                        16 => break 'label16,
                                                                                                        17 => break 'label12,
                                                                                                        18 => break 'label8,
                                                                                                        19 => break 'label4,
                                                                                                        20 => break 'label3,
                                                                                                        21 => break 'label2,
                                                                                                        22 => break 'label1,
                                                                                                        _ => break 'label22,
                                                                                                    }
                                                                                                    break;
                                                                                                }
                                                                                                break 'label0;
                                                                                                break;
                                                                                            }
                                                                                            self.wasm_sound_channel1_Channel1_updateNRx0(imports, var1);
                                                                                            return 1i32;
                                                                                            break;
                                                                                        }
                                                                                        self.wasm_sound_channel3_Channel3_updateNRx0(imports, var1);
                                                                                        return 1i32;
                                                                                        break;
                                                                                    }
                                                                                    self.wasm_sound_channel1_Channel1_updateNRx1(imports, var1);
                                                                                    return 1i32;
                                                                                    break;
                                                                                }
                                                                                self.wasm_sound_channel2_Channel2_updateNRx1(imports, var1);
                                                                                return 1i32;
                                                                                break;
                                                                            }
                                                                            self.wasm_sound_channel3_Channel3_updateNRx1(imports, var1);
                                                                            return 1i32;
                                                                            break;
                                                                        }
                                                                        self.wasm_sound_channel4_Channel4_updateNRx1(imports, var1);
                                                                        return 1i32;
                                                                        break;
                                                                    }
                                                                    self.wasm_sound_channel1_Channel1_updateNRx2(imports, var1);
                                                                    return 1i32;
                                                                    break;
                                                                }
                                                                self.wasm_sound_channel2_Channel2_updateNRx2(imports, var1);
                                                                return 1i32;
                                                                break;
                                                            }
                                                            self.global60 = 1i32;
                                                            self.wasm_sound_channel3_Channel3_updateNRx2(imports, var1);
                                                            return 1i32;
                                                            break;
                                                        }
                                                        self.wasm_sound_channel4_Channel4_updateNRx2(imports, var1);
                                                        return 1i32;
                                                        break;
                                                    }
                                                    self.wasm_sound_channel1_Channel1_updateNRx3(imports, var1);
                                                    return 1i32;
                                                    break;
                                                }
                                                self.wasm_sound_channel2_Channel2_updateNRx3(imports, var1);
                                                return 1i32;
                                                break;
                                            }
                                            self.wasm_sound_channel3_Channel3_updateNRx3(imports, var1);
                                            return 1i32;
                                            break;
                                        }
                                        self.wasm_sound_channel4_Channel4_updateNRx3(imports, var1);
                                        return 1i32;
                                        break;
                                    }
                                    let var5 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
                                    if var5 != 0 {
                                        self.wasm_sound_channel1_Channel1_updateNRx4(imports, var1);
                                        self.wasm_sound_channel1_Channel1_trigger(imports);
                                    }
                                    return 1i32;
                                    break;
                                }
                                let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
                                if var6 != 0 {
                                    self.wasm_sound_channel2_Channel2_updateNRx4(imports, var1);
                                    self.wasm_sound_channel2_Channel2_trigger(imports);
                                }
                                return 1i32;
                                break;
                            }
                            let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
                            if var7 != 0 {
                                self.wasm_sound_channel3_Channel3_updateNRx4(imports, var1);
                                self.wasm_sound_channel3_Channel3_trigger(imports);
                            }
                            return 1i32;
                            break;
                        }
                        let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
                        if var8 != 0 {
                            self.wasm_sound_channel4_Channel4_updateNRx4(imports, var1);
                            self.wasm_sound_channel4_Channel4_trigger(imports);
                        }
                        return 1i32;
                        break;
                    }
                    self.wasm_sound_sound_Sound_updateNR50(imports, var1);
                    self.global61 = 1i32;
                    return 1i32;
                    break;
                }
                self.wasm_sound_sound_Sound_updateNR51(imports, var1);
                self.global62 = 1i32;
                return 1i32;
                break;
            }
            self.wasm_sound_sound_Sound_updateNR52(imports, var1);
            let var9 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
            if (var9 == 0) as i32 != 0 {
                var0 = 65296i32;
                'label23: loop {
                    if (var0 < 65318i32) as i32 != 0 {
                        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, 0i32);
                        var0 = var0.wrapping_add(1i32);
                        continue 'label23;
                    }
                    break;
                }
            }
            return 1i32;
            break;
        }
        1i32
    }
    fn wasm_graphics_lcd_Lcd_updateLcdControl<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        self.global172 = var1;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
        self.global173 = var2;
        let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var0);
        self.global174 = var3;
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
        self.global52 = var4;
        let var5 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        self.global53 = var5;
        let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global175 = var6;
        let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var0);
        self.global176 = var7;
        let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
        self.global177 = var8;
    }
    fn wasm_memory_dma_startDmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        var0 = var0.wrapping_shl(8i32 as u32);
        'label0: loop {
            if (var1 <= 159i32) as i32 != 0 {
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0.wrapping_add(var1));
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var1.wrapping_add(65024i32), var2);
                var1 = var1.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
        self.global179 = 644i32;
    }
    fn wasm_memory_dma_getHdmaSourceFromMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global186;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
        let var2 = self.global187;
        let var3 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var2);
        let var4 = self.wasm_helpers_index_concatenateBytes(imports, var1, var3);
        var4 & 65520i32
    }
    fn wasm_memory_dma_getHdmaDestinationFromMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global188;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
        let var2 = self.global189;
        let var3 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var2);
        let var4 = self.wasm_helpers_index_concatenateBytes(imports, var1, var3);
        (var4 & 8176i32).wrapping_add(32768i32)
    }
    fn wasm_helpers_index_resetBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1 & (1i32.wrapping_shl(var0 as u32) ^ -1i32)
    }
    fn wasm_sound_registers_SoundRegisterReadTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 == 65318i32) as i32 != 0 {
            let var2 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65318i32);
            var1 = var2 & 128i32;
            let var3 = self.global79;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var1);
                var4 = var5;
            } else {
                let var6 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var1);
                var4 = var6;
            }
            var4;
            let var7 = self.global82;
            let var8: i32;
            if var7 != 0 {
                let var9 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var1);
                var8 = var9;
            } else {
                let var10 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var1);
                var8 = var10;
            }
            var8;
            let var11 = self.global85;
            let var12: i32;
            if var11 != 0 {
                let var13 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var1);
                var12 = var13;
            } else {
                let var14 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var1);
                var12 = var14;
            }
            var12;
            let var15 = self.global88;
            let var16: i32;
            if var15 != 0 {
                let var17 = self.wasm_helpers_index_setBitOnByte(imports, 3i32, var1);
                var16 = var17;
            } else {
                let var18 = self.wasm_helpers_index_resetBitOnByte(imports, 3i32, var1);
                var16 = var18;
            }
            var16;
            return var1 | 112i32;
        }
        -1i32
    }
    fn wasm_joypad_index_getJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global30;
        var0 = var1;
        let var2 = self.global31;
        if var2 != 0 {
            let var3 = self.global22;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var0);
                var4 = var5;
            } else {
                let var6 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var0);
                var4 = var6;
            }
            var0 = var4;
            let var7 = self.global23;
            let var8: i32;
            if var7 != 0 {
                let var9 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                var8 = var9;
            } else {
                let var10 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var0);
                var8 = var10;
            }
            var0 = var8;
            let var11 = self.global24;
            let var12: i32;
            if var11 != 0 {
                let var13 = self.wasm_helpers_index_resetBitOnByte(imports, 3i32, var0);
                var12 = var13;
            } else {
                let var14 = self.wasm_helpers_index_setBitOnByte(imports, 3i32, var0);
                var12 = var14;
            }
            var0 = var12;
            let var15 = self.global25;
            let var16: i32;
            if var15 != 0 {
                let var17 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var0);
                var16 = var17;
            } else {
                let var18 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var0);
                var16 = var18;
            }
            var0 = var16;
        } else {
            let var19 = self.global32;
            if var19 != 0 {
                let var20 = self.global26;
                let var21: i32;
                if var20 != 0 {
                    let var22 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                    var21 = var22;
                } else {
                    let var23 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var0);
                    var21 = var23;
                }
                var0 = var21;
                let var24 = self.global27;
                let var25: i32;
                if var24 != 0 {
                    let var26 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var0);
                    var25 = var26;
                } else {
                    let var27 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var0);
                    var25 = var27;
                }
                var0 = var25;
                let var28 = self.global28;
                let var29: i32;
                if var28 != 0 {
                    let var30 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var0);
                    var29 = var30;
                } else {
                    let var31 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var0);
                    var29 = var31;
                }
                var0 = var29;
                let var32 = self.global29;
                let var33: i32;
                if var32 != 0 {
                    let var34 = self.wasm_helpers_index_resetBitOnByte(imports, 3i32, var0);
                    var33 = var34;
                } else {
                    let var35 = self.wasm_helpers_index_setBitOnByte(imports, 3i32, var0);
                    var33 = var35;
                }
                var0 = var33;
            }
        }
        var0 | 240i32
    }
    fn wasm_memory_readTraps_checkReadTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        var1 = 32768i32;
        if (var0 < var1) as i32 != 0 {
            return -1i32;
        }
        var1 = (var0 >= 32768i32) as i32;
        let var2: i32;
        if var1 != 0 {
            var2 = (var0 < 40960i32) as i32;
        } else {
            var2 = var1;
        }
        if var2 & 1i32 != 0 {
            return -1i32;
        }
        var1 = (var0 >= 57344i32) as i32;
        let var3: i32;
        if var1 != 0 {
            var3 = (var0 < 65024i32) as i32;
        } else {
            var3 = var1;
        }
        if var3 & 1i32 != 0 {
            let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0.wrapping_sub(8192i32));
            return var4;
        }
        var1 = (var0 >= 65024i32) as i32;
        let var5: i32;
        if var1 != 0 {
            var5 = (var0 <= 65183i32) as i32;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            let var6 = self.global73;
            if (var6 < 2i32) as i32 != 0 {
                return 255i32;
            }
            return -1i32;
        }
        if (var0 == 65348i32) as i32 != 0 {
            let var7 = self.global59;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var7);
            let var8 = self.global59;
            return var8;
        }
        var1 = (var0 >= 65296i32) as i32;
        let var9: i32;
        if var1 != 0 {
            var9 = (var0 <= 65318i32) as i32;
        } else {
            var9 = var1;
        }
        if var9 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
            let var10 = self.wasm_sound_registers_SoundRegisterReadTraps(imports, var0);
            return var10;
        }
        var1 = (var0 >= 65328i32) as i32;
        let var11: i32;
        if var1 != 0 {
            var11 = (var0 <= 65343i32) as i32;
        } else {
            var11 = var1;
        }
        if var11 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
            return -1i32;
        }
        if (var0 == 65284i32) as i32 != 0 {
            let var12 = self.global63;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var12);
            let var13 = self.global63;
            return var13;
        }
        if (var0 == 65285i32) as i32 != 0 {
            let var14 = self.global193;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var14);
            let var15 = self.global193;
            return var15;
        }
        if (var0 == 65280i32) as i32 != 0 {
            let var16 = self.wasm_joypad_index_getJoypadState(imports);
            return var16;
        }
        -1i32
    }
    fn wasm_memory_load_eightBitLoadFromGBMemoryWithTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_readTraps_checkReadTraps(imports, var0);
        var1 = var2;
        if (var1 == -1i32) as i32 != 0 {
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
            return var3;
        }
        var1 & 255i32
    }
    fn wasm_memory_dma_hdmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        'label0: loop {
            if (var4 < var2) as i32 != 0 {
                let var6 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var0.wrapping_add(var4));
                var5 = var6;
                var3 = var1.wrapping_add(var4);
                'label1: loop {
                    if (var3 > 40959i32) as i32 != 0 {
                        var3 = var3.wrapping_sub(8192i32);
                        continue 'label1;
                    }
                    break;
                }
                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var3, var5);
                var4 = var4.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
        var3 = 32i32;
        let var7 = self.global68;
        if var7 != 0 {
            var3 = 64i32;
        }
        let var8 = self.global179;
        self.global179 = var8.wrapping_add(var3.wrapping_mul(var2 / 16i32));
    }
    fn wasm_memory_dma_startHdmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global36;
        if (var4 == 0) as i32 != 0 {
            return;
        }
        let var5 = self.global185;
        let var6: i32;
        if var5 != 0 {
            let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
            var6 = (var7 == 0) as i32;
        } else {
            let var8 = self.global185;
            var6 = var8;
        }
        if var6 & 1i32 != 0 {
            self.global185 = 0i32;
            let var9 = self.global184;
            let var10 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var9);
            var1 = var10;
            let var11 = self.global184;
            let var12 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, var1);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var11, var12);
            return;
        }
        let var13 = self.wasm_memory_dma_getHdmaSourceFromMemory(imports);
        var1 = var13;
        let var14 = self.wasm_memory_dma_getHdmaDestinationFromMemory(imports);
        var2 = var14;
        let var15 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, var0);
        var3 = var15.wrapping_add(1i32).wrapping_mul(16i32);
        let var16 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        if var16 != 0 {
            self.global185 = 1i32;
            self.global190 = var3;
            self.global191 = var1;
            self.global192 = var2;
            let var17 = self.global184;
            let var18 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, var0);
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var17, var18);
        } else {
            self.wasm_memory_dma_hdmaTransfer(imports, var1, var2, var3);
            let var19 = self.global184;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var19, 255i32);
        }
    }
    fn wasm_memory_memory_storePaletteByteInWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        var3 = var0 & 63i32;
        if var2 != 0 {
            var3 = var3.wrapping_add(64i32);
        }
        self.memory.store8(var3.wrapping_add(67584i32) as usize, var1 as u8);
    }
    fn wasm_graphics_palette_incrementPaletteIndexIfSet<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        if var2 != 0 {
            let var3 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, var0.wrapping_add(1i32));
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var1, var3);
        }
    }
    fn wasm_graphics_palette_writeColorPaletteToMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global196;
        var2 = (var0 == var4) as i32;
        let var5: i32;
        if var2 != 0 {
            var5 = var2;
        } else {
            let var6 = self.global195;
            var5 = (var0 == var6) as i32;
        }
        if var5 & 1i32 != 0 {
            let var7 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0.wrapping_sub(1i32));
            let var8 = self.wasm_helpers_index_resetBitOnByte(imports, 6i32, var7);
            var2 = var8;
            let var9 = self.global195;
            if (var0 == var9) as i32 != 0 {
                var3 = 1i32;
            }
            self.wasm_memory_memory_storePaletteByteInWasmMemory(imports, var2, var1, var3);
            self.wasm_graphics_palette_incrementPaletteIndexIfSet(imports, var2, var0.wrapping_sub(1i32));
        }
    }
    fn wasm_timers_index_Timers_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        256i32
    }
    fn wasm_timers_index__checkDividerRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global198;
        self.global198 = var1.wrapping_add(var0);
        let var2 = self.global198;
        let var3 = self.wasm_timers_index_Timers_batchProcessCycles(imports);
        if (var2 >= var3) as i32 != 0 {
            let var4 = self.global198;
            let var5 = self.wasm_timers_index_Timers_batchProcessCycles(imports);
            self.global198 = var4.wrapping_sub(var5);
            let var6 = self.global63;
            self.global63 = var6.wrapping_add(1i32);
            let var7 = self.global63;
            if (var7 > 255i32) as i32 != 0 {
                self.global63 = 0i32;
            }
        }
    }
    fn wasm_interrupts_index_requestTimerInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global200 = 1i32;
        self.wasm_interrupts_index__requestInterrupt(imports, 2i32);
    }
    fn wasm_timers_index_updateTimers<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.wasm_timers_index__checkDividerRegister(imports, var0);
        let var1 = self.global64;
        if (var1 == 0) as i32 != 0 {
            return;
        }
        let var2 = self.global66;
        self.global66 = var2.wrapping_add(var0);
        'label0: loop {
            let var3 = self.global66;
            let var4 = self.global67;
            if (var3 >= var4) as i32 != 0 {
                let var5 = self.global66;
                let var6 = self.global67;
                self.global66 = var5.wrapping_sub(var6);
                let var7 = self.global193;
                if (var7 >= 255i32) as i32 != 0 {
                    let var8 = self.global199;
                    self.global193 = var8;
                    self.wasm_interrupts_index_requestTimerInterrupt(imports);
                } else {
                    let var9 = self.global193;
                    self.global193 = var9.wrapping_add(1i32);
                }
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_timers_index_batchProcessTimers<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.wasm_timers_index_Timers_batchProcessCycles(imports);
        var0 = var1;
        let var2 = self.global64;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.global67;
            var3 = (var4 < var0) as i32;
        } else {
            let var5 = self.global64;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global67;
            var0 = var6;
        }
        let var7 = self.global197;
        if (var7 < var0) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var8 = self.global197;
            if (var8 >= var0) as i32 != 0 {
                self.wasm_timers_index_updateTimers(imports, var0);
                let var9 = self.global197;
                self.global197 = var9.wrapping_sub(var0);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_timers_index_Timers_updateDividerRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global63 = 0i32;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65284i32, 0i32);
        self.global66 = 0i32;
        let var1 = self.global199;
        self.global193 = var1;
    }
    fn wasm_timers_index_Timers_updateTimerCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global193 = var0;
    }
    fn wasm_timers_index_Timers_updateTimerModulo<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global199 = var0;
    }
    fn wasm_joypad_index_Joypad_updateJoypad<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global30 = var0 ^ 255i32;
        let var1 = self.global30;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var1);
        self.global31 = var2;
        let var3 = self.global30;
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var3);
        self.global32 = var4;
    }
    fn wasm_interrupts_index_Interrupts_updateInterruptRequested<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
        self.global201 = var1;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var0);
        self.global202 = var2;
        let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global200 = var3;
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
        self.global33 = var4;
        self.global40 = var0;
    }
    fn wasm_interrupts_index_Interrupts_updateInterruptEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
        self.global203 = var1;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var0);
        self.global204 = var2;
        let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global205 = var3;
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
        self.global206 = var4;
        self.global207 = var0;
    }
    fn wasm_memory_writeTraps_checkWriteTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        var2 = 32768i32;
        if (var0 < var2) as i32 != 0 {
            self.wasm_memory_banking_handleBanking(imports, var0, var1);
            return 0i32;
        }
        var2 = (var0 >= 32768i32) as i32;
        let var4: i32;
        if var2 != 0 {
            var4 = (var0 < 40960i32) as i32;
        } else {
            var4 = var2;
        }
        if var4 & 1i32 != 0 {
            return 1i32;
        }
        var3 = 65024i32;
        var2 = (var0 >= 57344i32) as i32;
        let var5: i32;
        if var2 != 0 {
            var5 = (var0 < 65024i32) as i32;
        } else {
            var5 = var2;
        }
        if var5 & 1i32 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0.wrapping_sub(8192i32), var1);
            return 1i32;
        }
        var2 = (var0 >= 65024i32) as i32;
        let var6: i32;
        if var2 != 0 {
            var6 = (var0 <= 65183i32) as i32;
        } else {
            var6 = var2;
        }
        if var6 & 1i32 != 0 {
            let var7 = self.global73;
            if (var7 < 2i32) as i32 != 0 {
                return 0i32;
            }
            return 1i32;
        }
        var2 = (var0 >= 65184i32) as i32;
        let var8: i32;
        if var2 != 0 {
            var8 = (var0 <= 65279i32) as i32;
        } else {
            var8 = var2;
        }
        if var8 & 1i32 != 0 {
            return 0i32;
        }
        var2 = (var0 >= 65296i32) as i32;
        let var9: i32;
        if var2 != 0 {
            var9 = (var0 <= 65318i32) as i32;
        } else {
            var9 = var2;
        }
        if var9 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
            let var10 = self.wasm_sound_registers_SoundRegisterWriteTraps(imports, var0, var1);
            return var10;
        }
        var2 = (var0 >= 65328i32) as i32;
        let var11: i32;
        if var2 != 0 {
            var11 = (var0 <= 65343i32) as i32;
        } else {
            var11 = var2;
        }
        if var11 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
        }
        var2 = (var0 >= 65344i32) as i32;
        let var12: i32;
        if var2 != 0 {
            var12 = (var0 <= 65355i32) as i32;
        } else {
            var12 = var2;
        }
        if var12 & 1i32 != 0 {
            if (var0 == 65344i32) as i32 != 0 {
                self.wasm_graphics_lcd_Lcd_updateLcdControl(imports, var1);
                return 1i32;
            }
            if (var0 == 65348i32) as i32 != 0 {
                self.global59 = 0i32;
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, 0i32);
                return 0i32;
            }
            if (var0 == 65349i32) as i32 != 0 {
                self.global178 = var1;
                return 1i32;
            }
            if (var0 == 65350i32) as i32 != 0 {
                self.wasm_memory_dma_startDmaTransfer(imports, var1);
                return 1i32;
            }
            'label0: loop {
                'label1: loop {
                    'label2: loop {
                        'label3: loop {
                            'label4: loop {
                                'label5: loop {
                                    match var0.wrapping_sub(65346i32) {
                                        0 => break 'label3,
                                        1 => break 'label4,
                                        2 => break 'label5,
                                        3 => break 'label5,
                                        4 => break 'label5,
                                        5 => break 'label5,
                                        6 => break 'label5,
                                        7 => break 'label5,
                                        8 => break 'label1,
                                        9 => break 'label2,
                                        _ => break 'label5,
                                    }
                                    break;
                                }
                                break 'label0;
                                break;
                            }
                            self.global180 = var1;
                            return 1i32;
                            break;
                        }
                        self.global181 = var1;
                        return 1i32;
                        break;
                    }
                    self.global182 = var1;
                    return 1i32;
                    break;
                }
                self.global183 = var1;
                return 1i32;
                break;
            }
            return 1i32;
        }
        let var13 = self.global184;
        if (var0 == var13) as i32 != 0 {
            self.wasm_memory_dma_startHdmaTransfer(imports, var1);
            return 0i32;
        }
        let var14 = self.global39;
        var2 = (var0 == var14) as i32;
        let var15: i32;
        if var2 != 0 {
            var15 = var2;
        } else {
            let var16 = self.global37;
            var15 = (var0 == var16) as i32;
        }
        if var15 & 1i32 != 0 {
            let var17 = self.global185;
            if var17 != 0 {
                let var18 = self.global191;
                var2 = (var18 >= 16384i32) as i32;
                let var19: i32;
                if var2 != 0 {
                    let var20 = self.global191;
                    var19 = (var20 <= 32767i32) as i32;
                } else {
                    var19 = var2;
                }
                var2 = var19 & 1i32;
                let var21: i32;
                if var2 != 0 {
                    var21 = var2;
                } else {
                    let var22 = self.global191;
                    var2 = (var22 >= 53248i32) as i32;
                    let var23: i32;
                    if var2 != 0 {
                        let var24 = self.global191;
                        var23 = (var24 <= 57343i32) as i32;
                    } else {
                        var23 = var2;
                    }
                    var21 = var23 & 1i32;
                }
                if var21 & 1i32 != 0 {
                    return 0i32;
                }
            }
        }
        let var25 = self.global194;
        var2 = (var0 >= var25) as i32;
        let var26: i32;
        if var2 != 0 {
            let var27 = self.global195;
            var26 = (var0 <= var27) as i32;
        } else {
            var26 = var2;
        }
        if var26 & 1i32 != 0 {
            self.wasm_graphics_palette_writeColorPaletteToMemory(imports, var0, var1);
            return 1i32;
        }
        var2 = (var0 >= 65284i32) as i32;
        let var28: i32;
        if var2 != 0 {
            var28 = (var0 <= 65287i32) as i32;
        } else {
            var28 = var2;
        }
        if var28 & 1i32 != 0 {
            self.wasm_timers_index_batchProcessTimers(imports);
            'label6: loop {
                'label7: loop {
                    'label8: loop {
                        'label9: loop {
                            'label10: loop {
                                'label11: loop {
                                    match var0.wrapping_sub(65284i32) {
                                        0 => break 'label10,
                                        1 => break 'label9,
                                        2 => break 'label8,
                                        3 => break 'label7,
                                        _ => break 'label11,
                                    }
                                    break;
                                }
                                break 'label6;
                                break;
                            }
                            self.wasm_timers_index_Timers_updateDividerRegister(imports, var1);
                            return 0i32;
                            break;
                        }
                        self.wasm_timers_index_Timers_updateTimerCounter(imports, var1);
                        return 1i32;
                        break;
                    }
                    self.wasm_timers_index_Timers_updateTimerModulo(imports, var1);
                    return 1i32;
                    break;
                }
                self.wasm_timers_index_Timers_updateTimerControl(imports, var1);
                return 1i32;
                break;
            }
            return 1i32;
        }
        if (var0 == 65280i32) as i32 != 0 {
            self.wasm_joypad_index_Joypad_updateJoypad(imports, var1);
        }
        if (var0 == 65295i32) as i32 != 0 {
            self.wasm_interrupts_index_Interrupts_updateInterruptRequested(imports, var1);
            return 1i32;
        }
        if (var0 == 65535i32) as i32 != 0 {
            self.wasm_interrupts_index_Interrupts_updateInterruptEnabled(imports, var1);
            return 1i32;
        }
        1i32
    }
    fn wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var1);
        if var2 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var1);
        }
    }
    fn wasm_cpu_flags_setFlagBit<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = 1i32.wrapping_shl(var0 as u32) & 255i32;
        if (var1 > 0i32) as i32 != 0 {
            let var3 = self.global49;
            self.global49 = (var3 | var2) & 255i32;
        } else {
            let var4 = self.global49;
            self.global49 = var4 & (var2 ^ 255i32);
        }
        let var5 = self.global49;
        var5
    }
    fn wasm_cpu_flags_setHalfCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 5i32, var0);
        var1;
    }
    fn wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        if (var1 >= 0i32) as i32 != 0 {
            if (var0 & 15i32).wrapping_add(var1 & 15i32) & 16i32 != 0 {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            }
        } else {
            var2 = var1;
            if ((({ let a = var2; let b = 0i32.wrapping_sub(var2); if (var2 > 0i32) as i32 != 0 { a } else { b } }) & 15i32) as u32 > (var0 & 15i32) as u32) as i32 != 0 {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            }
        }
    }
    fn wasm_cpu_flags_setZeroFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 7i32, var0);
        var1;
    }
    fn wasm_cpu_flags_setSubtractFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 6i32, var0);
        var1;
    }
    fn wasm_cpu_flags_setCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 4i32, var0);
        var1;
    }
    fn wasm_helpers_index_rotateByteLeft<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        (var0.wrapping_shl(1i32 as u32) & 255i32 | (var0 as u32).wrapping_shr(7i32 as u32) as i32) & 255i32
    }
    fn wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.wasm_helpers_index_splitHighByte(imports, var1);
        var2 = var3;
        let var4 = self.wasm_helpers_index_splitLowByte(imports, var1);
        var1 = var4;
        let var5 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var1);
        if var5 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var1);
        }
        var0 = var0.wrapping_add(1i32);
        let var6 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var2);
        if var6 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var2);
        }
    }
    fn wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        if var2 != 0 {
            var2 = var0 ^ var1 ^ var0.wrapping_add(var1);
            if var2 & 16i32 != 0 {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            }
            if var2 & 256i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
        } else {
            var2 = var0.wrapping_add(var1 & 65535i32) & 65535i32;
            if ((var2 as u32) < var0 as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
            if (var0 ^ var1 & 65535i32 ^ var2) & 4096i32 != 0 {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            }
        }
    }
    fn wasm_helpers_index_rotateByteRight<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        ((var0 as u32).wrapping_shr(1i32 as u32) as i32 | var0.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    fn wasm_cpu_opcodes_handleOpcode0x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0 {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    let var3 = self.wasm_helpers_index_splitHighByte(imports, var2);
                                                                    self.global43 = var3 & 255i32;
                                                                    let var4 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    let var5 = self.wasm_helpers_index_splitLowByte(imports, var4);
                                                                    self.global44 = var5 & 255i32;
                                                                    let var6 = self.global50;
                                                                    self.global50 = var6.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global43;
                                                                let var8 = self.global44;
                                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                                let var10 = self.global42;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var9, var10);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var11 = self.global43;
                                                            let var12 = self.global44;
                                                            let var13 = self.wasm_helpers_index_concatenateBytes(imports, var11, var12);
                                                            var0 = (var13 & 65535i32).wrapping_add(1i32) & 65535i32;
                                                            let var14 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global43 = var14 & 255i32;
                                                            let var15 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global44 = var15 & 255i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var16 = self.global43;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var16, 1i32);
                                                        let var17 = self.global43;
                                                        self.global43 = var17.wrapping_add(1i32) & 255i32;
                                                        let var18 = self.global43;
                                                        if var18 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var19 = self.global43;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var19, -1i32);
                                                    let var20 = self.global43;
                                                    self.global43 = var20.wrapping_sub(1i32) & 255i32;
                                                    let var21 = self.global43;
                                                    if var21 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var22 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                self.global43 = var22;
                                                let var23 = self.global50;
                                                self.global50 = var23.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var24 = self.global42;
                                            if (var24 & 128i32 == 128i32) as i32 != 0 {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            } else {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
                                            }
                                            let var25 = self.global42;
                                            let var26 = self.wasm_helpers_index_rotateByteLeft(imports, var25);
                                            self.global42 = var26;
                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            return 4i32;
                                            break;
                                        }
                                        let var27 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                        let var28 = self.global51;
                                        self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var27, var28);
                                        let var29 = self.global50;
                                        self.global50 = var29.wrapping_add(2i32) & 65535i32;
                                        return 20i32;
                                        break;
                                    }
                                    let var30 = self.global47;
                                    let var31 = self.global48;
                                    let var32 = self.wasm_helpers_index_concatenateBytes(imports, var30, var31);
                                    var0 = var32 & 65535i32;
                                    let var33 = self.global43;
                                    let var34 = self.global44;
                                    let var35 = self.wasm_helpers_index_concatenateBytes(imports, var33, var34);
                                    var1 = var35 & 65535i32;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var0, var1, 0i32);
                                    var0 = var0.wrapping_add(var1) & 65535i32;
                                    let var36 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                    self.global47 = var36 & 255i32;
                                    let var37 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                    self.global48 = var37 & 255i32;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var38 = self.global43;
                                let var39 = self.global44;
                                let var40 = self.wasm_helpers_index_concatenateBytes(imports, var38, var39);
                                let var41 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var40);
                                self.global42 = var41 & 255i32;
                                return 8i32;
                                break;
                            }
                            let var42 = self.global43;
                            let var43 = self.global44;
                            let var44 = self.wasm_helpers_index_concatenateBytes(imports, var42, var43);
                            var0 = (var44 & 65535i32).wrapping_sub(1i32) & 65535i32;
                            let var45 = self.wasm_helpers_index_splitHighByte(imports, var0);
                            self.global43 = var45 & 255i32;
                            let var46 = self.wasm_helpers_index_splitLowByte(imports, var0);
                            self.global44 = var46 & 255i32;
                            return 8i32;
                            break;
                        }
                        let var47 = self.global44;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var47, 1i32);
                        let var48 = self.global44;
                        self.global44 = var48.wrapping_add(1i32) & 255i32;
                        let var49 = self.global44;
                        if var49 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var50 = self.global44;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var50, -1i32);
                    let var51 = self.global44;
                    self.global44 = var51.wrapping_sub(1i32) & 255i32;
                    let var52 = self.global44;
                    if var52 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                let var53 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.global44 = var53;
                let var54 = self.global50;
                self.global50 = var54.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var55 = self.global42;
            if ((var55 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
            let var56 = self.global42;
            let var57 = self.wasm_helpers_index_rotateByteRight(imports, var56);
            self.global42 = var57;
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_flags_getCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global49;
        (var0 as u32).wrapping_shr(4i32 as u32) as i32 & 1i32
    }
    fn wasm_helpers_index_rotateByteLeftThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_cpu_flags_getCarryFlag(imports);
        (var0.wrapping_shl(1i32 as u32) & 255i32 | var1) & 255i32
    }
    fn wasm_cpu_instructions_relativeJump<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global50;
        self.global50 = var1.wrapping_add(var0.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32)) & 65535i32;
        let var2 = self.global50;
        self.global50 = var2.wrapping_add(1i32) & 65535i32;
    }
    fn wasm_helpers_index_rotateByteRightThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_cpu_flags_getCarryFlag(imports);
        ((var0 as u32).wrapping_shr(1i32 as u32) as i32 | var1.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    fn wasm_cpu_opcodes_handleOpcode1x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(16i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var2 = self.global36;
                                                                        if var2 != 0 {
                                                                            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, 65357i32);
                                                                            var0 = var3;
                                                                            let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
                                                                            if var4 != 0 {
                                                                                let var5 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                                                                                var0 = var5;
                                                                                let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
                                                                                let var7: i32;
                                                                                if var6 != 0 {
                                                                                    self.global68 = 0i32;
                                                                                    let var8 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, var0);
                                                                                    var7 = var8;
                                                                                } else {
                                                                                    self.global68 = 1i32;
                                                                                    let var9 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, var0);
                                                                                    var7 = var9;
                                                                                }
                                                                                var0 = var7;
                                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, 65357i32, var0);
                                                                                return 76i32;
                                                                            }
                                                                        }
                                                                        let var10 = self.global50;
                                                                        self.global50 = var10.wrapping_add(1i32) & 65535i32;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var11 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    let var12 = self.wasm_helpers_index_splitHighByte(imports, var11);
                                                                    self.global45 = var12 & 255i32;
                                                                    let var13 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    let var14 = self.wasm_helpers_index_splitLowByte(imports, var13);
                                                                    self.global46 = var14 & 255i32;
                                                                    let var15 = self.global50;
                                                                    self.global50 = var15.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var16 = self.global45;
                                                                let var17 = self.global46;
                                                                let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                                                                let var19 = self.global42;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var18, var19);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var20 = self.global45;
                                                            let var21 = self.global46;
                                                            let var22 = self.wasm_helpers_index_concatenateBytes(imports, var20, var21);
                                                            var0 = (var22 & 65535i32).wrapping_add(1i32) & 65535i32;
                                                            let var23 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global45 = var23 & 255i32;
                                                            let var24 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global46 = var24 & 255i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var25 = self.global45;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var25, 1i32);
                                                        let var26 = self.global45;
                                                        self.global45 = var26.wrapping_add(1i32) & 255i32;
                                                        let var27 = self.global45;
                                                        if var27 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var28 = self.global45;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var28, -1i32);
                                                    let var29 = self.global45;
                                                    self.global45 = var29.wrapping_sub(1i32) & 255i32;
                                                    let var30 = self.global45;
                                                    if var30 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var31 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                self.global45 = var31;
                                                let var32 = self.global50;
                                                self.global50 = var32.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            var0 = 0i32;
                                            let var33 = self.global42;
                                            if (var33 & 128i32 == 128i32) as i32 != 0 {
                                                var0 = 1i32;
                                            }
                                            let var34 = self.global42;
                                            let var35 = self.wasm_helpers_index_rotateByteLeftThroughCarry(imports, var34);
                                            self.global42 = var35;
                                            if var0 != 0 {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            } else {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
                                            }
                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            return 4i32;
                                            break;
                                        }
                                        let var36 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                        self.wasm_cpu_instructions_relativeJump(imports, var36);
                                        return 12i32;
                                        break;
                                    }
                                    let var37 = self.global47;
                                    let var38 = self.global48;
                                    let var39 = self.wasm_helpers_index_concatenateBytes(imports, var37, var38);
                                    var0 = var39 & 65535i32;
                                    let var40 = self.global45;
                                    let var41 = self.global46;
                                    let var42 = self.wasm_helpers_index_concatenateBytes(imports, var40, var41);
                                    var1 = var42 & 65535i32;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var0, var1, 0i32);
                                    var0 = var0.wrapping_add(var1) & 65535i32;
                                    let var43 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                    self.global47 = var43 & 255i32;
                                    let var44 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                    self.global48 = var44 & 255i32;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var45 = self.global45;
                                let var46 = self.global46;
                                let var47 = self.wasm_helpers_index_concatenateBytes(imports, var45, var46);
                                let var48 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var47 & 65535i32);
                                self.global42 = var48 & 255i32;
                                return 8i32;
                                break;
                            }
                            let var49 = self.global45;
                            let var50 = self.global46;
                            let var51 = self.wasm_helpers_index_concatenateBytes(imports, var49, var50);
                            var0 = (var51 & 65535i32).wrapping_sub(1i32) & 65535i32;
                            let var52 = self.wasm_helpers_index_splitHighByte(imports, var0);
                            self.global45 = var52 & 255i32;
                            let var53 = self.wasm_helpers_index_splitLowByte(imports, var0);
                            self.global46 = var53 & 255i32;
                            return 8i32;
                            break;
                        }
                        let var54 = self.global46;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var54, 1i32);
                        let var55 = self.global46;
                        self.global46 = var55.wrapping_add(1i32) & 255i32;
                        let var56 = self.global46;
                        if var56 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var57 = self.global46;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var57, -1i32);
                    let var58 = self.global46;
                    self.global46 = var58.wrapping_sub(1i32) & 255i32;
                    let var59 = self.global46;
                    if var59 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                let var60 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.global46 = var60;
                let var61 = self.global50;
                self.global50 = var61.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            var0 = 0i32;
            let var62 = self.global42;
            if (var62 & 1i32 == 1i32) as i32 != 0 {
                var0 = 1i32;
            }
            let var63 = self.global42;
            let var64 = self.wasm_helpers_index_rotateByteRightThroughCarry(imports, var63);
            self.global42 = var64;
            if var0 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_flags_getZeroFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global49;
        (var0 as u32).wrapping_shr(7i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_flags_getHalfCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global49;
        (var0 as u32).wrapping_shr(5i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_flags_getSubtractFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global49;
        (var0 as u32).wrapping_shr(6i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_opcodes_handleOpcode2x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(32i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var2 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                        if var2 != 0 {
                                                                            let var3 = self.global50;
                                                                            self.global50 = var3.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            let var4 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                                            self.wasm_cpu_instructions_relativeJump(imports, var4);
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var5 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    var0 = var5;
                                                                    let var6 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                    self.global47 = var6 & 255i32;
                                                                    let var7 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                    self.global48 = var7 & 255i32;
                                                                    let var8 = self.global50;
                                                                    self.global50 = var8.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var9 = self.global47;
                                                                let var10 = self.global48;
                                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                                var0 = var11 & 65535i32;
                                                                let var12 = self.global42;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var0, var12);
                                                                var0 = var0.wrapping_add(1i32) & 65535i32;
                                                                let var13 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                self.global47 = var13 & 255i32;
                                                                let var14 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                self.global48 = var14 & 255i32;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var15 = self.global47;
                                                            let var16 = self.global48;
                                                            let var17 = self.wasm_helpers_index_concatenateBytes(imports, var15, var16);
                                                            var0 = (var17 & 65535i32).wrapping_add(1i32) & 65535i32;
                                                            let var18 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global47 = var18 & 255i32;
                                                            let var19 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global48 = var19 & 255i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var20 = self.global47;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var20, 1i32);
                                                        let var21 = self.global47;
                                                        self.global47 = var21.wrapping_add(1i32) & 255i32;
                                                        let var22 = self.global47;
                                                        if var22 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var23 = self.global47;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var23, -1i32);
                                                    let var24 = self.global47;
                                                    self.global47 = var24.wrapping_sub(1i32) & 255i32;
                                                    let var25 = self.global47;
                                                    if var25 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var26 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                self.global47 = var26;
                                                let var27 = self.global50;
                                                self.global50 = var27.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var28 = self.wasm_cpu_flags_getHalfCarryFlag(imports);
                                            if (var28 as u32 > 0i32 as u32) as i32 != 0 {
                                                var1 = 6i32;
                                            }
                                            let var29 = self.wasm_cpu_flags_getCarryFlag(imports);
                                            if (var29 as u32 > 0i32 as u32) as i32 != 0 {
                                                var1 = (var1 | 96i32) & 255i32;
                                            }
                                            let var30 = self.wasm_cpu_flags_getSubtractFlag(imports);
                                            let var31: i32;
                                            if (var30 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var32 = self.global42;
                                                var31 = var32.wrapping_sub(var1) & 255i32;
                                            } else {
                                                let var33 = self.global42;
                                                if ((var33 & 15i32) as u32 > 9i32 as u32) as i32 != 0 {
                                                    var1 = (var1 | 6i32) & 255i32;
                                                }
                                                let var34 = self.global42;
                                                if (var34 as u32 > 153i32 as u32) as i32 != 0 {
                                                    var1 = (var1 | 96i32) & 255i32;
                                                }
                                                let var35 = self.global42;
                                                var31 = var35.wrapping_add(var1) & 255i32;
                                            }
                                            var0 = var31;
                                            if var0 != 0 {
                                                self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                            } else {
                                                self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                            }
                                            if var1 & 96i32 != 0 {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            } else {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
                                            }
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            self.global42 = var0;
                                            return 4i32;
                                            break;
                                        }
                                        let var36 = self.wasm_cpu_flags_getZeroFlag(imports);
                                        if (var36 as u32 > 0i32 as u32) as i32 != 0 {
                                            let var37 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                            self.wasm_cpu_instructions_relativeJump(imports, var37);
                                            return 12i32;
                                        } else {
                                            let var38 = self.global50;
                                            self.global50 = var38.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var39 = self.global47;
                                    let var40 = self.global48;
                                    let var41 = self.wasm_helpers_index_concatenateBytes(imports, var39, var40);
                                    var1 = var41 & 65535i32;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var1, var1, 0i32);
                                    var1 = var1.wrapping_mul(2i32) & 65535i32;
                                    let var42 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                    self.global47 = var42 & 255i32;
                                    let var43 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                    self.global48 = var43 & 255i32;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var44 = self.global47;
                                let var45 = self.global48;
                                let var46 = self.wasm_helpers_index_concatenateBytes(imports, var44, var45);
                                var1 = var46 & 65535i32;
                                let var47 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var1);
                                self.global42 = var47 & 255i32;
                                var1 = var1.wrapping_add(1i32) & 65535i32;
                                let var48 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                self.global47 = var48 & 255i32;
                                let var49 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                self.global48 = var49 & 255i32;
                                return 8i32;
                                break;
                            }
                            let var50 = self.global47;
                            let var51 = self.global48;
                            let var52 = self.wasm_helpers_index_concatenateBytes(imports, var50, var51);
                            var1 = (var52 & 65535i32).wrapping_sub(1i32) & 65535i32;
                            let var53 = self.wasm_helpers_index_splitHighByte(imports, var1);
                            self.global47 = var53 & 255i32;
                            let var54 = self.wasm_helpers_index_splitLowByte(imports, var1);
                            self.global48 = var54 & 255i32;
                            return 8i32;
                            break;
                        }
                        let var55 = self.global48;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var55, 1i32);
                        let var56 = self.global48;
                        self.global48 = var56.wrapping_add(1i32) & 255i32;
                        let var57 = self.global48;
                        if var57 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var58 = self.global48;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var58, -1i32);
                    let var59 = self.global48;
                    self.global48 = var59.wrapping_sub(1i32) & 255i32;
                    let var60 = self.global48;
                    if var60 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                let var61 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.global48 = var61;
                let var62 = self.global50;
                self.global50 = var62.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var63 = self.global42;
            self.global42 = (var63 ^ -1i32) & 255i32;
            self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode3x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(48i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var3 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                                        if var3 != 0 {
                                                                            let var4 = self.global50;
                                                                            self.global50 = var4.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            let var5 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                                            self.wasm_cpu_instructions_relativeJump(imports, var5);
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var6 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    self.global51 = var6;
                                                                    let var7 = self.global50;
                                                                    self.global50 = var7.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var8 = self.global47;
                                                                let var9 = self.global48;
                                                                let var10 = self.wasm_helpers_index_concatenateBytes(imports, var8, var9);
                                                                var0 = var10 & 65535i32;
                                                                let var11 = self.global42;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var0, var11);
                                                                var0 = var0.wrapping_sub(1i32) & 65535i32;
                                                                let var12 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                self.global47 = var12 & 255i32;
                                                                let var13 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                self.global48 = var13 & 255i32;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var14 = self.global51;
                                                            self.global51 = var14.wrapping_add(1i32) & 65535i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var15 = self.global47;
                                                        let var16 = self.global48;
                                                        let var17 = self.wasm_helpers_index_concatenateBytes(imports, var15, var16);
                                                        var0 = var17 & 65535i32;
                                                        let var18 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var0);
                                                        var1 = var18 & 255i32;
                                                        var2 = 1i32;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, var2);
                                                        var1 = var1.wrapping_add(1i32) & 255i32;
                                                        if var1 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var0, var1);
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var19 = self.global47;
                                                    let var20 = self.global48;
                                                    let var21 = self.wasm_helpers_index_concatenateBytes(imports, var19, var20);
                                                    var2 = var21 & 65535i32;
                                                    let var22 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var2);
                                                    var1 = var22 & 255i32;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, -1i32);
                                                    var1 = var1.wrapping_sub(1i32) & 255i32;
                                                    if var1 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var2, var1);
                                                    return 12i32;
                                                    break;
                                                }
                                                let var23 = self.global47;
                                                let var24 = self.global48;
                                                let var25 = self.wasm_helpers_index_concatenateBytes(imports, var23, var24);
                                                let var26 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var25 & 65535i32, var26);
                                                let var27 = self.global50;
                                                self.global50 = var27.wrapping_add(1i32) & 65535i32;
                                                return 12i32;
                                                break;
                                            }
                                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            return 4i32;
                                            break;
                                        }
                                        let var28 = self.wasm_cpu_flags_getCarryFlag(imports);
                                        if (var28 == 1i32) as i32 != 0 {
                                            let var29 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                            self.wasm_cpu_instructions_relativeJump(imports, var29);
                                            return 12i32;
                                        } else {
                                            let var30 = self.global50;
                                            self.global50 = var30.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var31 = self.global47;
                                    let var32 = self.global48;
                                    let var33 = self.wasm_helpers_index_concatenateBytes(imports, var31, var32);
                                    var1 = var33 & 65535i32;
                                    let var34 = self.global51;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var1, var34, 0i32);
                                    let var35 = self.global51;
                                    var2 = var1.wrapping_add(var35) & 65535i32;
                                    let var36 = self.wasm_helpers_index_splitHighByte(imports, var2);
                                    self.global47 = var36 & 255i32;
                                    let var37 = self.wasm_helpers_index_splitLowByte(imports, var2);
                                    self.global48 = var37 & 255i32;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var38 = self.global47;
                                let var39 = self.global48;
                                let var40 = self.wasm_helpers_index_concatenateBytes(imports, var38, var39);
                                var2 = var40 & 65535i32;
                                let var41 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var2);
                                self.global42 = var41 & 255i32;
                                var2 = var2.wrapping_sub(1i32) & 65535i32;
                                let var42 = self.wasm_helpers_index_splitHighByte(imports, var2);
                                self.global47 = var42 & 255i32;
                                let var43 = self.wasm_helpers_index_splitLowByte(imports, var2);
                                self.global48 = var43 & 255i32;
                                return 8i32;
                                break;
                            }
                            let var44 = self.global51;
                            self.global51 = var44.wrapping_sub(1i32) & 65535i32;
                            return 8i32;
                            break;
                        }
                        let var45 = self.global42;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var45, 1i32);
                        let var46 = self.global42;
                        self.global42 = var46.wrapping_add(1i32) & 255i32;
                        let var47 = self.global42;
                        if var47 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var48 = self.global42;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var48, -1i32);
                    let var49 = self.global42;
                    self.global42 = var49.wrapping_sub(1i32) & 255i32;
                    let var50 = self.global42;
                    if var50 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                let var51 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.global42 = var51;
                let var52 = self.global50;
                self.global50 = var52.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            let var53 = self.wasm_cpu_flags_getCarryFlag(imports);
            if (var53 as u32 > 0i32 as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            }
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode4x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(64i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var1 = self.global44;
                                                                    self.global43 = var1;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var2 = self.global45;
                                                                self.global43 = var2;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var3 = self.global46;
                                                            self.global43 = var3;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var4 = self.global47;
                                                        self.global43 = var4;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var5 = self.global48;
                                                    self.global43 = var5;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var6 = self.global47;
                                                let var7 = self.global48;
                                                let var8 = self.wasm_helpers_index_concatenateBytes(imports, var6, var7);
                                                let var9 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var8);
                                                self.global43 = var9 & 255i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var10 = self.global42;
                                            self.global43 = var10;
                                            return 4i32;
                                            break;
                                        }
                                        let var11 = self.global43;
                                        self.global44 = var11;
                                        return 4i32;
                                        break;
                                    }
                                    return 4i32;
                                    break;
                                }
                                let var12 = self.global45;
                                self.global44 = var12;
                                return 4i32;
                                break;
                            }
                            let var13 = self.global46;
                            self.global44 = var13;
                            return 4i32;
                            break;
                        }
                        let var14 = self.global47;
                        self.global44 = var14;
                        return 4i32;
                        break;
                    }
                    let var15 = self.global48;
                    self.global44 = var15;
                    return 4i32;
                    break;
                }
                let var16 = self.global47;
                let var17 = self.global48;
                let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                let var19 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var18);
                self.global44 = var19 & 255i32;
                return 8i32;
                break;
            }
            let var20 = self.global42;
            self.global44 = var20;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode5x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(80i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.global45 = var1;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.global45 = var2;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var3 = self.global46;
                                                            self.global45 = var3;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var4 = self.global47;
                                                        self.global45 = var4;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var5 = self.global48;
                                                    self.global45 = var5;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var6 = self.global47;
                                                let var7 = self.global48;
                                                let var8 = self.wasm_helpers_index_concatenateBytes(imports, var6, var7);
                                                let var9 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var8);
                                                self.global45 = var9 & 255i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var10 = self.global42;
                                            self.global45 = var10;
                                            return 4i32;
                                            break;
                                        }
                                        let var11 = self.global43;
                                        self.global46 = var11;
                                        return 4i32;
                                        break;
                                    }
                                    let var12 = self.global44;
                                    self.global46 = var12;
                                    return 4i32;
                                    break;
                                }
                                let var13 = self.global45;
                                self.global46 = var13;
                                return 4i32;
                                break;
                            }
                            return 4i32;
                            break;
                        }
                        let var14 = self.global47;
                        self.global46 = var14;
                        return 4i32;
                        break;
                    }
                    let var15 = self.global48;
                    self.global46 = var15;
                    return 4i32;
                    break;
                }
                let var16 = self.global47;
                let var17 = self.global48;
                let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                let var19 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var18);
                self.global46 = var19 & 255i32;
                return 4i32;
                break;
            }
            let var20 = self.global42;
            self.global46 = var20;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode6x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(96i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.global47 = var1;
                                                                        return 8i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.global47 = var2;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var3 = self.global45;
                                                                self.global47 = var3;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var4 = self.global46;
                                                            self.global47 = var4;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var5 = self.global48;
                                                    self.global47 = var5;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var6 = self.global47;
                                                let var7 = self.global48;
                                                let var8 = self.wasm_helpers_index_concatenateBytes(imports, var6, var7);
                                                let var9 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var8);
                                                self.global47 = var9 & 255i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var10 = self.global42;
                                            self.global47 = var10;
                                            return 4i32;
                                            break;
                                        }
                                        let var11 = self.global43;
                                        self.global48 = var11;
                                        return 4i32;
                                        break;
                                    }
                                    let var12 = self.global44;
                                    self.global48 = var12;
                                    return 4i32;
                                    break;
                                }
                                let var13 = self.global45;
                                self.global48 = var13;
                                return 4i32;
                                break;
                            }
                            let var14 = self.global46;
                            self.global48 = var14;
                            return 4i32;
                            break;
                        }
                        let var15 = self.global47;
                        self.global48 = var15;
                        return 4i32;
                        break;
                    }
                    return 4i32;
                    break;
                }
                let var16 = self.global47;
                let var17 = self.global48;
                let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                let var19 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var18);
                self.global48 = var19 & 255i32;
                return 8i32;
                break;
            }
            let var20 = self.global42;
            self.global48 = var20;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode7x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(112i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global47;
                                                                        let var2 = self.global48;
                                                                        let var3 = self.wasm_helpers_index_concatenateBytes(imports, var1, var2);
                                                                        let var4 = self.global43;
                                                                        self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var3, var4);
                                                                        return 8i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global47;
                                                                    let var6 = self.global48;
                                                                    let var7 = self.wasm_helpers_index_concatenateBytes(imports, var5, var6);
                                                                    let var8 = self.global44;
                                                                    self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var7, var8);
                                                                    return 8i32;
                                                                    break;
                                                                }
                                                                let var9 = self.global47;
                                                                let var10 = self.global48;
                                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                                let var12 = self.global45;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var11, var12);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var13 = self.global47;
                                                            let var14 = self.global48;
                                                            let var15 = self.wasm_helpers_index_concatenateBytes(imports, var13, var14);
                                                            let var16 = self.global46;
                                                            self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var15, var16);
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var17 = self.global47;
                                                        let var18 = self.global48;
                                                        let var19 = self.wasm_helpers_index_concatenateBytes(imports, var17, var18);
                                                        let var20 = self.global47;
                                                        self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var19, var20);
                                                        return 8i32;
                                                        break;
                                                    }
                                                    let var21 = self.global47;
                                                    let var22 = self.global48;
                                                    let var23 = self.wasm_helpers_index_concatenateBytes(imports, var21, var22);
                                                    let var24 = self.global48;
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var23, var24);
                                                    return 8i32;
                                                    break;
                                                }
                                                let var25 = self.global185;
                                                if (var25 == 0) as i32 != 0 {
                                                    self.global70 = 1i32;
                                                }
                                                return 4i32;
                                                break;
                                            }
                                            let var26 = self.global47;
                                            let var27 = self.global48;
                                            let var28 = self.wasm_helpers_index_concatenateBytes(imports, var26, var27);
                                            let var29 = self.global42;
                                            self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var28, var29);
                                            return 8i32;
                                            break;
                                        }
                                        let var30 = self.global43;
                                        self.global42 = var30;
                                        return 4i32;
                                        break;
                                    }
                                    let var31 = self.global44;
                                    self.global42 = var31;
                                    return 4i32;
                                    break;
                                }
                                let var32 = self.global45;
                                self.global42 = var32;
                                return 4i32;
                                break;
                            }
                            let var33 = self.global46;
                            self.global42 = var33;
                            return 4i32;
                            break;
                        }
                        let var34 = self.global47;
                        self.global42 = var34;
                        return 4i32;
                        break;
                    }
                    let var35 = self.global48;
                    self.global42 = var35;
                    return 4i32;
                    break;
                }
                let var36 = self.global47;
                let var37 = self.global48;
                let var38 = self.wasm_helpers_index_concatenateBytes(imports, var36, var37);
                let var39 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var38);
                self.global42 = var39 & 255i32;
                return 8i32;
                break;
            }
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_flags_checkAndSetEightBitCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        if (var1 >= 0i32) as i32 != 0 {
            if (var0 as u32 > (var0.wrapping_add(var1 & 255i32) & 255i32) as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
        } else {
            var2 = var1;
            if (({ let a = var2; let b = 0i32.wrapping_sub(var2); if (var2 > 0i32) as i32 != 0 { a } else { b } }) > var0) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
        }
    }
    fn wasm_cpu_instructions_addARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global42;
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, var0);
        let var2 = self.global42;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var2, var0);
        let var3 = self.global42;
        self.global42 = var3.wrapping_add(var0) & 255i32;
        let var4 = self.global42;
        if var4 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
    }
    fn wasm_cpu_instructions_addAThroughCarryRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global42;
        let var3 = self.wasm_cpu_flags_getCarryFlag(imports);
        var1 = var2.wrapping_add(var0).wrapping_add(var3) & 255i32;
        let var4 = self.global42;
        if (var4 ^ var0 ^ var1) & 16i32 != 0 {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        }
        let var5 = self.global42;
        let var6 = self.wasm_cpu_flags_getCarryFlag(imports);
        if ((var5.wrapping_add(var0).wrapping_add(var6) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        self.global42 = var1;
        let var7 = self.global42;
        if var7 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
    }
    fn wasm_cpu_opcodes_handleOpcode8x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(128i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.wasm_cpu_instructions_addARegister(imports, var1);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.wasm_cpu_instructions_addARegister(imports, var2);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var3 = self.global45;
                                                                self.wasm_cpu_instructions_addARegister(imports, var3);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var4 = self.global46;
                                                            self.wasm_cpu_instructions_addARegister(imports, var4);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var5 = self.global47;
                                                        self.wasm_cpu_instructions_addARegister(imports, var5);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var6 = self.global48;
                                                    self.wasm_cpu_instructions_addARegister(imports, var6);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var7 = self.global47;
                                                let var8 = self.global48;
                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                let var10 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var9);
                                                self.wasm_cpu_instructions_addARegister(imports, var10 & 255i32);
                                                return 8i32;
                                                break;
                                            }
                                            let var11 = self.global42;
                                            self.wasm_cpu_instructions_addARegister(imports, var11);
                                            return 4i32;
                                            break;
                                        }
                                        let var12 = self.global43;
                                        self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var12);
                                        return 4i32;
                                        break;
                                    }
                                    let var13 = self.global44;
                                    self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var13);
                                    return 4i32;
                                    break;
                                }
                                let var14 = self.global45;
                                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var14);
                                return 4i32;
                                break;
                            }
                            let var15 = self.global46;
                            self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var15);
                            return 4i32;
                            break;
                        }
                        let var16 = self.global47;
                        self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var16);
                        return 4i32;
                        break;
                    }
                    let var17 = self.global48;
                    self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var17);
                    return 4i32;
                    break;
                }
                let var18 = self.global47;
                let var19 = self.global48;
                let var20 = self.wasm_helpers_index_concatenateBytes(imports, var18, var19);
                let var21 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var20);
                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var21 & 255i32);
                return 8i32;
                break;
            }
            let var22 = self.global42;
            self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var22);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_subARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global42;
        var1 = var0.wrapping_mul(-1i32);
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var2, var1);
        let var3 = self.global42;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var3, var1);
        let var4 = self.global42;
        self.global42 = var4.wrapping_sub(var0) & 255i32;
        let var5 = self.global42;
        if var5 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_instructions_subAThroughCarryRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global42;
        let var3 = self.wasm_cpu_flags_getCarryFlag(imports);
        var1 = var2.wrapping_sub(var0).wrapping_sub(var3) & 255i32;
        let var4 = self.global42;
        if (var4 ^ var0 ^ var1) & 16i32 != 0 {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        }
        let var5 = self.global42;
        let var6 = self.wasm_cpu_flags_getCarryFlag(imports);
        if ((var5.wrapping_sub(var0).wrapping_sub(var6) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        self.global42 = var1;
        let var7 = self.global42;
        if var7 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_opcodes_handleOpcode9x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(144i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.wasm_cpu_instructions_subARegister(imports, var1);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.wasm_cpu_instructions_subARegister(imports, var2);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var3 = self.global45;
                                                                self.wasm_cpu_instructions_subARegister(imports, var3);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var4 = self.global46;
                                                            self.wasm_cpu_instructions_subARegister(imports, var4);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var5 = self.global47;
                                                        self.wasm_cpu_instructions_subARegister(imports, var5);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var6 = self.global48;
                                                    self.wasm_cpu_instructions_subARegister(imports, var6);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var7 = self.global47;
                                                let var8 = self.global48;
                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                let var10 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var9);
                                                self.wasm_cpu_instructions_subARegister(imports, var10 & 255i32);
                                                return 8i32;
                                                break;
                                            }
                                            let var11 = self.global42;
                                            self.wasm_cpu_instructions_subARegister(imports, var11);
                                            return 4i32;
                                            break;
                                        }
                                        let var12 = self.global43;
                                        self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var12);
                                        return 4i32;
                                        break;
                                    }
                                    let var13 = self.global44;
                                    self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var13);
                                    return 4i32;
                                    break;
                                }
                                let var14 = self.global45;
                                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var14);
                                return 4i32;
                                break;
                            }
                            let var15 = self.global46;
                            self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var15);
                            return 4i32;
                            break;
                        }
                        let var16 = self.global47;
                        self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var16);
                        return 4i32;
                        break;
                    }
                    let var17 = self.global48;
                    self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var17);
                    return 4i32;
                    break;
                }
                let var18 = self.global47;
                let var19 = self.global48;
                let var20 = self.wasm_helpers_index_concatenateBytes(imports, var18, var19);
                let var21 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var20);
                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var21 & 255i32);
                return 8i32;
                break;
            }
            let var22 = self.global42;
            self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var22);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_andARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global42;
        self.global42 = var1 & var0 & 255i32;
        let var2 = self.global42;
        if var2 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
    }
    fn wasm_cpu_instructions_xorARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global42;
        self.global42 = (var1 ^ var0) & 255i32;
        let var2 = self.global42;
        if var2 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
    }
    fn wasm_cpu_opcodes_handleOpcodeAx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(160i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.wasm_cpu_instructions_andARegister(imports, var1);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.wasm_cpu_instructions_andARegister(imports, var2);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var3 = self.global45;
                                                                self.wasm_cpu_instructions_andARegister(imports, var3);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var4 = self.global46;
                                                            self.wasm_cpu_instructions_andARegister(imports, var4);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var5 = self.global47;
                                                        self.wasm_cpu_instructions_andARegister(imports, var5);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var6 = self.global48;
                                                    self.wasm_cpu_instructions_andARegister(imports, var6);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var7 = self.global47;
                                                let var8 = self.global48;
                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                let var10 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var9);
                                                self.wasm_cpu_instructions_andARegister(imports, var10 & 255i32);
                                                return 8i32;
                                                break;
                                            }
                                            let var11 = self.global42;
                                            self.wasm_cpu_instructions_andARegister(imports, var11);
                                            return 4i32;
                                            break;
                                        }
                                        let var12 = self.global43;
                                        self.wasm_cpu_instructions_xorARegister(imports, var12);
                                        return 4i32;
                                        break;
                                    }
                                    let var13 = self.global44;
                                    self.wasm_cpu_instructions_xorARegister(imports, var13);
                                    return 4i32;
                                    break;
                                }
                                let var14 = self.global45;
                                self.wasm_cpu_instructions_xorARegister(imports, var14);
                                return 4i32;
                                break;
                            }
                            let var15 = self.global46;
                            self.wasm_cpu_instructions_xorARegister(imports, var15);
                            return 4i32;
                            break;
                        }
                        let var16 = self.global47;
                        self.wasm_cpu_instructions_xorARegister(imports, var16);
                        return 4i32;
                        break;
                    }
                    let var17 = self.global48;
                    self.wasm_cpu_instructions_xorARegister(imports, var17);
                    return 4i32;
                    break;
                }
                let var18 = self.global47;
                let var19 = self.global48;
                let var20 = self.wasm_helpers_index_concatenateBytes(imports, var18, var19);
                let var21 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var20);
                self.wasm_cpu_instructions_xorARegister(imports, var21 & 255i32);
                return 8i32;
                break;
            }
            let var22 = self.global42;
            self.wasm_cpu_instructions_xorARegister(imports, var22);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_orARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global42;
        self.global42 = (var1 | var0) & 255i32;
        let var2 = self.global42;
        if var2 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
    }
    fn wasm_cpu_instructions_cpARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global42;
        var1 = var0.wrapping_mul(-1i32);
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var2, var1);
        let var3 = self.global42;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var3, var1);
        let var4 = self.global42;
        if var4.wrapping_add(var1) != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_opcodes_handleOpcodeBx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(176i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var1 = self.global43;
                                                                        self.wasm_cpu_instructions_orARegister(imports, var1);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var2 = self.global44;
                                                                    self.wasm_cpu_instructions_orARegister(imports, var2);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var3 = self.global45;
                                                                self.wasm_cpu_instructions_orARegister(imports, var3);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var4 = self.global46;
                                                            self.wasm_cpu_instructions_orARegister(imports, var4);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var5 = self.global47;
                                                        self.wasm_cpu_instructions_orARegister(imports, var5);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var6 = self.global48;
                                                    self.wasm_cpu_instructions_orARegister(imports, var6);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var7 = self.global47;
                                                let var8 = self.global48;
                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                let var10 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var9);
                                                self.wasm_cpu_instructions_orARegister(imports, var10 & 255i32);
                                                return 8i32;
                                                break;
                                            }
                                            let var11 = self.global42;
                                            self.wasm_cpu_instructions_orARegister(imports, var11);
                                            return 4i32;
                                            break;
                                        }
                                        let var12 = self.global43;
                                        self.wasm_cpu_instructions_cpARegister(imports, var12);
                                        return 4i32;
                                        break;
                                    }
                                    let var13 = self.global44;
                                    self.wasm_cpu_instructions_cpARegister(imports, var13);
                                    return 4i32;
                                    break;
                                }
                                let var14 = self.global45;
                                self.wasm_cpu_instructions_cpARegister(imports, var14);
                                return 4i32;
                                break;
                            }
                            let var15 = self.global46;
                            self.wasm_cpu_instructions_cpARegister(imports, var15);
                            return 4i32;
                            break;
                        }
                        let var16 = self.global47;
                        self.wasm_cpu_instructions_cpARegister(imports, var16);
                        return 4i32;
                        break;
                    }
                    let var17 = self.global48;
                    self.wasm_cpu_instructions_cpARegister(imports, var17);
                    return 4i32;
                    break;
                }
                let var18 = self.global47;
                let var19 = self.global48;
                let var20 = self.wasm_helpers_index_concatenateBytes(imports, var18, var19);
                let var21 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var20);
                self.wasm_cpu_instructions_cpARegister(imports, var21 & 255i32);
                return 8i32;
                break;
            }
            let var22 = self.global42;
            self.wasm_cpu_instructions_cpARegister(imports, var22);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_memory_load_sixteenBitLoadFromGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        'label0: loop {
            'label1: loop {
                let var3 = self.wasm_memory_readTraps_checkReadTraps(imports, var0);
                var1 = var3;
                if (var1 != -1i32) as i32 != 0 {
                    break 'label0;
                }
                let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
                var1 = var4;
                break;
            }
            break;
        }
        'label2: loop {
            'label3: loop {
                var2 = var0.wrapping_add(1i32);
                let var5 = self.wasm_memory_readTraps_checkReadTraps(imports, var2);
                var0 = var5;
                if (var0 != -1i32) as i32 != 0 {
                    break 'label2;
                }
                let var6 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var2);
                var0 = var6;
                break;
            }
            break;
        }
        let var7 = self.wasm_helpers_index_concatenateBytes(imports, var0, var1);
        var7
    }
    fn wasm_cpu_instructions_rotateRegisterLeft<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        if (var0 & 128i32 == 128i32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        let var1 = self.wasm_helpers_index_rotateByteLeft(imports, var0);
        var0 = var1;
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_rotateRegisterRight<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        if ((var0 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        let var1 = self.wasm_helpers_index_rotateByteRight(imports, var0);
        var0 = var1;
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_rotateRegisterLeftThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var2 = self.wasm_helpers_index_rotateByteLeftThroughCarry(imports, var0);
        var0 = var2;
        if var1 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_rotateRegisterRightThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var2 = self.wasm_helpers_index_rotateByteRightThroughCarry(imports, var0);
        var0 = var2;
        if var1 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_shiftLeftRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        var0 = var0.wrapping_shl(1i32 as u32) & 255i32;
        if var1 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_shiftRightArithmeticRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        if (var0 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        if (var0 & 1i32 == 1i32) as i32 != 0 {
            var2 = 1i32;
        }
        var0 = (var0 as u32).wrapping_shr(1i32 as u32) as i32;
        if var1 != 0 {
            var0 = (var0 | 128i32) & 255i32;
        }
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        if var2 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        var0
    }
    fn wasm_cpu_instructions_swapNibblesOnRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        var0 = (var0 & 15i32).wrapping_shl(4i32 as u32) | ((var0 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32;
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        var0
    }
    fn wasm_cpu_instructions_shiftRightLogicalRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        var0 = (var0 as u32).wrapping_shr(1i32 as u32) as i32;
        if var0 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        if var1 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        var0
    }
    fn wasm_cpu_instructions_testBitOnRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        if var1 & 1i32.wrapping_shl(var0 as u32) & 255i32 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        var1
    }
    fn wasm_cpu_instructions_setBitOnRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let var3: i32;
        if (var1 > 0i32) as i32 != 0 {
            var3 = (var2 | 1i32.wrapping_shl(var0 as u32) & 255i32) & 255i32;
        } else {
            var3 = var2 & (1i32.wrapping_shl(var0 as u32) & 255i32 ^ -1i32) & 255i32;
        }
        var2 = var3;
        var2
    }
    fn wasm_cpu_cbOpcodes_handleCbOpcode<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        var4 = -1i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                var5 = var0.wrapping_rem(8i32);
                                                match var5 {
                                                    0 => break 'label8,
                                                    1 => break 'label7,
                                                    2 => break 'label6,
                                                    3 => break 'label5,
                                                    4 => break 'label4,
                                                    5 => break 'label3,
                                                    6 => break 'label2,
                                                    7 => break 'label1,
                                                    _ => break 'label9,
                                                }
                                                break;
                                            }
                                            break 'label0;
                                            break;
                                        }
                                        let var6 = self.global43;
                                        var1 = var6;
                                        break 'label0;
                                        break;
                                    }
                                    let var7 = self.global44;
                                    var1 = var7;
                                    break 'label0;
                                    break;
                                }
                                let var8 = self.global45;
                                var1 = var8;
                                break 'label0;
                                break;
                            }
                            let var9 = self.global46;
                            var1 = var9;
                            break 'label0;
                            break;
                        }
                        let var10 = self.global47;
                        var1 = var10;
                        break 'label0;
                        break;
                    }
                    let var11 = self.global48;
                    var1 = var11;
                    break 'label0;
                    break;
                }
                let var12 = self.global47;
                let var13 = self.global48;
                let var14 = self.wasm_helpers_index_concatenateBytes(imports, var12, var13);
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var14);
                var1 = var15 & 255i32;
                break 'label0;
                break;
            }
            let var16 = self.global42;
            var1 = var16;
            break;
        }
        'label10: loop {
            'label11: loop {
                'label12: loop {
                    'label13: loop {
                        'label14: loop {
                            'label15: loop {
                                'label16: loop {
                                    'label17: loop {
                                        'label18: loop {
                                            'label19: loop {
                                                'label20: loop {
                                                    'label21: loop {
                                                        'label22: loop {
                                                            'label23: loop {
                                                                'label24: loop {
                                                                    'label25: loop {
                                                                        'label26: loop {
                                                                            'label27: loop {
                                                                                match (var0 & 240i32).wrapping_shr(4i32 as u32) {
                                                                                    0 => break 'label26,
                                                                                    1 => break 'label25,
                                                                                    2 => break 'label24,
                                                                                    3 => break 'label23,
                                                                                    4 => break 'label22,
                                                                                    5 => break 'label21,
                                                                                    6 => break 'label20,
                                                                                    7 => break 'label19,
                                                                                    8 => break 'label18,
                                                                                    9 => break 'label17,
                                                                                    10 => break 'label16,
                                                                                    11 => break 'label15,
                                                                                    12 => break 'label14,
                                                                                    13 => break 'label13,
                                                                                    14 => break 'label12,
                                                                                    15 => break 'label11,
                                                                                    _ => break 'label27,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label10;
                                                                            break;
                                                                        }
                                                                        if (var0 <= 7i32) as i32 != 0 {
                                                                            let var17 = self.wasm_cpu_instructions_rotateRegisterLeft(imports, var1);
                                                                            var2 = var17;
                                                                            var3 = 1i32;
                                                                        } else {
                                                                            if (var0 <= 15i32) as i32 != 0 {
                                                                                let var18 = self.wasm_cpu_instructions_rotateRegisterRight(imports, var1);
                                                                                var2 = var18;
                                                                                var3 = 1i32;
                                                                            }
                                                                        }
                                                                        break 'label10;
                                                                        break;
                                                                    }
                                                                    if (var0 <= 23i32) as i32 != 0 {
                                                                        let var19 = self.wasm_cpu_instructions_rotateRegisterLeftThroughCarry(imports, var1);
                                                                        var2 = var19;
                                                                        var3 = 1i32;
                                                                    } else {
                                                                        if (var0 <= 31i32) as i32 != 0 {
                                                                            let var20 = self.wasm_cpu_instructions_rotateRegisterRightThroughCarry(imports, var1);
                                                                            var2 = var20;
                                                                            var3 = 1i32;
                                                                        }
                                                                    }
                                                                    break 'label10;
                                                                    break;
                                                                }
                                                                if (var0 <= 39i32) as i32 != 0 {
                                                                    let var21 = self.wasm_cpu_instructions_shiftLeftRegister(imports, var1);
                                                                    var2 = var21;
                                                                    var3 = 1i32;
                                                                } else {
                                                                    if (var0 <= 47i32) as i32 != 0 {
                                                                        let var22 = self.wasm_cpu_instructions_shiftRightArithmeticRegister(imports, var1);
                                                                        var2 = var22;
                                                                        var3 = 1i32;
                                                                    }
                                                                }
                                                                break 'label10;
                                                                break;
                                                            }
                                                            if (var0 <= 55i32) as i32 != 0 {
                                                                let var23 = self.wasm_cpu_instructions_swapNibblesOnRegister(imports, var1);
                                                                var2 = var23;
                                                                var3 = 1i32;
                                                            } else {
                                                                if (var0 <= 63i32) as i32 != 0 {
                                                                    let var24 = self.wasm_cpu_instructions_shiftRightLogicalRegister(imports, var1);
                                                                    var2 = var24;
                                                                    var3 = 1i32;
                                                                }
                                                            }
                                                            break 'label10;
                                                            break;
                                                        }
                                                        if (var0 <= 71i32) as i32 != 0 {
                                                            let var25 = self.wasm_cpu_instructions_testBitOnRegister(imports, 0i32, var1);
                                                            var2 = var25;
                                                            var3 = 1i32;
                                                        } else {
                                                            if (var0 <= 79i32) as i32 != 0 {
                                                                let var26 = self.wasm_cpu_instructions_testBitOnRegister(imports, 1i32, var1);
                                                                var2 = var26;
                                                                var3 = 1i32;
                                                            }
                                                        }
                                                        break 'label10;
                                                        break;
                                                    }
                                                    if (var0 <= 87i32) as i32 != 0 {
                                                        let var27 = self.wasm_cpu_instructions_testBitOnRegister(imports, 2i32, var1);
                                                        var2 = var27;
                                                        var3 = 1i32;
                                                    } else {
                                                        if (var0 <= 95i32) as i32 != 0 {
                                                            let var28 = self.wasm_cpu_instructions_testBitOnRegister(imports, 3i32, var1);
                                                            var2 = var28;
                                                            var3 = 1i32;
                                                        }
                                                    }
                                                    break 'label10;
                                                    break;
                                                }
                                                if (var0 <= 103i32) as i32 != 0 {
                                                    let var29 = self.wasm_cpu_instructions_testBitOnRegister(imports, 4i32, var1);
                                                    var2 = var29;
                                                    var3 = 1i32;
                                                } else {
                                                    if (var0 <= 111i32) as i32 != 0 {
                                                        let var30 = self.wasm_cpu_instructions_testBitOnRegister(imports, 5i32, var1);
                                                        var2 = var30;
                                                        var3 = 1i32;
                                                    }
                                                }
                                                break 'label10;
                                                break;
                                            }
                                            if (var0 <= 119i32) as i32 != 0 {
                                                let var31 = self.wasm_cpu_instructions_testBitOnRegister(imports, 6i32, var1);
                                                var2 = var31;
                                                var3 = 1i32;
                                            } else {
                                                if (var0 <= 127i32) as i32 != 0 {
                                                    let var32 = self.wasm_cpu_instructions_testBitOnRegister(imports, 7i32, var1);
                                                    var2 = var32;
                                                    var3 = 1i32;
                                                }
                                            }
                                            break 'label10;
                                            break;
                                        }
                                        if (var0 <= 135i32) as i32 != 0 {
                                            let var33 = self.wasm_cpu_instructions_setBitOnRegister(imports, 0i32, 0i32, var1);
                                            var2 = var33;
                                            var3 = 1i32;
                                        } else {
                                            if (var0 <= 143i32) as i32 != 0 {
                                                let var34 = self.wasm_cpu_instructions_setBitOnRegister(imports, 1i32, 0i32, var1);
                                                var2 = var34;
                                                var3 = 1i32;
                                            }
                                        }
                                        break 'label10;
                                        break;
                                    }
                                    if (var0 <= 151i32) as i32 != 0 {
                                        let var35 = self.wasm_cpu_instructions_setBitOnRegister(imports, 2i32, 0i32, var1);
                                        var2 = var35;
                                        var3 = 1i32;
                                    } else {
                                        if (var0 <= 159i32) as i32 != 0 {
                                            let var36 = self.wasm_cpu_instructions_setBitOnRegister(imports, 3i32, 0i32, var1);
                                            var2 = var36;
                                            var3 = 1i32;
                                        }
                                    }
                                    break 'label10;
                                    break;
                                }
                                if (var0 <= 167i32) as i32 != 0 {
                                    let var37 = self.wasm_cpu_instructions_setBitOnRegister(imports, 4i32, 0i32, var1);
                                    var2 = var37;
                                    var3 = 1i32;
                                } else {
                                    if (var0 <= 175i32) as i32 != 0 {
                                        let var38 = self.wasm_cpu_instructions_setBitOnRegister(imports, 5i32, 0i32, var1);
                                        var2 = var38;
                                        var3 = 1i32;
                                    }
                                }
                                break 'label10;
                                break;
                            }
                            if (var0 <= 183i32) as i32 != 0 {
                                let var39 = self.wasm_cpu_instructions_setBitOnRegister(imports, 6i32, 0i32, var1);
                                var2 = var39;
                                var3 = 1i32;
                            } else {
                                if (var0 <= 191i32) as i32 != 0 {
                                    let var40 = self.wasm_cpu_instructions_setBitOnRegister(imports, 7i32, 0i32, var1);
                                    var2 = var40;
                                    var3 = 1i32;
                                }
                            }
                            break 'label10;
                            break;
                        }
                        if (var0 <= 199i32) as i32 != 0 {
                            let var41 = self.wasm_cpu_instructions_setBitOnRegister(imports, 0i32, 1i32, var1);
                            var2 = var41;
                            var3 = 1i32;
                        } else {
                            if (var0 <= 207i32) as i32 != 0 {
                                let var42 = self.wasm_cpu_instructions_setBitOnRegister(imports, 1i32, 1i32, var1);
                                var2 = var42;
                                var3 = 1i32;
                            }
                        }
                        break 'label10;
                        break;
                    }
                    if (var0 <= 215i32) as i32 != 0 {
                        let var43 = self.wasm_cpu_instructions_setBitOnRegister(imports, 2i32, 1i32, var1);
                        var2 = var43;
                        var3 = 1i32;
                    } else {
                        if (var0 <= 223i32) as i32 != 0 {
                            let var44 = self.wasm_cpu_instructions_setBitOnRegister(imports, 3i32, 1i32, var1);
                            var2 = var44;
                            var3 = 1i32;
                        }
                    }
                    break 'label10;
                    break;
                }
                if (var0 <= 231i32) as i32 != 0 {
                    let var45 = self.wasm_cpu_instructions_setBitOnRegister(imports, 4i32, 1i32, var1);
                    var2 = var45;
                    var3 = 1i32;
                } else {
                    if (var0 <= 239i32) as i32 != 0 {
                        let var46 = self.wasm_cpu_instructions_setBitOnRegister(imports, 5i32, 1i32, var1);
                        var2 = var46;
                        var3 = 1i32;
                    }
                }
                break 'label10;
                break;
            }
            if (var0 <= 247i32) as i32 != 0 {
                let var47 = self.wasm_cpu_instructions_setBitOnRegister(imports, 6i32, 1i32, var1);
                var2 = var47;
                var3 = 1i32;
            } else {
                if (var0 <= 255i32) as i32 != 0 {
                    let var48 = self.wasm_cpu_instructions_setBitOnRegister(imports, 7i32, 1i32, var1);
                    var2 = var48;
                    var3 = 1i32;
                }
            }
            break;
        }
        'label28: loop {
            'label29: loop {
                'label30: loop {
                    'label31: loop {
                        'label32: loop {
                            'label33: loop {
                                'label34: loop {
                                    'label35: loop {
                                        'label36: loop {
                                            'label37: loop {
                                                match var5 {
                                                    0 => break 'label36,
                                                    1 => break 'label35,
                                                    2 => break 'label34,
                                                    3 => break 'label33,
                                                    4 => break 'label32,
                                                    5 => break 'label31,
                                                    6 => break 'label30,
                                                    7 => break 'label29,
                                                    _ => break 'label37,
                                                }
                                                break;
                                            }
                                            break 'label28;
                                            break;
                                        }
                                        self.global43 = var2;
                                        break 'label28;
                                        break;
                                    }
                                    self.global44 = var2;
                                    break 'label28;
                                    break;
                                }
                                self.global45 = var2;
                                break 'label28;
                                break;
                            }
                            self.global46 = var2;
                            break 'label28;
                            break;
                        }
                        self.global47 = var2;
                        break 'label28;
                        break;
                    }
                    self.global48 = var2;
                    break 'label28;
                    break;
                }
                let var49 = self.global47;
                let var50 = self.global48;
                let var51 = self.wasm_helpers_index_concatenateBytes(imports, var49, var50);
                self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var51, var2);
                break 'label28;
                break;
            }
            self.global42 = var2;
            break;
        }
        let var52 = self.global50;
        self.global50 = var52.wrapping_add(1i32) & 65535i32;
        if var3 != 0 {
            var4 = 8i32;
            if (var5 == 6i32) as i32 != 0 {
                var4 = 16i32;
            }
        }
        var4
    }
    fn wasm_cpu_opcodes_handleOpcodeCx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            'label17: loop {
                                                                                match var0.wrapping_sub(192i32) {
                                                                                    0 => break 'label16,
                                                                                    1 => break 'label15,
                                                                                    2 => break 'label14,
                                                                                    3 => break 'label13,
                                                                                    4 => break 'label12,
                                                                                    5 => break 'label11,
                                                                                    6 => break 'label10,
                                                                                    7 => break 'label9,
                                                                                    8 => break 'label8,
                                                                                    9 => break 'label7,
                                                                                    10 => break 'label6,
                                                                                    11 => break 'label5,
                                                                                    12 => break 'label4,
                                                                                    13 => break 'label3,
                                                                                    14 => break 'label2,
                                                                                    15 => break 'label1,
                                                                                    _ => break 'label17,
                                                                                }
                                                                                break;
                                                                            }
                                                                            break 'label0;
                                                                            break;
                                                                        }
                                                                        let var2 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                        if var2 != 0 {
                                                                            return 8i32;
                                                                        } else {
                                                                            let var3 = self.global51;
                                                                            let var4 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var3);
                                                                            self.global50 = var4 & 65535i32;
                                                                            let var5 = self.global51;
                                                                            self.global51 = var5.wrapping_add(2i32) & 65535i32;
                                                                            return 20i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var6 = self.global51;
                                                                    let var7 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var6);
                                                                    var1 = var7;
                                                                    let var8 = self.global51;
                                                                    self.global51 = var8.wrapping_add(2i32) & 65535i32;
                                                                    let var9 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                                                    self.global43 = var9 & 255i32;
                                                                    let var10 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                                                    self.global44 = var10 & 255i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var11 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                if var11 != 0 {
                                                                    let var12 = self.global50;
                                                                    self.global50 = var12.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                } else {
                                                                    let var13 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                                    self.global50 = var13;
                                                                    return 16i32;
                                                                }
                                                                unreachable!();
                                                                break;
                                                            }
                                                            let var14 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                            self.global50 = var14;
                                                            return 16i32;
                                                            break;
                                                        }
                                                        let var15 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                        if var15 != 0 {
                                                            let var16 = self.global50;
                                                            self.global50 = var16.wrapping_add(2i32) & 65535i32;
                                                            return 12i32;
                                                        } else {
                                                            let var17 = self.global51;
                                                            self.global51 = var17.wrapping_sub(2i32) & 65535i32;
                                                            let var18 = self.global51;
                                                            let var19 = self.global50;
                                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var18, var19.wrapping_add(2i32) & 65535i32);
                                                            let var20 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                            self.global50 = var20;
                                                            return 24i32;
                                                        }
                                                        unreachable!();
                                                        break;
                                                    }
                                                    let var21 = self.global51;
                                                    self.global51 = var21.wrapping_sub(2i32) & 65535i32;
                                                    let var22 = self.global51;
                                                    let var23 = self.global43;
                                                    let var24 = self.global44;
                                                    let var25 = self.wasm_helpers_index_concatenateBytes(imports, var23, var24);
                                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var22, var25);
                                                    return 16i32;
                                                    break;
                                                }
                                                let var26 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                self.wasm_cpu_instructions_addARegister(imports, var26);
                                                let var27 = self.global50;
                                                self.global50 = var27.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var28 = self.global51;
                                            self.global51 = var28.wrapping_sub(2i32) & 65535i32;
                                            let var29 = self.global51;
                                            let var30 = self.global50;
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var29, var30);
                                            self.global50 = 0i32;
                                            return 16i32;
                                            break;
                                        }
                                        let var31 = self.wasm_cpu_flags_getZeroFlag(imports);
                                        if (var31 == 1i32) as i32 != 0 {
                                            let var32 = self.global51;
                                            let var33 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var32);
                                            self.global50 = var33 & 65535i32;
                                            let var34 = self.global51;
                                            self.global51 = var34.wrapping_add(2i32) & 65535i32;
                                            return 20i32;
                                        } else {
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var35 = self.global51;
                                    let var36 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var35);
                                    self.global50 = var36 & 65535i32;
                                    let var37 = self.global51;
                                    self.global51 = var37.wrapping_add(2i32) & 65535i32;
                                    return 16i32;
                                    break;
                                }
                                let var38 = self.wasm_cpu_flags_getZeroFlag(imports);
                                if (var38 == 1i32) as i32 != 0 {
                                    let var39 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                    self.global50 = var39;
                                    return 16i32;
                                } else {
                                    let var40 = self.global50;
                                    self.global50 = var40.wrapping_add(2i32) & 65535i32;
                                    return 12i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var41 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                            let var42 = self.wasm_cpu_cbOpcodes_handleCbOpcode(imports, var41);
                            var1 = var42;
                            if (var1 > 0i32) as i32 != 0 {
                                var1 = var1.wrapping_add(4i32);
                            }
                            return var1;
                            break;
                        }
                        let var43 = self.wasm_cpu_flags_getZeroFlag(imports);
                        if (var43 == 1i32) as i32 != 0 {
                            let var44 = self.global51;
                            self.global51 = var44.wrapping_sub(2i32) & 65535i32;
                            let var45 = self.global51;
                            let var46 = self.global50;
                            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var45, var46.wrapping_add(2i32) & 65535i32);
                            let var47 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                            self.global50 = var47;
                            return 24i32;
                        } else {
                            let var48 = self.global50;
                            self.global50 = var48.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var49 = self.global51;
                    self.global51 = var49.wrapping_sub(2i32) & 65535i32;
                    let var50 = self.global51;
                    let var51 = self.global50;
                    self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var50, var51.wrapping_add(2i32) & 65535i32);
                    let var52 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                    self.global50 = var52;
                    return 24i32;
                    break;
                }
                let var53 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var53);
                let var54 = self.global50;
                self.global50 = var54.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var55 = self.global51;
            self.global51 = var55.wrapping_sub(2i32) & 65535i32;
            let var56 = self.global51;
            let var57 = self.global50;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var56, var57);
            self.global50 = 8i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_interrupts_index_setInterrupts<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global208 = var0;
    }
    fn wasm_cpu_opcodes_handleOpcodeDx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    match var0.wrapping_sub(208i32) {
                                                                        0 => break 'label13,
                                                                        1 => break 'label12,
                                                                        2 => break 'label11,
                                                                        3 => break 'label14,
                                                                        4 => break 'label10,
                                                                        5 => break 'label9,
                                                                        6 => break 'label8,
                                                                        7 => break 'label7,
                                                                        8 => break 'label6,
                                                                        9 => break 'label5,
                                                                        10 => break 'label4,
                                                                        11 => break 'label14,
                                                                        12 => break 'label3,
                                                                        13 => break 'label14,
                                                                        14 => break 'label2,
                                                                        15 => break 'label1,
                                                                        _ => break 'label14,
                                                                    }
                                                                    break;
                                                                }
                                                                break 'label0;
                                                                break;
                                                            }
                                                            let var2 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                            if var2 != 0 {
                                                                return 8i32;
                                                            } else {
                                                                let var3 = self.global51;
                                                                let var4 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var3);
                                                                self.global50 = var4 & 65535i32;
                                                                let var5 = self.global51;
                                                                self.global51 = var5.wrapping_add(2i32) & 65535i32;
                                                                return 20i32;
                                                            }
                                                            unreachable!();
                                                            break;
                                                        }
                                                        let var6 = self.global51;
                                                        let var7 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var6);
                                                        var1 = var7;
                                                        let var8 = self.global51;
                                                        self.global51 = var8.wrapping_add(2i32) & 65535i32;
                                                        let var9 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                                        self.global45 = var9 & 255i32;
                                                        let var10 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                                        self.global46 = var10 & 255i32;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var11 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                    if var11 != 0 {
                                                        let var12 = self.global50;
                                                        self.global50 = var12.wrapping_add(2i32) & 65535i32;
                                                        return 12i32;
                                                    } else {
                                                        let var13 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                        self.global50 = var13;
                                                        return 16i32;
                                                    }
                                                    unreachable!();
                                                    break;
                                                }
                                                let var14 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                if var14 != 0 {
                                                    let var15 = self.global50;
                                                    self.global50 = var15.wrapping_add(2i32) & 65535i32;
                                                    return 12i32;
                                                } else {
                                                    let var16 = self.global51;
                                                    self.global51 = var16.wrapping_sub(2i32) & 65535i32;
                                                    let var17 = self.global51;
                                                    let var18 = self.global50;
                                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var17, var18.wrapping_add(2i32) & 65535i32);
                                                    let var19 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                                                    self.global50 = var19;
                                                    return 24i32;
                                                }
                                                unreachable!();
                                                break;
                                            }
                                            let var20 = self.global51;
                                            self.global51 = var20.wrapping_sub(2i32) & 65535i32;
                                            let var21 = self.global51;
                                            let var22 = self.global45;
                                            let var23 = self.global46;
                                            let var24 = self.wasm_helpers_index_concatenateBytes(imports, var22, var23);
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var21, var24);
                                            return 16i32;
                                            break;
                                        }
                                        let var25 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                        self.wasm_cpu_instructions_subARegister(imports, var25);
                                        let var26 = self.global50;
                                        self.global50 = var26.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var27 = self.global51;
                                    self.global51 = var27.wrapping_sub(2i32) & 65535i32;
                                    let var28 = self.global51;
                                    let var29 = self.global50;
                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var28, var29);
                                    self.global50 = 16i32;
                                    return 16i32;
                                    break;
                                }
                                let var30 = self.wasm_cpu_flags_getCarryFlag(imports);
                                if (var30 == 1i32) as i32 != 0 {
                                    let var31 = self.global51;
                                    let var32 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var31);
                                    self.global50 = var32 & 65535i32;
                                    let var33 = self.global51;
                                    self.global51 = var33.wrapping_add(2i32) & 65535i32;
                                    return 20i32;
                                } else {
                                    return 8i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var34 = self.global51;
                            let var35 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var34);
                            self.global50 = var35 & 65535i32;
                            self.wasm_interrupts_index_setInterrupts(imports, 1i32);
                            let var36 = self.global51;
                            self.global51 = var36.wrapping_add(2i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var37 = self.wasm_cpu_flags_getCarryFlag(imports);
                        if (var37 == 1i32) as i32 != 0 {
                            let var38 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                            self.global50 = var38;
                            return 16i32;
                        } else {
                            let var39 = self.global50;
                            self.global50 = var39.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var40 = self.wasm_cpu_flags_getCarryFlag(imports);
                    if (var40 == 1i32) as i32 != 0 {
                        let var41 = self.global51;
                        self.global51 = var41.wrapping_sub(2i32) & 65535i32;
                        let var42 = self.global51;
                        let var43 = self.global50;
                        self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var42, var43.wrapping_add(2i32) & 65535i32);
                        let var44 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                        self.global50 = var44;
                        return 24i32;
                    } else {
                        let var45 = self.global50;
                        self.global50 = var45.wrapping_add(2i32) & 65535i32;
                        return 12i32;
                    }
                    unreachable!();
                    break;
                }
                let var46 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var46);
                let var47 = self.global50;
                self.global50 = var47.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var48 = self.global51;
            self.global51 = var48.wrapping_sub(2i32) & 65535i32;
            let var49 = self.global51;
            let var50 = self.global50;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var49, var50);
            self.global50 = 24i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcodeEx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            match var0.wrapping_sub(224i32) {
                                                                0 => break 'label11,
                                                                1 => break 'label10,
                                                                2 => break 'label9,
                                                                3 => break 'label12,
                                                                4 => break 'label12,
                                                                5 => break 'label8,
                                                                6 => break 'label7,
                                                                7 => break 'label6,
                                                                8 => break 'label5,
                                                                9 => break 'label4,
                                                                10 => break 'label3,
                                                                11 => break 'label12,
                                                                12 => break 'label12,
                                                                13 => break 'label12,
                                                                14 => break 'label2,
                                                                15 => break 'label1,
                                                                _ => break 'label12,
                                                            }
                                                            break;
                                                        }
                                                        break 'label0;
                                                        break;
                                                    }
                                                    let var2 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                    let var3 = self.global42;
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var2.wrapping_add(65280i32), var3);
                                                    let var4 = self.global50;
                                                    self.global50 = var4.wrapping_add(1i32) & 65535i32;
                                                    return 12i32;
                                                    break;
                                                }
                                                let var5 = self.global51;
                                                let var6 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var5);
                                                var1 = var6;
                                                let var7 = self.global51;
                                                self.global51 = var7.wrapping_add(2i32) & 65535i32;
                                                let var8 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                                self.global47 = var8 & 255i32;
                                                let var9 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                                self.global48 = var9 & 255i32;
                                                return 12i32;
                                                break;
                                            }
                                            let var10 = self.global44;
                                            let var11 = self.global42;
                                            self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var10.wrapping_add(65280i32), var11);
                                            return 8i32;
                                            break;
                                        }
                                        let var12 = self.global51;
                                        self.global51 = var12.wrapping_sub(2i32) & 65535i32;
                                        let var13 = self.global51;
                                        let var14 = self.global47;
                                        let var15 = self.global48;
                                        let var16 = self.wasm_helpers_index_concatenateBytes(imports, var14, var15);
                                        self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var13, var16);
                                        return 16i32;
                                        break;
                                    }
                                    let var17 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                    self.wasm_cpu_instructions_andARegister(imports, var17);
                                    let var18 = self.global50;
                                    self.global50 = var18.wrapping_add(1i32) & 65535i32;
                                    return 8i32;
                                    break;
                                }
                                let var19 = self.global51;
                                self.global51 = var19.wrapping_sub(2i32) & 65535i32;
                                let var20 = self.global51;
                                let var21 = self.global50;
                                self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var20, var21);
                                self.global50 = 32i32;
                                return 16i32;
                                break;
                            }
                            let var22 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                            var1 = var22.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            let var23 = self.global51;
                            self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var23, var1, 1i32);
                            let var24 = self.global51;
                            self.global51 = var24.wrapping_add(var1) & 65535i32;
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                            let var25 = self.global50;
                            self.global50 = var25.wrapping_add(1i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var26 = self.global47;
                        let var27 = self.global48;
                        let var28 = self.wasm_helpers_index_concatenateBytes(imports, var26, var27);
                        self.global50 = var28 & 65535i32;
                        return 4i32;
                        break;
                    }
                    let var29 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                    let var30 = self.global42;
                    self.wasm_memory_store_eightBitStoreIntoGBMemoryWithTraps(imports, var29, var30);
                    let var31 = self.global50;
                    self.global50 = var31.wrapping_add(2i32) & 65535i32;
                    return 16i32;
                    break;
                }
                let var32 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.wasm_cpu_instructions_xorARegister(imports, var32);
                let var33 = self.global50;
                self.global50 = var33.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var34 = self.global51;
            self.global51 = var34.wrapping_sub(2i32) & 65535i32;
            let var35 = self.global51;
            let var36 = self.global50;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var35, var36);
            self.global50 = 40i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcodeFx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    match var0.wrapping_sub(240i32) {
                                                                        0 => break 'label13,
                                                                        1 => break 'label12,
                                                                        2 => break 'label11,
                                                                        3 => break 'label10,
                                                                        4 => break 'label14,
                                                                        5 => break 'label9,
                                                                        6 => break 'label8,
                                                                        7 => break 'label7,
                                                                        8 => break 'label6,
                                                                        9 => break 'label5,
                                                                        10 => break 'label4,
                                                                        11 => break 'label3,
                                                                        12 => break 'label14,
                                                                        13 => break 'label14,
                                                                        14 => break 'label2,
                                                                        15 => break 'label1,
                                                                        _ => break 'label14,
                                                                    }
                                                                    break;
                                                                }
                                                                break 'label0;
                                                                break;
                                                            }
                                                            let var1 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                                            let var2 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var1.wrapping_add(65280i32));
                                                            self.global42 = var2 & 255i32;
                                                            let var3 = self.global50;
                                                            self.global50 = var3.wrapping_add(1i32) & 65535i32;
                                                            return 12i32;
                                                            break;
                                                        }
                                                        let var4 = self.global51;
                                                        let var5 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var4);
                                                        var0 = var5 & 65535i32;
                                                        let var6 = self.global51;
                                                        self.global51 = var6.wrapping_add(2i32) & 65535i32;
                                                        let var7 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                        self.global42 = var7 & 255i32;
                                                        let var8 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                        self.global49 = var8 & 255i32;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var9 = self.global44;
                                                    let var10 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var9.wrapping_add(65280i32));
                                                    self.global42 = var10 & 255i32;
                                                    return 8i32;
                                                    break;
                                                }
                                                self.wasm_interrupts_index_setInterrupts(imports, 0i32);
                                                return 4i32;
                                                break;
                                            }
                                            let var11 = self.global51;
                                            self.global51 = var11.wrapping_sub(2i32) & 65535i32;
                                            let var12 = self.global51;
                                            let var13 = self.global42;
                                            let var14 = self.global49;
                                            let var15 = self.wasm_helpers_index_concatenateBytes(imports, var13, var14);
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var12, var15);
                                            return 16i32;
                                            break;
                                        }
                                        let var16 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                        self.wasm_cpu_instructions_orARegister(imports, var16);
                                        let var17 = self.global50;
                                        self.global50 = var17.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var18 = self.global51;
                                    self.global51 = var18.wrapping_sub(2i32) & 65535i32;
                                    let var19 = self.global51;
                                    let var20 = self.global50;
                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var19, var20);
                                    self.global50 = 48i32;
                                    return 16i32;
                                    break;
                                }
                                let var21 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                                var0 = var21.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                let var22 = self.global51;
                                self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var22, var0, 1i32);
                                let var23 = self.global51;
                                var0 = var23.wrapping_add(var0) & 65535i32;
                                let var24 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                self.global47 = var24 & 255i32;
                                let var25 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                self.global48 = var25 & 255i32;
                                let var26 = self.global50;
                                self.global50 = var26.wrapping_add(1i32) & 65535i32;
                                return 12i32;
                                break;
                            }
                            let var27 = self.global47;
                            let var28 = self.global48;
                            let var29 = self.wasm_helpers_index_concatenateBytes(imports, var27, var28);
                            self.global51 = var29 & 65535i32;
                            return 8i32;
                            break;
                        }
                        let var30 = self.wasm_cpu_opcodes_getConcatenatedDataByte(imports);
                        let var31 = self.wasm_memory_load_eightBitLoadFromGBMemoryWithTraps(imports, var30);
                        self.global42 = var31 & 255i32;
                        let var32 = self.global50;
                        self.global50 = var32.wrapping_add(2i32) & 65535i32;
                        return 16i32;
                        break;
                    }
                    self.wasm_interrupts_index_setInterrupts(imports, 1i32);
                    return 4i32;
                    break;
                }
                let var33 = self.wasm_debug_debug_cpu_getOpcodeAtProgramCounter(imports);
                self.wasm_cpu_instructions_cpARegister(imports, var33);
                let var34 = self.global50;
                self.global50 = var34.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var35 = self.global51;
            self.global51 = var35.wrapping_sub(2i32) & 65535i32;
            let var36 = self.global51;
            let var37 = self.global50;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemoryWithTraps(imports, var36, var37);
            self.global50 = 56i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_executeOpcode<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global50;
        self.global50 = var1.wrapping_add(1i32) & 65535i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                'label6: loop {
                                    'label7: loop {
                                        'label8: loop {
                                            'label9: loop {
                                                'label10: loop {
                                                    'label11: loop {
                                                        'label12: loop {
                                                            'label13: loop {
                                                                'label14: loop {
                                                                    'label15: loop {
                                                                        'label16: loop {
                                                                            match (var0 & 240i32).wrapping_shr(4i32 as u32) {
                                                                                0 => break 'label15,
                                                                                1 => break 'label14,
                                                                                2 => break 'label13,
                                                                                3 => break 'label12,
                                                                                4 => break 'label11,
                                                                                5 => break 'label10,
                                                                                6 => break 'label9,
                                                                                7 => break 'label8,
                                                                                8 => break 'label7,
                                                                                9 => break 'label6,
                                                                                10 => break 'label5,
                                                                                11 => break 'label4,
                                                                                12 => break 'label3,
                                                                                13 => break 'label2,
                                                                                14 => break 'label1,
                                                                                _ => break 'label16,
                                                                            }
                                                                            break;
                                                                        }
                                                                        break 'label0;
                                                                        break;
                                                                    }
                                                                    let var2 = self.wasm_cpu_opcodes_handleOpcode0x(imports, var0);
                                                                    return var2;
                                                                    break;
                                                                }
                                                                let var3 = self.wasm_cpu_opcodes_handleOpcode1x(imports, var0);
                                                                return var3;
                                                                break;
                                                            }
                                                            let var4 = self.wasm_cpu_opcodes_handleOpcode2x(imports, var0);
                                                            return var4;
                                                            break;
                                                        }
                                                        let var5 = self.wasm_cpu_opcodes_handleOpcode3x(imports, var0);
                                                        return var5;
                                                        break;
                                                    }
                                                    let var6 = self.wasm_cpu_opcodes_handleOpcode4x(imports, var0);
                                                    return var6;
                                                    break;
                                                }
                                                let var7 = self.wasm_cpu_opcodes_handleOpcode5x(imports, var0);
                                                return var7;
                                                break;
                                            }
                                            let var8 = self.wasm_cpu_opcodes_handleOpcode6x(imports, var0);
                                            return var8;
                                            break;
                                        }
                                        let var9 = self.wasm_cpu_opcodes_handleOpcode7x(imports, var0);
                                        return var9;
                                        break;
                                    }
                                    let var10 = self.wasm_cpu_opcodes_handleOpcode8x(imports, var0);
                                    return var10;
                                    break;
                                }
                                let var11 = self.wasm_cpu_opcodes_handleOpcode9x(imports, var0);
                                return var11;
                                break;
                            }
                            let var12 = self.wasm_cpu_opcodes_handleOpcodeAx(imports, var0);
                            return var12;
                            break;
                        }
                        let var13 = self.wasm_cpu_opcodes_handleOpcodeBx(imports, var0);
                        return var13;
                        break;
                    }
                    let var14 = self.wasm_cpu_opcodes_handleOpcodeCx(imports, var0);
                    return var14;
                    break;
                }
                let var15 = self.wasm_cpu_opcodes_handleOpcodeDx(imports, var0);
                return var15;
                break;
            }
            let var16 = self.wasm_cpu_opcodes_handleOpcodeEx(imports, var0);
            return var16;
            break;
        }
        let var17 = self.wasm_cpu_opcodes_handleOpcodeFx(imports, var0);
        var17
    }
    fn wasm_interrupts_index_Interrupts_areInterruptsPending<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global40;
        let var1 = self.global207;
        (var0 & var1 > 0i32) as i32
    }
    fn wasm_memory_store_sixteenBitStoreIntoGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.wasm_helpers_index_splitHighByte(imports, var1);
        var2 = var3;
        let var4 = self.wasm_helpers_index_splitLowByte(imports, var1);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var4);
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0.wrapping_add(1i32), var2);
    }
    fn wasm_interrupts_index__handleInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        self.wasm_interrupts_index_setInterrupts(imports, 0i32);
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65295i32);
        let var3 = self.wasm_helpers_index_resetBitOnByte(imports, var0, var2);
        var1 = var3;
        self.global40 = var1;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65295i32, var1);
        let var4 = self.global51;
        self.global51 = var4.wrapping_sub(2i32) & 65535i32;
        let var5 = self.global51;
        let var6 = self.global50;
        self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var5, var6);
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                match var0 {
                                    0 => break 'label4,
                                    1 => break 'label3,
                                    2 => break 'label2,
                                    3 => break 'label5,
                                    4 => break 'label1,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label0;
                            break;
                        }
                        self.global201 = 0i32;
                        self.global50 = 64i32;
                        break 'label0;
                        break;
                    }
                    self.global202 = 0i32;
                    self.global50 = 72i32;
                    break 'label0;
                    break;
                }
                self.global200 = 0i32;
                self.global50 = 80i32;
                break 'label0;
                break;
            }
            self.global33 = 0i32;
            self.global50 = 96i32;
            break;
        }
    }
    fn wasm_interrupts_index_checkInterrupts<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global208;
        let var2: i32;
        if var1 != 0 {
            let var3 = self.global207;
            var2 = (var3 > 0i32) as i32;
        } else {
            let var4 = self.global208;
            var2 = var4;
        }
        var0 = var2 & 1i32;
        let var5: i32;
        if var0 != 0 {
            let var6 = self.global40;
            var5 = (var6 > 0i32) as i32;
        } else {
            var5 = var0;
        }
        if var5 & 1i32 != 0 {
            var0 = 0i32;
            let var7 = self.global203;
            let var8: i32;
            if var7 != 0 {
                let var9 = self.global201;
                var8 = var9;
            } else {
                let var10 = self.global203;
                var8 = var10;
            }
            if var8 & 1i32 != 0 {
                self.wasm_interrupts_index__handleInterrupt(imports, 0i32);
                var0 = 1i32;
            } else {
                let var11 = self.global204;
                let var12: i32;
                if var11 != 0 {
                    let var13 = self.global202;
                    var12 = var13;
                } else {
                    let var14 = self.global204;
                    var12 = var14;
                }
                if var12 & 1i32 != 0 {
                    self.wasm_interrupts_index__handleInterrupt(imports, 1i32);
                    var0 = 1i32;
                } else {
                    let var15 = self.global205;
                    let var16: i32;
                    if var15 != 0 {
                        let var17 = self.global200;
                        var16 = var17;
                    } else {
                        let var18 = self.global205;
                        var16 = var18;
                    }
                    if var16 & 1i32 != 0 {
                        self.wasm_interrupts_index__handleInterrupt(imports, 2i32);
                        var0 = 1i32;
                    } else {
                        let var19 = self.global206;
                        let var20: i32;
                        if var19 != 0 {
                            let var21 = self.global33;
                            var20 = var21;
                        } else {
                            let var22 = self.global206;
                            var20 = var22;
                        }
                        if var20 & 1i32 != 0 {
                            self.wasm_interrupts_index__handleInterrupt(imports, 4i32);
                            var0 = 1i32;
                        }
                    }
                }
            }
            if var0 != 0 {
                var0 = 20i32;
                let var23 = self.global70;
                if var23 != 0 {
                    self.global70 = 0i32;
                    var0 = 24i32;
                }
                return var0;
            }
        }
        0i32
    }
    fn wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 912i32;
        }
        456i32
    }
    fn wasm_graphics_graphics_Graphics_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
        var0
    }
    fn wasm_memory_memory_getRgbPixelStart<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1.wrapping_mul(160i32).wrapping_add(var0).wrapping_mul(3i32)
    }
    fn wasm_memory_memory_setPixelOnFrame<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) {
        let var4 = self.wasm_memory_memory_getRgbPixelStart(imports, var0, var1);
        self.memory.store8(var4.wrapping_add(93184i32).wrapping_add(var2) as usize, var3 as u8);
    }
    fn wasm_graphics_priority_getPriorityforPixel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = self.wasm_graphics_priority_getPixelStart(imports, var0, var1);
        let var3 = self.memory.load8(var2.wrapping_add(69632i32) as usize) as i32;
        var3
    }
    fn wasm_graphics_backgroundWindow_drawLineOfTileFromTileCache<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) -> i32 {
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        var5 = (var1 > 0i32) as i32;
        let var10: i32;
        if var5 != 0 {
            var10 = (var0 > 8i32) as i32;
        } else {
            var10 = var5;
        }
        var5 = var10 & 1i32;
        let var11: i32;
        if var5 != 0 {
            let var12 = self.global211;
            var11 = (var6 == var12) as i32;
        } else {
            var11 = var5;
        }
        var5 = var11 & 1i32;
        let var13: i32;
        if var5 != 0 {
            let var14 = self.global212;
            var13 = (var0 == var14) as i32;
        } else {
            var13 = var5;
        }
        if var13 & 1i32 != 0 {
            var5 = 0i32;
            var6 = 0i32;
            let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4.wrapping_sub(1i32));
            let var16 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var15);
            if var16 != 0 {
                var5 = 1i32;
            }
            let var17 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4);
            let var18 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var17);
            if var18 != 0 {
                var6 = 1i32;
            }
            var3 = 0i32;
            'label0: loop {
                if (var3 < 8i32) as i32 != 0 {
                    if (var5 != var6) as i32 != 0 {
                        var3 = 7i32.wrapping_sub(var3);
                    }
                    if (var0.wrapping_add(var3) <= 160i32) as i32 != 0 {
                        var8 = var0.wrapping_sub(8i32.wrapping_sub(var3));
                        let var19 = self.wasm_memory_memory_getRgbPixelStart(imports, var0.wrapping_add(var3), var1);
                        var9 = var19.wrapping_add(93184i32);
                        var4 = 0i32;
                        'label1: loop {
                            if (var4 < 3i32) as i32 != 0 {
                                let var20 = self.memory.load8(var9.wrapping_add(var4) as usize) as i32;
                                self.wasm_memory_memory_setPixelOnFrame(imports, var0.wrapping_add(var3), var1, var4, var20);
                                var4 = var4.wrapping_add(1i32);
                                continue 'label1;
                            }
                            break;
                        }
                        let var21 = self.wasm_graphics_priority_getPriorityforPixel(imports, var8, var1);
                        var4 = var21;
                        let var22 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var4);
                        let var23 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var4);
                        self.wasm_graphics_priority_addPriorityforPixel(imports, var0.wrapping_add(var3), var1, var22, var23);
                        var7 = var7.wrapping_add(1i32);
                    }
                    var3 = var3.wrapping_add(1i32);
                    continue 'label0;
                }
                break;
            }
        } else {
            self.global211 = var6;
        }
        let var24 = self.global212;
        if (var0 >= var24) as i32 != 0 {
            self.global212 = var0.wrapping_add(8i32);
            var6 = var2.wrapping_rem(8i32);
            if (var0 < var6) as i32 != 0 {
                let var25 = self.global212;
                self.global212 = var25.wrapping_add(var6);
            }
        }
        var7
    }
    fn wasm_graphics_backgroundWindow_drawLineOfTileFromTileId<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) -> i32 {
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        var3 = var3.wrapping_rem(8i32);
        if (var0 == 0) as i32 != 0 {
            var7 = var2.wrapping_sub((var2 / 8i32).wrapping_mul(8i32));
        }
        var8 = 7i32;
        if (var0.wrapping_add(8i32) > 160i32) as i32 != 0 {
            var8 = 160i32.wrapping_sub(var0);
        }
        var2 = -1i32;
        let var10 = self.global36;
        if var10 != 0 {
            let var11 = self.wasm_memory_memory_loadFromVramBank(imports, var4, 1i32);
            var2 = var11;
            let var12 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var2 & 255i32);
            if var12 != 0 {
                var9 = 1i32;
            }
            let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var2);
            if var13 != 0 {
                var3 = 7i32.wrapping_sub(var3);
            }
        }
        let var14 = self.wasm_graphics_tiles_drawPixelsFromLineOfTile(imports, var6, var5, var9, var7, var8, var3, var0, var1, 160i32, 93184i32, 0i32, 0i32, var2);
        var14
    }
    fn wasm_graphics_backgroundWindow_drawColorPixelFromTileId<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) {
        let mut var7: i32 = 0;
        let var8 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var5, var6);
        var6 = var8;
        let var9 = self.wasm_memory_memory_loadFromVramBank(imports, var4, 1i32);
        var4 = var9;
        var3 = var3.wrapping_rem(8i32);
        let var10 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var4);
        if var10 != 0 {
            var3 = 7i32.wrapping_sub(var3);
        }
        var5 = 0i32;
        let var11 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var4);
        if var11 != 0 {
            var5 = 1i32;
        }
        let var12 = self.wasm_memory_memory_loadFromVramBank(imports, var6.wrapping_add(var3.wrapping_mul(2i32)), var5);
        var7 = var12;
        let var13 = self.wasm_memory_memory_loadFromVramBank(imports, var6.wrapping_add(var3.wrapping_mul(2i32)).wrapping_add(1i32), var5);
        var5 = var13;
        var3 = var2.wrapping_rem(8i32);
        let var14 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var4);
        if (var14 == 0) as i32 != 0 {
            var3 = 7i32.wrapping_sub(var3);
        }
        var2 = 0i32;
        let var15 = self.wasm_helpers_index_checkBitOnByte(imports, var3, var5);
        if var15 != 0 {
            var2 = 2i32;
        }
        let var16 = self.wasm_helpers_index_checkBitOnByte(imports, var3, var7);
        if var16 != 0 {
            var2 = var2.wrapping_add(1i32);
        }
        let var17 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var4 & 7i32, var2, 0i32);
        var3 = var17;
        let var18 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var3);
        var5 = var18;
        let var19 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var3);
        var6 = var19;
        let var20 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var3);
        var3 = var20;
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 0i32, var5);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 1i32, var6);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 2i32, var3);
        let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var4);
        self.wasm_graphics_priority_addPriorityforPixel(imports, var0, var1, var2, var21);
    }
    fn wasm_graphics_backgroundWindow_drawMonochromePixelFromTileId<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32) {
        let var6 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var4, var5);
        var4 = var6;
        var3 = var3.wrapping_rem(8i32);
        let var7 = self.wasm_memory_memory_loadFromVramBank(imports, var4.wrapping_add(var3.wrapping_mul(2i32)), 0i32);
        var5 = var7;
        let var8 = self.wasm_memory_memory_loadFromVramBank(imports, var4.wrapping_add(var3.wrapping_mul(2i32)).wrapping_add(1i32), 0i32);
        var4 = var8;
        var3 = 0i32;
        var2 = 7i32.wrapping_sub(var2.wrapping_rem(8i32));
        let var9 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var4);
        if var9 != 0 {
            var3 = 2i32;
        }
        let var10 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var5);
        if var10 != 0 {
            var3 = var3.wrapping_add(1i32) & 255i32;
        }
        let var11 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var3, 65351i32, 0i32);
        var2 = var11;
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 0i32, var2);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 1i32, var2);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 2i32, var2);
        self.wasm_graphics_priority_addPriorityforPixel(imports, var0, var1, var3, 0i32);
    }
    fn wasm_graphics_backgroundWindow_drawBackgroundWindowScanline<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        var11 = var3.wrapping_shr(3i32 as u32);
        'label0: loop {
            if (var4 < 160i32) as i32 != 0 {
                var6 = var4.wrapping_add(var5);
                if (var6 >= 256i32) as i32 != 0 {
                    var6 = var6.wrapping_sub(256i32);
                }
                var8 = var2.wrapping_add(var11.wrapping_mul(32i32)).wrapping_add(var6.wrapping_shr(3i32 as u32));
                let var12 = self.wasm_memory_memory_loadFromVramBank(imports, var8, 0i32);
                var7 = var12;
                var9 = 0i32;
                let var13 = self.global20;
                if var13 != 0 {
                    let var14 = self.wasm_graphics_backgroundWindow_drawLineOfTileFromTileCache(imports, var4, var0, var6, var3, var8, var1, var7);
                    var10 = var14;
                    if (var10 > 0i32) as i32 != 0 {
                        var4 = var4.wrapping_add(var10.wrapping_sub(1i32));
                        var9 = 1i32;
                    }
                }
                let var15 = self.global19;
                let var16: i32;
                if var15 != 0 {
                    var16 = (var9 == 0) as i32;
                } else {
                    let var17 = self.global19;
                    var16 = var17;
                }
                if var16 & 1i32 != 0 {
                    let var18 = self.wasm_graphics_backgroundWindow_drawLineOfTileFromTileId(imports, var4, var0, var6, var3, var8, var1, var7);
                    var10 = var18;
                    if (var10 > 0i32) as i32 != 0 {
                        var4 = var4.wrapping_add(var10.wrapping_sub(1i32));
                    }
                } else {
                    if (var9 == 0) as i32 != 0 {
                        let var19 = self.global36;
                        if var19 != 0 {
                            self.wasm_graphics_backgroundWindow_drawColorPixelFromTileId(imports, var4, var0, var6, var3, var8, var1, var7);
                        } else {
                            self.wasm_graphics_backgroundWindow_drawMonochromePixelFromTileId(imports, var4, var0, var6, var3, var1, var7);
                        }
                    }
                }
                var4 = var4.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_backgroundWindow_renderBackground<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let var4 = self.global181;
        var3 = var0.wrapping_add(var4);
        if (var3 >= 256i32) as i32 != 0 {
            var3 = var3.wrapping_sub(256i32);
        }
        let var5 = self.global180;
        self.wasm_graphics_backgroundWindow_drawBackgroundWindowScanline(imports, var0, var1, var2, var3, 0i32, var5);
    }
    fn wasm_graphics_backgroundWindow_renderWindow<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global182;
        var3 = var5;
        let var6 = self.global183;
        var4 = var6;
        if (var0 < var4) as i32 != 0 {
            return;
        }
        var3 = var3.wrapping_sub(7i32);
        self.wasm_graphics_backgroundWindow_drawBackgroundWindowScanline(imports, var0, var1, var2, var0.wrapping_sub(var4), var3, var3.wrapping_mul(-1i32));
    }
    fn wasm_graphics_sprites_renderSprites<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let mut var14: i32 = 0;
        let mut var15: i32 = 0;
        var8 = 39i32;
        'label0: loop {
            if (var8 >= 0i32) as i32 != 0 {
                var4 = var8.wrapping_mul(4i32);
                let var16 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4.wrapping_add(65024i32));
                var3 = var16;
                let var17 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4.wrapping_add(65025i32));
                var9 = var17;
                let var18 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4.wrapping_add(65026i32));
                var2 = var18;
                var3 = var3.wrapping_sub(16i32);
                var9 = var9.wrapping_sub(8i32);
                var5 = 8i32;
                if var1 != 0 {
                    var5 = 16i32;
                    if (var2.wrapping_rem(2i32) == 1i32) as i32 != 0 {
                        var2 = var2.wrapping_sub(1i32);
                    }
                }
                var7 = (var0 >= var3) as i32;
                let var19: i32;
                if var7 != 0 {
                    var19 = (var0 < var3.wrapping_add(var5)) as i32;
                } else {
                    var19 = var7;
                }
                if var19 & 1i32 != 0 {
                    let var20 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var4.wrapping_add(65027i32));
                    var7 = var20;
                    let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var7);
                    var10 = var21;
                    let var22 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var7);
                    var4 = var22;
                    let var23 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var7);
                    var13 = var23;
                    var3 = var0.wrapping_sub(var3);
                    if var4 != 0 {
                        var3 = var3.wrapping_sub(var5).wrapping_mul(-1i32).wrapping_sub(1i32);
                    }
                    let var24 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, 32768i32, var2);
                    var3 = var24.wrapping_add(var3.wrapping_mul(2i32));
                    var2 = 0i32;
                    let var25 = self.global36;
                    let var26: i32;
                    if var25 != 0 {
                        let var27 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var7);
                        var26 = var27;
                    } else {
                        let var28 = self.global36;
                        var26 = var28;
                    }
                    if var26 & 1i32 != 0 {
                        var2 = 1i32;
                    }
                    let var29 = self.wasm_memory_memory_loadFromVramBank(imports, var3, var2);
                    var14 = var29;
                    let var30 = self.wasm_memory_memory_loadFromVramBank(imports, var3.wrapping_add(1i32), var2);
                    var15 = var30;
                    var3 = 7i32;
                    'label1: loop {
                        if (var3 >= 0i32) as i32 != 0 {
                            var2 = var3;
                            if var13 != 0 {
                                var2 = var2.wrapping_sub(7i32).wrapping_mul(-1i32);
                            }
                            var4 = 0i32;
                            let var31 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var15);
                            if var31 != 0 {
                                var4 = 2i32;
                            }
                            let var32 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var14);
                            if var32 != 0 {
                                var4 = var4.wrapping_add(1i32);
                            }
                            if var4 != 0 {
                                var5 = var9.wrapping_add(7i32.wrapping_sub(var3));
                                var6 = (var5 >= 0i32) as i32;
                                let var33: i32;
                                if var6 != 0 {
                                    var33 = (var5 <= 160i32) as i32;
                                } else {
                                    var33 = var6;
                                }
                                if var33 & 1i32 != 0 {
                                    var6 = 0i32;
                                    var11 = 0i32;
                                    var12 = 0i32;
                                    let var34 = self.global36;
                                    let var35: i32;
                                    if var34 != 0 {
                                        let var36 = self.global177;
                                        var35 = (var36 == 0) as i32;
                                    } else {
                                        let var37 = self.global36;
                                        var35 = var37;
                                    }
                                    if var35 & 1i32 != 0 {
                                        var6 = 1i32;
                                    }
                                    if (var6 == 0) as i32 != 0 {
                                        let var38 = self.wasm_graphics_priority_getPriorityforPixel(imports, var5, var0);
                                        var2 = var38;
                                        let var39: i32;
                                        if var10 != 0 {
                                            var39 = (var2 & 3i32 > 0i32) as i32;
                                        } else {
                                            var39 = var10;
                                        }
                                        if var39 & 1i32 != 0 {
                                            var11 = 1i32;
                                        } else {
                                            let var40 = self.global36;
                                            let var41: i32;
                                            if var40 != 0 {
                                                let var42 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var2);
                                                var41 = var42;
                                            } else {
                                                let var43 = self.global36;
                                                var41 = var43;
                                            }
                                            if var41 & 1i32 != 0 {
                                                var12 = 1i32;
                                            }
                                        }
                                    }
                                    let var44: i32;
                                    if var6 != 0 {
                                        var44 = var6;
                                    } else {
                                        var2 = (var11 == 0) as i32;
                                        let var45: i32;
                                        if var2 != 0 {
                                            var45 = (var12 == 0) as i32;
                                        } else {
                                            var45 = var2;
                                        }
                                        var44 = var45 & 1i32;
                                    }
                                    if var44 & 1i32 != 0 {
                                        let var46 = self.global36;
                                        if var46 != 0 {
                                            let var47 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var7 & 7i32, var4, 1i32);
                                            var2 = var47;
                                            let var48 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var2);
                                            var4 = var48;
                                            let var49 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var2);
                                            var6 = var49;
                                            let var50 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var2);
                                            var2 = var50;
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 0i32, var4);
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 1i32, var6);
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 2i32, var2);
                                        } else {
                                            var2 = 65352i32;
                                            let var51 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var7);
                                            if var51 != 0 {
                                                var2 = 65353i32;
                                            }
                                            let var52 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var4, var2, 0i32);
                                            var2 = var52;
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 0i32, var2);
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 1i32, var2);
                                            self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 2i32, var2);
                                        }
                                    }
                                }
                            }
                            var3 = var3.wrapping_sub(1i32);
                            continue 'label1;
                        }
                        break;
                    }
                }
                var8 = var8.wrapping_sub(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_graphics__drawScanline<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        var2 = 34816i32;
        let var3 = self.global52;
        if var3 != 0 {
            var2 = 32768i32;
        }
        let var4 = self.global36;
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global36;
            var5 = var6;
        } else {
            let var7 = self.global177;
            var5 = var7;
        }
        if var5 & 1i32 != 0 {
            var1 = 38912i32;
            let var8 = self.global53;
            if var8 != 0 {
                var1 = 39936i32;
            }
            self.wasm_graphics_backgroundWindow_renderBackground(imports, var0, var2, var1);
        }
        let var9 = self.global174;
        if var9 != 0 {
            var1 = 38912i32;
            let var10 = self.global173;
            if var10 != 0 {
                var1 = 39936i32;
            }
            self.wasm_graphics_backgroundWindow_renderWindow(imports, var0, var2, var1);
        }
        let var11 = self.global176;
        if var11 != 0 {
            let var12 = self.global175;
            self.wasm_graphics_sprites_renderSprites(imports, var0, var12);
        }
    }
    fn wasm_graphics_graphics__renderEntireFrame<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        'label0: loop {
            if (var0 as u32 <= 144i32 as u32) as i32 != 0 {
                self.wasm_graphics_graphics__drawScanline(imports, var0);
                var0 = var0.wrapping_add(1i32) & 255i32;
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_priority_clearPriorityMap<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        'label0: loop {
            if (var0 < 144i32) as i32 != 0 {
                var1 = 0i32;
                'label1: loop {
                    if (var1 < 160i32) as i32 != 0 {
                        let var2 = self.wasm_graphics_priority_getPixelStart(imports, var1, var0);
                        self.memory.store8(var2.wrapping_add(69632i32) as usize, 0i32 as u8);
                        var1 = var1.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                var0 = var0.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_tiles_resetTileCache<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global211 = -1i32;
        self.global212 = -1i32;
    }
    fn wasm_graphics_graphics_Graphics_MIN_CYCLES_SPRITES_LCD_MODE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 752i32;
        }
        376i32
    }
    fn wasm_graphics_graphics_Graphics_MIN_CYCLES_TRANSFER_DATA_LCD_MODE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global68;
        if var0 != 0 {
            return 498i32;
        }
        249i32
    }
    fn wasm_interrupts_index_requestLcdInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global202 = 1i32;
        self.wasm_interrupts_index__requestInterrupt(imports, 1i32);
    }
    fn wasm_memory_dma_updateHblankHdma<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global185;
        if (var1 == 0) as i32 != 0 {
            return;
        }
        let var2 = self.global190;
        var0 = 16i32;
        if (var2 < var0) as i32 != 0 {
            let var3 = self.global190;
            var0 = var3;
        }
        let var4 = self.global191;
        let var5 = self.global192;
        self.wasm_memory_dma_hdmaTransfer(imports, var4, var5, var0);
        let var6 = self.global191;
        self.global191 = var6.wrapping_add(var0);
        let var7 = self.global192;
        self.global192 = var7.wrapping_add(var0);
        let var8 = self.global190;
        self.global190 = var8.wrapping_sub(var0);
        let var9 = self.global190;
        if (var9 <= 0i32) as i32 != 0 {
            self.global185 = 0i32;
            let var10 = self.global184;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var10, 255i32);
        } else {
            let var11 = self.global184;
            let var12 = self.global190;
            let var13 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, (var12 / 16i32).wrapping_sub(1i32));
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var11, var13);
        }
    }
    fn wasm_interrupts_index_requestVBlankInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global201 = 1i32;
        self.wasm_interrupts_index__requestInterrupt(imports, 0i32);
    }
    fn wasm_graphics_lcd_setLcdStatus<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global172;
        if (var5 == 0) as i32 != 0 {
            self.global210 = 0i32;
            self.global59 = 0i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65348i32, 0i32);
            let var6 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65345i32);
            let var7 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var6);
            let var8 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var7);
            var3 = var8;
            self.global73 = 0i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65345i32, var3);
            return;
        }
        let var9 = self.global73;
        var2 = var9;
        let var10 = self.global59;
        var3 = var10;
        if (var3 >= 144i32) as i32 != 0 {
            var1 = 1i32;
        } else {
            let var11 = self.global210;
            let var12 = self.wasm_graphics_graphics_Graphics_MIN_CYCLES_SPRITES_LCD_MODE(imports);
            if (var11 >= var12) as i32 != 0 {
                var1 = 2i32;
            } else {
                let var13 = self.global210;
                let var14 = self.wasm_graphics_graphics_Graphics_MIN_CYCLES_TRANSFER_DATA_LCD_MODE(imports);
                if (var13 >= var14) as i32 != 0 {
                    var1 = 3i32;
                }
            }
        }
        if (var2 != var1) as i32 != 0 {
            let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65345i32);
            var0 = var15;
            self.global73 = var1;
            var2 = 0i32;
            'label0: loop {
                'label1: loop {
                    'label2: loop {
                        'label3: loop {
                            'label4: loop {
                                'label5: loop {
                                    match var1 {
                                        0 => break 'label4,
                                        1 => break 'label3,
                                        2 => break 'label2,
                                        3 => break 'label1,
                                        _ => break 'label5,
                                    }
                                    break;
                                }
                                break 'label0;
                                break;
                            }
                            let var16 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                            let var17 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var16);
                            var0 = var17;
                            let var18 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
                            var2 = var18;
                            break 'label0;
                            break;
                        }
                        let var19 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var0);
                        let var20 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var19);
                        var0 = var20;
                        let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
                        var2 = var21;
                        break 'label0;
                        break;
                    }
                    let var22 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                    let var23 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var22);
                    var0 = var23;
                    let var24 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var0);
                    var2 = var24;
                    break 'label0;
                    break;
                }
                let var25 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var0);
                let var26 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var25);
                var0 = var26;
                break;
            }
            if var2 != 0 {
                self.wasm_interrupts_index_requestLcdInterrupt(imports);
            }
            if (var1 == 0) as i32 != 0 {
                self.wasm_memory_dma_updateHblankHdma(imports);
            }
            if (var1 == 1i32) as i32 != 0 {
                self.wasm_interrupts_index_requestVBlankInterrupt(imports);
            }
            let var27 = self.global178;
            var2 = var27;
            var4 = (var1 == 0) as i32;
            let var28: i32;
            if var4 != 0 {
                var28 = var4;
            } else {
                var28 = (var1 == 1i32) as i32;
            }
            var4 = var28 & 1i32;
            let var29: i32;
            if var4 != 0 {
                var29 = (var3 == var2) as i32;
            } else {
                var29 = var4;
            }
            if var29 & 1i32 != 0 {
                let var30 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var0);
                var0 = var30;
                let var31 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var0);
                if var31 != 0 {
                    self.wasm_interrupts_index_requestLcdInterrupt(imports);
                }
            } else {
                let var32 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var0);
                var0 = var32;
            }
            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65345i32, var0);
        }
    }
    fn wasm_graphics_graphics_updateGraphics<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global172;
        if var2 != 0 {
            let var3 = self.global210;
            self.global210 = var3.wrapping_add(var0);
            let var4 = self.global210;
            let var5 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
            if (var4 >= var5) as i32 != 0 {
                let var6 = self.global210;
                let var7 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
                self.global210 = var6.wrapping_sub(var7);
                let var8 = self.global59;
                var1 = var8;
                if (var1 == 144i32) as i32 != 0 {
                    let var9 = self.global17;
                    if var9 != 0 {
                        self.wasm_graphics_graphics__renderEntireFrame(imports);
                    } else {
                        self.wasm_graphics_graphics__drawScanline(imports, var1);
                    }
                    self.wasm_graphics_priority_clearPriorityMap(imports);
                    self.wasm_graphics_tiles_resetTileCache(imports);
                } else {
                    if (var1 < 144i32) as i32 != 0 {
                        let var10 = self.global17;
                        if (var10 == 0) as i32 != 0 {
                            self.wasm_graphics_graphics__drawScanline(imports, var1);
                        }
                    }
                }
                let var11: i32;
                if (var1 > 153i32) as i32 != 0 {
                    var11 = 0i32;
                } else {
                    var11 = var1.wrapping_add(1i32);
                }
                var1 = var11;
                self.global59 = var1;
            }
        }
        self.wasm_graphics_lcd_setLcdStatus(imports);
    }
    fn wasm_graphics_graphics_batchProcessGraphics<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global209;
        let var1 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
        if (var0 < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global209;
            let var3 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
                self.wasm_graphics_graphics_updateGraphics(imports, var4);
                let var5 = self.global209;
                let var6 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
                self.global209 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_index_emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        self.global54 = 1i32;
        var0 = 4i32;
        let var2 = self.global70;
        var1 = (var2 == 0) as i32;
        let var3: i32;
        if var1 != 0 {
            let var4 = self.global21;
            var3 = (var4 == 0) as i32;
        } else {
            var3 = var1;
        }
        if var3 & 1i32 != 0 {
            let var5 = self.global50;
            let var6 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var5);
            let var7 = self.wasm_cpu_opcodes_executeOpcode(imports, var6 & 255i32);
            var0 = var7;
        } else {
            let var8 = self.global70;
            let var9: i32;
            if var8 != 0 {
                let var10 = self.global208;
                var9 = (var10 == 0) as i32;
            } else {
                let var11 = self.global70;
                var9 = var11;
            }
            var1 = var9 & 1i32;
            let var12: i32;
            if var1 != 0 {
                let var13 = self.wasm_interrupts_index_Interrupts_areInterruptsPending(imports);
                var12 = var13;
            } else {
                var12 = var1;
            }
            if var12 & 1i32 != 0 {
                self.global70 = 0i32;
                self.global21 = 0i32;
                let var14 = self.global50;
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var14);
                let var16 = self.wasm_cpu_opcodes_executeOpcode(imports, var15 & 255i32);
                var0 = var16;
                let var17 = self.global50;
                self.global50 = var17.wrapping_sub(1i32) & 65535i32;
            }
        }
        let var18 = self.global49;
        self.global49 = var18 & 240i32;
        if (var0 <= 0i32) as i32 != 0 {
            return var0;
        }
        let var19 = self.global179;
        if (var19 > 0i32) as i32 != 0 {
            let var20 = self.global179;
            var0 = var0.wrapping_add(var20);
            self.global179 = 0i32;
        }
        let var21 = self.wasm_interrupts_index_checkInterrupts(imports);
        var0 = var0.wrapping_add(var21);
        let var22 = self.global69;
        self.global69 = var22.wrapping_add(var0);
        let var23 = self.global21;
        if (var23 == 0) as i32 != 0 {
            let var24 = self.global15;
            if var24 != 0 {
                let var25 = self.global209;
                self.global209 = var25.wrapping_add(var0);
                self.wasm_graphics_graphics_batchProcessGraphics(imports);
            } else {
                self.wasm_graphics_graphics_updateGraphics(imports, var0);
            }
            let var26 = self.global14;
            if var26 != 0 {
                let var27 = self.global74;
                self.global74 = var27.wrapping_add(var0);
            } else {
                self.wasm_sound_sound_updateSound(imports, var0);
            }
        }
        let var28 = self.global16;
        if var28 != 0 {
            let var29 = self.global197;
            self.global197 = var29.wrapping_add(var0);
            self.wasm_timers_index_batchProcessTimers(imports);
        } else {
            self.wasm_timers_index_updateTimers(imports, var0);
        }
        var0
    }
    fn wasm_index_update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        'label0: loop {
            var1 = (var0 == 0) as i32;
            let var2: i32;
            if var1 != 0 {
                let var3 = self.global69;
                let var4 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
                var2 = (var3 < var4) as i32;
            } else {
                var2 = var1;
            }
            if var2 & 1i32 != 0 {
                let var5 = self.wasm_index_emulationStep(imports);
                if (var5 < 0i32) as i32 != 0 {
                    var0 = 1i32;
                }
                continue 'label0;
            }
            break;
        }
        let var6 = self.global69;
        let var7 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
        if (var6 >= var7) as i32 != 0 {
            let var8 = self.global69;
            let var9 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
            self.global69 = var8.wrapping_sub(var9);
            return 1i32;
        }
        let var10 = self.global50;
        self.global50 = var10.wrapping_sub(1i32) & 65535i32;
        -1i32
    }
    fn wasm_memory_memory_getSaveStateMemoryOffset<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var0.wrapping_add(1024i32).wrapping_add(var1.wrapping_mul(50i32))
    }
    fn wasm_memory_store_storeBooleanDirectlyToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        if var1 != 0 {
            self.memory.store8(var0 as usize, 1i32 as u8);
        } else {
            self.memory.store8(var0 as usize, 0i32 as u8);
        }
    }
    fn wasm_cpu_cpu_Cpu_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 0i32);
        let var1 = self.global42;
        self.memory.store8(var0 as usize, var1 as u8);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 0i32);
        let var3 = self.global43;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 0i32);
        let var5 = self.global44;
        self.memory.store8(var4 as usize, var5 as u8);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 3i32, 0i32);
        let var7 = self.global45;
        self.memory.store8(var6 as usize, var7 as u8);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 0i32);
        let var9 = self.global46;
        self.memory.store8(var8 as usize, var9 as u8);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 0i32);
        let var11 = self.global47;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 0i32);
        let var13 = self.global48;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 0i32);
        let var15 = self.global49;
        self.memory.store8(var14 as usize, var15 as u8);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 0i32);
        let var17 = self.global51;
        self.memory.store16(var16 as usize, var17 as u16);
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 0i32);
        let var19 = self.global50;
        self.memory.store16(var18 as usize, var19 as u16);
        let var20 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 12i32, 0i32);
        let var21 = self.global69;
        self.memory.store32(var20 as usize, var21 as u32);
        let var22 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 17i32, 0i32);
        let var23 = self.global70;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var22, var23);
        let var24 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 18i32, 0i32);
        let var25 = self.global21;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var24, var25);
    }
    fn wasm_graphics_graphics_Graphics_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 1i32);
        let var1 = self.global210;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 1i32);
        let var3 = self.global73;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.global59;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65348i32, var4);
    }
    fn wasm_interrupts_index_Interrupts_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 2i32);
        let var1 = self.global208;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 2i32);
        let var3 = self.global213;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var2, var3);
    }
    fn wasm_joypad_index_Joypad_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    fn wasm_memory_memory_Memory_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 4i32);
        let var1 = self.global34;
        self.memory.store16(var0 as usize, var1 as u16);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 4i32);
        let var3 = self.global38;
        self.memory.store16(var2 as usize, var3 as u16);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 4i32);
        let var5 = self.global71;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var4, var5);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 4i32);
        let var7 = self.global72;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var6, var7);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 4i32);
        let var9 = self.global55;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var8, var9);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 4i32);
        let var11 = self.global56;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var10, var11);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 4i32);
        let var13 = self.global57;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var12, var13);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 4i32);
        let var15 = self.global58;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var14, var15);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 4i32);
        let var17 = self.global35;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var16, var17);
    }
    fn wasm_timers_index_Timers_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 5i32);
        let var1 = self.global66;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 5i32);
        let var3 = self.global67;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 5i32);
        let var5 = self.global198;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.global63;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65284i32, var6);
        let var7 = self.global193;
        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65285i32, var7);
    }
    fn wasm_sound_sound_Sound_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 6i32);
        let var1 = self.global75;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 6i32);
        let var3 = self.global143;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 6i32);
        let var5 = self.global76;
        self.memory.store8(var4 as usize, var5 as u8);
    }
    fn wasm_sound_channel1_Channel1_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 7i32);
        let var1 = self.global79;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 7i32);
        let var3 = self.global111;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 7i32);
        let var5 = self.global98;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 7i32);
        let var7 = self.global77;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 7i32);
        let var9 = self.global101;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 7i32);
        let var11 = self.global214;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 7i32);
        let var13 = self.global127;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 25i32, 7i32);
        let var15 = self.global91;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var14, var15);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 26i32, 7i32);
        let var17 = self.global89;
        self.memory.store32(var16 as usize, var17 as u32);
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 31i32, 7i32);
        let var19 = self.global92;
        self.memory.store16(var18 as usize, var19 as u16);
    }
    fn wasm_sound_channel2_Channel2_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 8i32);
        let var1 = self.global82;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 8i32);
        let var3 = self.global121;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 8i32);
        let var5 = self.global102;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 8i32);
        let var7 = self.global80;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 8i32);
        let var9 = self.global105;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 8i32);
        let var11 = self.global215;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 8i32);
        let var13 = self.global131;
        self.memory.store8(var12 as usize, var13 as u8);
    }
    fn wasm_sound_channel3_Channel3_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 9i32);
        let var1 = self.global85;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 9i32);
        let var3 = self.global123;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 9i32);
        let var5 = self.global83;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 9i32);
        let var7 = self.global135;
        self.memory.store16(var6 as usize, var7 as u16);
    }
    fn wasm_sound_channel4_Channel4_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 10i32);
        let var1 = self.global88;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 10i32);
        let var3 = self.global125;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 10i32);
        let var5 = self.global106;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 10i32);
        let var7 = self.global86;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 10i32);
        let var9 = self.global109;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 10i32);
        let var11 = self.global140;
        self.memory.store16(var10 as usize, var11 as u16);
    }
    fn wasm_index_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_cpu_cpu_Cpu_saveState(imports);
        self.wasm_graphics_graphics_Graphics_saveState(imports);
        self.wasm_interrupts_index_Interrupts_saveState(imports);
        self.wasm_joypad_index_Joypad_saveState(imports);
        self.wasm_memory_memory_Memory_saveState(imports);
        self.wasm_timers_index_Timers_saveState(imports);
        self.wasm_sound_sound_Sound_saveState(imports);
        self.wasm_sound_channel1_Channel1_saveState(imports);
        self.wasm_sound_channel2_Channel2_saveState(imports);
        self.wasm_sound_channel3_Channel3_saveState(imports);
        self.wasm_sound_channel4_Channel4_saveState(imports);
    }
    fn wasm_memory_load_loadBooleanDirectlyFromWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.memory.load8(var0 as usize) as i32;
        if (var1 > 0i32) as i32 != 0 {
            return 1i32;
        }
        0i32
    }
    fn wasm_cpu_cpu_Cpu_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 0i32);
        let var1 = self.memory.load8(var0 as usize) as i32;
        self.global42 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 0i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global43 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 0i32);
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global44 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 3i32, 0i32);
        let var7 = self.memory.load8(var6 as usize) as i32;
        self.global45 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 0i32);
        let var9 = self.memory.load8(var8 as usize) as i32;
        self.global46 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 0i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global47 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 0i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global48 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 0i32);
        let var15 = self.memory.load8(var14 as usize) as i32;
        self.global49 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 0i32);
        let var17 = self.memory.load16(var16 as usize) as i32;
        self.global51 = var17;
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 0i32);
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global50 = var19;
        let var20 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 12i32, 0i32);
        let var21 = self.memory.load32(var20 as usize) as i32;
        self.global69 = var21;
        let var22 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 17i32, 0i32);
        let var23 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var22);
        self.global70 = var23;
        let var24 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 18i32, 0i32);
        let var25 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var24);
        self.global21 = var25;
    }
    fn wasm_graphics_graphics_Graphics_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 1i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global210 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 1i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global73 = var3;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65348i32);
        self.global59 = var4;
        let var5 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65344i32);
        self.wasm_graphics_lcd_Lcd_updateLcdControl(imports, var5);
    }
    fn wasm_interrupts_index_Interrupts_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 2i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global208 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 2i32);
        let var3 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var2);
        self.global213 = var3;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65535i32);
        self.wasm_interrupts_index_Interrupts_updateInterruptEnabled(imports, var4);
        let var5 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65295i32);
        self.wasm_interrupts_index_Interrupts_updateInterruptRequested(imports, var5);
    }
    fn wasm_joypad_index_Joypad_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65280i32);
        self.wasm_joypad_index_Joypad_updateJoypad(imports, var0);
    }
    fn wasm_memory_memory_Memory_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 4i32);
        let var1 = self.memory.load16(var0 as usize) as i32;
        self.global34 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 4i32);
        let var3 = self.memory.load16(var2 as usize) as i32;
        self.global38 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 4i32);
        let var5 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var4);
        self.global71 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 4i32);
        let var7 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var6);
        self.global72 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 4i32);
        let var9 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var8);
        self.global55 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 4i32);
        let var11 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var10);
        self.global56 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 4i32);
        let var13 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var12);
        self.global57 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 4i32);
        let var15 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var14);
        self.global58 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 4i32);
        let var17 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var16);
        self.global35 = var17;
    }
    fn wasm_timers_index_Timers_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 5i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global66 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 5i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global67 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 5i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global198 = var5;
        let var6 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65284i32);
        self.global63 = var6;
        let var7 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65285i32);
        self.wasm_timers_index_Timers_updateTimerCounter(imports, var7);
        let var8 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65286i32);
        self.wasm_timers_index_Timers_updateTimerModulo(imports, var8);
        let var9 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65287i32);
        self.wasm_timers_index_Timers_updateTimerControl(imports, var9);
    }
    fn wasm_sound_sound_Sound_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 6i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global75 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 6i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global143 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 6i32);
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global76 = var5;
        self.wasm_sound_sound_resetAudioQueue(imports);
    }
    fn wasm_sound_channel1_Channel1_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 7i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global79 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 7i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global111 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 7i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global98 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 7i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global77 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 7i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global101 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 7i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global214 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 7i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global127 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 25i32, 7i32);
        let var15 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var14);
        self.global91 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 26i32, 7i32);
        let var17 = self.memory.load32(var16 as usize) as i32;
        self.global89 = var17;
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 31i32, 7i32);
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global92 = var19;
    }
    fn wasm_sound_channel2_Channel2_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 8i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global82 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 8i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global121 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 8i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global102 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 8i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global80 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 8i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global105 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 8i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global215 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 8i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global131 = var13;
    }
    fn wasm_sound_channel3_Channel3_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 9i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global85 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 9i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global123 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 9i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global83 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 9i32);
        let var7 = self.memory.load16(var6 as usize) as i32;
        self.global135 = var7;
    }
    fn wasm_sound_channel4_Channel4_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 10i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global88 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 10i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global125 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 10i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global106 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 10i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global86 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 10i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global109 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 10i32);
        let var11 = self.memory.load16(var10 as usize) as i32;
        self.global140 = var11;
    }
    fn wasm_index_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_cpu_cpu_Cpu_loadState(imports);
        self.wasm_graphics_graphics_Graphics_loadState(imports);
        self.wasm_interrupts_index_Interrupts_loadState(imports);
        self.wasm_joypad_index_Joypad_loadState(imports);
        self.wasm_memory_memory_Memory_loadState(imports);
        self.wasm_timers_index_Timers_loadState(imports);
        self.wasm_sound_sound_Sound_loadState(imports);
        self.wasm_sound_channel1_Channel1_loadState(imports);
        self.wasm_sound_channel2_Channel2_loadState(imports);
        self.wasm_sound_channel3_Channel3_loadState(imports);
        self.wasm_sound_channel4_Channel4_loadState(imports);
        self.global54 = 0i32;
    }
    fn start<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.memory.size();
        if (var0 < 139i32) as i32 != 0 {
            let var1 = self.memory.size();
            let var2 = self.memory.grow(139i32.wrapping_sub(var1) as usize);
            var2;
        }
    }
}

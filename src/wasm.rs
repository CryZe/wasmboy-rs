#![allow(unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case, non_upper_case_globals, unused_parens, unconditional_recursion)]

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
}

pub mod consts {
    pub const wasmMemorySize: i32 = 9109504;
    pub const wasmBoyInternalStateLocation: i32 = 1024;
    pub const wasmBoyInternalStateSize: i32 = 1024;
    pub const gameBoyInternalMemoryLocation: i32 = 2048;
    pub const gameBoyInternalMemorySize: i32 = 65535;
    pub const videoOutputLocation: i32 = 67584;
    pub const gameboyColorPaletteLocation: i32 = 67584;
    pub const frameInProgressVideoOutputLocation: i32 = 93184;
    pub const currentFrameVideoOutputLocation: i32 = 162816;
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
                global17: 65359,
                global18: 0,
                global19: 65392,
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
                global30: 1,
                global31: 0,
                global32: 0,
                global33: 0,
                global34: 0,
                global35: 0,
                global36: 0,
                global37: 0,
                global38: 0,
                global39: 0,
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
                global55: 0,
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
                global67: 0,
                global68: 0,
                global69: 0,
                global70: 0,
                global71: 0,
                global72: 0,
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
                global83: 15,
                global84: 0,
                global85: 15,
                global86: 0,
                global87: 15,
                global88: 0,
                global89: 0,
                global90: 15,
                global91: 0,
                global92: 127,
                global93: 127,
                global94: 0,
                global95: 48000,
                global96: 0,
                global97: 131072,
                global98: 0,
                global99: 1,
                global100: 0,
                global101: 256,
                global102: 0,
                global103: 0,
                global104: 0,
                global105: 0,
                global106: 65365,
                global107: 0,
                global108: 0,
                global109: 0,
                global110: 0,
                global111: 0,
                global112: 65361,
                global113: 65362,
                global114: 65363,
                global115: 65364,
                global116: 65384,
                global117: 65387,
                global118: 65385,
                global119: 0,
                global120: 0,
                global121: 0,
                global122: 0,
                global123: 0,
                global124: 0,
            },
        };
        instance.context.start(&mut instance.imports);
        instance
    }
    pub fn initialize(&mut self, var0: i32, var1: i32) {
        self.context.wasm_cpu_cpu_initialize(&mut self.imports, var0, var1)
    }
    pub fn config(&mut self, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32) {
        self.context.wasm_config_config(&mut self.imports, var0, var1, var2, var3, var4)
    }
    pub fn update(&mut self) -> i32 {
        self.context.wasm_cpu_opcodes_update(&mut self.imports)
    }
    pub fn emulationStep(&mut self, var0: i32, var1: i32, var2: i32) -> i32 {
        self.context.wasm_cpu_opcodes_emulationStep(&mut self.imports, var0, var1, var2)
    }
    pub fn areInterruptsEnabled(&mut self) -> i32 {
        self.context.wasm_interrupts_index_areInterruptsEnabled(&mut self.imports)
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
    pub fn saveState(&mut self) {
        self.context.wasm_index_saveState(&mut self.imports)
    }
    pub fn loadState(&mut self) {
        self.context.wasm_index_loadState(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32) {
        self.wasm_cpu_cpu_initialize(imports, var0, var1)
    }
    pub fn config<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32) {
        self.wasm_config_config(imports, var0, var1, var2, var3, var4)
    }
    pub fn update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_cpu_opcodes_update(imports)
    }
    pub fn emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32) -> i32 {
        self.wasm_cpu_opcodes_emulationStep(imports, var0, var1, var2)
    }
    pub fn areInterruptsEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.wasm_interrupts_index_areInterruptsEnabled(imports)
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
    pub fn saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_index_saveState(imports)
    }
    pub fn loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_index_loadState(imports)
    }
    fn wasm_memory_banking_getRomBankAddress<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global14;
        var1 = var3;
        let var4 = self.global15;
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
        let var1 = self.global18;
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
                        let var3 = self.global16;
                        if var3 != 0 {
                            let var4 = self.global17;
                            let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var4);
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
            let var7 = self.global16;
            if var7 != 0 {
                let var8 = self.global19;
                let var9 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var8);
                var1 = var9 & 7i32;
            }
            if ((var1 as u32) < 1i32 as u32) as i32 != 0 {
                var1 = 1i32;
            }
            return var0.wrapping_add(var1.wrapping_mul(4096i32)).wrapping_add(-34816i32);
            break;
        }
        var0.wrapping_add(-6144i32)
    }
    fn wasm_memory_load__eightBitLoadFromWasmBoyMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(imports, var0);
        let var2 = self.memory.load8(var1 as usize) as i32;
        var2
    }
    fn wasm_memory_load_eightBitLoadFromGBMemorySkipTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_memory_load__eightBitLoadFromWasmBoyMemory(imports, var0);
        var1
    }
    fn wasm_helpers_index_log<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) {
        imports.log(self, var0, var1, var2, var3, var4, var5, var6);
    }
    fn wasm_memory_store__eightBitStoreIntoWasmBoyMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.wasm_memory_memoryMap_getWasmBoyOffsetFromGameBoyOffset(imports, var0);
        self.memory.store8(var2 as usize, var1 as u8);
    }
    fn wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0, var1);
    }
    fn wasm_memory_memory_initializeCartridge<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 327i32);
        var0 = var2;
        self.global30 = 0i32;
        self.global31 = 0i32;
        self.global32 = 0i32;
        self.global33 = 0i32;
        self.global15 = 0i32;
        if var0 != 0 {
            var1 = (var0 as u32 >= 1i32 as u32) as i32;
            let var3: i32;
            if var1 != 0 {
                var3 = (var0 as u32 <= 3i32 as u32) as i32;
            } else {
                var3 = var1;
            }
            if var3 & 1i32 != 0 {
                self.global31 = 1i32;
            } else {
                var1 = (var0 as u32 >= 5i32 as u32) as i32;
                let var4: i32;
                if var1 != 0 {
                    var4 = (var0 as u32 <= 6i32 as u32) as i32;
                } else {
                    var4 = var1;
                }
                if var4 & 1i32 != 0 {
                    self.global32 = 1i32;
                } else {
                    var1 = (var0 as u32 >= 15i32 as u32) as i32;
                    let var5: i32;
                    if var1 != 0 {
                        var5 = (var0 as u32 <= 19i32 as u32) as i32;
                    } else {
                        var5 = var1;
                    }
                    if var5 & 1i32 != 0 {
                        self.global33 = 1i32;
                    } else {
                        var1 = (var0 as u32 >= 25i32 as u32) as i32;
                        let var6: i32;
                        if var1 != 0 {
                            var6 = (var0 as u32 <= 30i32 as u32) as i32;
                        } else {
                            var6 = var1;
                        }
                        if var6 & 1i32 != 0 {
                            self.global15 = 1i32;
                        }
                    }
                }
            }
        } else {
            self.global30 = 1i32;
        }
        self.global14 = 1i32;
        self.global18 = 0i32;
    }
    fn wasm_sound_channel1_Channel1_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65296i32, 128i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65297i32, 191i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65298i32, 243i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65299i32, 193i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65300i32, 191i32);
    }
    fn wasm_sound_channel2_Channel2_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65301i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65302i32, 63i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65303i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65304i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65305i32, 184i32);
    }
    fn wasm_sound_channel3_Channel3_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65306i32, 127i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65307i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65308i32, 159i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65309i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65310i32, 184i32);
        self.global34 = 1i32;
    }
    fn wasm_sound_channel4_Channel4_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65311i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65312i32, 255i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65313i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65314i32, 0i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65315i32, 191i32);
    }
    fn wasm_sound_sound_initializeSound<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_sound_channel1_Channel1_initialize(imports);
        self.wasm_sound_channel2_Channel2_initialize(imports);
        self.wasm_sound_channel3_Channel3_initialize(imports);
        self.wasm_sound_channel4_Channel4_initialize(imports);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65316i32, 119i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65317i32, 243i32);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65318i32, 241i32);
        self.global35 = 1i32;
        self.global36 = 1i32;
    }
    fn wasm_cpu_cpu_initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 323i32);
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
            self.global16 = 1i32;
        }
        self.wasm_helpers_index_log(imports, 4i32, 1i32, var1, -9999i32, -9999i32, -9999i32, -9999i32);
        if (var1 <= 0i32) as i32 != 0 {
            let var7 = self.global16;
            if var7 != 0 {
                self.global20 = 17i32;
                self.global21 = 128i32;
                self.global22 = 0i32;
                self.global23 = 0i32;
                self.global24 = 255i32;
                self.global25 = 86i32;
                self.global26 = 0i32;
                self.global27 = 13i32;
                self.global28 = 256i32;
                self.global29 = 65534i32;
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65344i32, 145i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65345i32, 129i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65348i32, 144i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65351i32, 252i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65392i32, 248i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65359i32, 254i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65357i32, 126i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65280i32, 207i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65282i32, 124i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65284i32, 47i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65287i32, 248i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65295i32, 225i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65384i32, 192i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65385i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65386i32, 193i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65387i32, 13i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65359i32, 0i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65392i32, 1i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65361i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65362i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65363i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65364i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65365i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65388i32, 254i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65397i32, 143i32);
            } else {
                self.global20 = 1i32;
                self.global21 = 176i32;
                self.global22 = 0i32;
                self.global23 = 19i32;
                self.global24 = 0i32;
                self.global25 = 216i32;
                self.global26 = 1i32;
                self.global27 = 77i32;
                self.global28 = 256i32;
                self.global29 = 65534i32;
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65344i32, 145i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65345i32, 133i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65350i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65351i32, 252i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65352i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65353i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65392i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65359i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65357i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65280i32, 207i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65282i32, 126i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65284i32, 171i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65287i32, 248i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65295i32, 225i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65384i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65385i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65386i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65387i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65359i32, 0i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65392i32, 1i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65361i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65362i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65363i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65364i32, 255i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65365i32, 255i32);
            }
            self.wasm_memory_memory_initializeCartridge(imports);
            self.wasm_sound_sound_initializeSound(imports);
        }
    }
    fn wasm_config_config<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) {
        if (var0 > 0i32) as i32 != 0 {
            self.global37 = 1i32;
        }
        if (var1 > 0i32) as i32 != 0 {
            self.global38 = 1i32;
        }
        if (var2 > 0i32) as i32 != 0 {
            self.global39 = 1i32;
        }
        if (var3 > 0i32) as i32 != 0 {
            self.global40 = 1i32;
        }
        if (var4 > 0i32) as i32 != 0 {
            self.global41 = 1i32;
        }
    }
    fn wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 140448i32;
        }
        70224i32
    }
    fn wasm_helpers_index_checkBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var1 & 1i32.wrapping_shl(var0 as u32) != 0i32) as i32
    }
    fn wasm_helpers_index_resetBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1 & (1i32.wrapping_shl(var0 as u32) & 255i32 ^ -1i32) & 255i32
    }
    fn wasm_helpers_index_setBitOnByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var1 | 1i32.wrapping_shl(var0 as u32) & 255i32) & 255i32
    }
    fn wasm_joypad_index_getJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65280i32);
        var0 = (var1 ^ 255i32) & 255i32;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var0);
        if var2 != 0 {
            let var3 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var0);
            if (var3 == 0) as i32 != 0 {
                var0 = (var0 | 240i32) & 255i32;
                let var4 = self.global51;
                let var5: i32;
                if var4 != 0 {
                    let var6 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var0);
                    var5 = var6;
                } else {
                    let var7 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var0);
                    var5 = var7;
                }
                var0 = var5;
                let var8 = self.global52;
                let var9: i32;
                if var8 != 0 {
                    let var10 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                    var9 = var10;
                } else {
                    let var11 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var0);
                    var9 = var11;
                }
                var0 = var9;
                let var12 = self.global53;
                let var13: i32;
                if var12 != 0 {
                    let var14 = self.wasm_helpers_index_resetBitOnByte(imports, 3i32, var0);
                    var13 = var14;
                } else {
                    let var15 = self.wasm_helpers_index_setBitOnByte(imports, 3i32, var0);
                    var13 = var15;
                }
                var0 = var13;
                let var16 = self.global54;
                let var17: i32;
                if var16 != 0 {
                    let var18 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var0);
                    var17 = var18;
                } else {
                    let var19 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var0);
                    var17 = var19;
                }
                var0 = var17;
            }
        } else {
            var0 = (var0 | 240i32) & 255i32;
            let var20 = self.global47;
            let var21: i32;
            if var20 != 0 {
                let var22 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                var21 = var22;
            } else {
                let var23 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var0);
                var21 = var23;
            }
            var0 = var21;
            let var24 = self.global48;
            let var25: i32;
            if var24 != 0 {
                let var26 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var0);
                var25 = var26;
            } else {
                let var27 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var0);
                var25 = var27;
            }
            var0 = var25;
            let var28 = self.global49;
            let var29: i32;
            if var28 != 0 {
                let var30 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var0);
                var29 = var30;
            } else {
                let var31 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var0);
                var29 = var31;
            }
            var0 = var29;
            let var32 = self.global50;
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
        var0
    }
    fn wasm_sound_sound_Sound_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 174i32;
        }
        87i32
    }
    fn wasm_sound_sound_Sound_maxFrameSequenceCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 16384i32;
        }
        8192i32
    }
    fn wasm_sound_registers_getRegister4OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                    let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65300i32);
                    return var1;
                    break;
                }
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65305i32);
                return var2;
                break;
            }
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65310i32);
            return var3;
            break;
        }
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65315i32);
        var4
    }
    fn wasm_sound_length_isChannelLengthEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister4OfChannel(imports, var0);
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var1);
        var2
    }
    fn wasm_sound_channel1_Channel1_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global58;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.wasm_sound_length_isChannelLengthEnabled(imports, 1i32);
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global58;
            self.global58 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global58;
        if (var5 == 0) as i32 != 0 {
            self.global59 = 0i32;
        }
    }
    fn wasm_sound_channel2_Channel2_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global60;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.wasm_sound_length_isChannelLengthEnabled(imports, 2i32);
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global60;
            self.global60 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global60;
        if (var5 == 0) as i32 != 0 {
            self.global61 = 0i32;
        }
    }
    fn wasm_sound_channel3_Channel3_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global62;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.wasm_sound_length_isChannelLengthEnabled(imports, 3i32);
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global62;
            self.global62 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global62;
        if (var5 == 0) as i32 != 0 {
            self.global63 = 0i32;
        }
    }
    fn wasm_sound_channel4_Channel4_updateLength<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global64;
        var0 = (var1 > 0i32) as i32;
        let var2: i32;
        if var0 != 0 {
            let var3 = self.wasm_sound_length_isChannelLengthEnabled(imports, 4i32);
            var2 = var3;
        } else {
            var2 = var0;
        }
        if var2 & 1i32 != 0 {
            let var4 = self.global64;
            self.global64 = var4.wrapping_sub(1i32);
        }
        let var5 = self.global64;
        if (var5 == 0) as i32 != 0 {
            self.global65 = 0i32;
        }
    }
    fn wasm_sound_channel1_getSweepPeriod<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65296i32);
        ((var0 & 112i32) as u32).wrapping_shr(4i32 as u32) as i32
    }
    fn wasm_sound_channel1_getSweepShift<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65296i32);
        var0 & 7i32
    }
    fn wasm_sound_channel1_getNewFrequencyFromSweep<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global68;
        let var2 = self.wasm_sound_channel1_getSweepShift(imports);
        var0 = (var1 as u32).wrapping_shr(var2 as u32) as i32;
        let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65296i32);
        let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var3);
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global68;
            var5 = var6.wrapping_sub(var0) & 65535i32;
        } else {
            let var7 = self.global68;
            var5 = var7.wrapping_add(var0) & 65535i32;
        }
        var0 = var5;
        var0
    }
    fn wasm_sound_registers_setRegister3OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
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
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label1;
                            break;
                        }
                        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65299i32, var1);
                        break 'label0;
                        break;
                    }
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65304i32, var1);
                    break 'label0;
                    break;
                }
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65309i32, var1);
                break 'label0;
                break;
            }
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65314i32, var1);
            break;
        }
    }
    fn wasm_sound_registers_setRegister4OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
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
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label1;
                            break;
                        }
                        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65300i32, var1);
                        break 'label0;
                        break;
                    }
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65305i32, var1);
                    break 'label0;
                    break;
                }
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65310i32, var1);
                break 'label0;
                break;
            }
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65315i32, var1);
            break;
        }
    }
    fn wasm_sound_frequency_setChannelFrequency<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.wasm_sound_registers_getRegister4OfChannel(imports, var0);
        var2 = var3 & 248i32 | (var1 as u32).wrapping_shr(8i32 as u32) as i32 & 255i32;
        self.wasm_sound_registers_setRegister3OfChannel(imports, var0, var1 & 255i32);
        self.wasm_sound_registers_setRegister4OfChannel(imports, var0, var2);
    }
    fn wasm_sound_channel1_calculateSweepAndCheckOverflow<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.wasm_sound_channel1_getNewFrequencyFromSweep(imports);
        var0 = var2;
        var1 = (var0 as u32 <= 2047i32 as u32) as i32;
        let var3: i32;
        if var1 != 0 {
            let var4 = self.wasm_sound_channel1_getSweepShift(imports);
            var3 = (var4 as u32 > 0i32 as u32) as i32;
        } else {
            var3 = var1;
        }
        if var3 & 1i32 != 0 {
            self.global68 = var0;
            self.wasm_sound_frequency_setChannelFrequency(imports, 1i32, var0);
            let var5 = self.wasm_sound_channel1_getNewFrequencyFromSweep(imports);
            var0 = var5;
        }
        if (var0 as u32 > 2047i32 as u32) as i32 != 0 {
            self.global59 = 0i32;
        }
    }
    fn wasm_sound_channel1_Channel1_updateSweep<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global66;
        self.global66 = var0.wrapping_sub(1i32);
        let var1 = self.global66;
        if (var1 <= 0i32) as i32 != 0 {
            let var2 = self.wasm_sound_channel1_getSweepPeriod(imports);
            self.global66 = var2;
            let var3 = self.global67;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.wasm_sound_channel1_getSweepPeriod(imports);
                var4 = (var5 as u32 > 0i32 as u32) as i32;
            } else {
                let var6 = self.global67;
                var4 = var6;
            }
            if var4 & 1i32 != 0 {
                self.wasm_sound_channel1_calculateSweepAndCheckOverflow(imports);
            }
        }
    }
    fn wasm_sound_registers_getRegister2OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                    let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65298i32);
                    return var1;
                    break;
                }
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65303i32);
                return var2;
                break;
            }
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65308i32);
            return var3;
            break;
        }
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65313i32);
        var4
    }
    fn wasm_sound_envelope_getChannelEnvelopePeriod<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister2OfChannel(imports, var0);
        var1 & 7i32
    }
    fn wasm_sound_envelope_getChannelEnvelopeAddMode<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister2OfChannel(imports, var0);
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var1);
        var2
    }
    fn wasm_sound_channel1_Channel1_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global69;
        self.global69 = var1.wrapping_sub(1i32);
        let var2 = self.global69;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 1i32);
            self.global69 = var3;
            let var4 = self.global69;
            if var4 != 0 {
                let var5 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 1i32);
                var0 = var5;
                let var6: i32;
                if var0 != 0 {
                    let var7 = self.global70;
                    var6 = ((var7) < 15i32) as i32;
                } else {
                    var6 = var0;
                }
                if var6 & 1i32 != 0 {
                    let var8 = self.global70;
                    self.global70 = var8.wrapping_add(1i32);
                } else {
                    let var9 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 1i32);
                    var0 = (var9 == 0) as i32;
                    let var10: i32;
                    if var0 != 0 {
                        let var11 = self.global70;
                        var10 = (var11 > 0i32) as i32;
                    } else {
                        var10 = var0;
                    }
                    if var10 & 1i32 != 0 {
                        let var12 = self.global70;
                        self.global70 = var12.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_channel2_Channel2_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global71;
        self.global71 = var1.wrapping_sub(1i32);
        let var2 = self.global71;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 2i32);
            self.global71 = var3;
            let var4 = self.global71;
            if var4 != 0 {
                let var5 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 2i32);
                var0 = var5;
                let var6: i32;
                if var0 != 0 {
                    let var7 = self.global72;
                    var6 = ((var7) < 15i32) as i32;
                } else {
                    var6 = var0;
                }
                if var6 & 1i32 != 0 {
                    let var8 = self.global72;
                    self.global72 = var8.wrapping_add(1i32);
                } else {
                    let var9 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 2i32);
                    var0 = (var9 == 0) as i32;
                    let var10: i32;
                    if var0 != 0 {
                        let var11 = self.global72;
                        var10 = (var11 > 0i32) as i32;
                    } else {
                        var10 = var0;
                    }
                    if var10 & 1i32 != 0 {
                        let var12 = self.global72;
                        self.global72 = var12.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_channel4_Channel4_updateEnvelope<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global73;
        self.global73 = var1.wrapping_sub(1i32);
        let var2 = self.global73;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 4i32);
            self.global73 = var3;
            let var4 = self.global73;
            if var4 != 0 {
                let var5 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 4i32);
                var0 = var5;
                let var6: i32;
                if var0 != 0 {
                    let var7 = self.global74;
                    var6 = ((var7) < 15i32) as i32;
                } else {
                    var6 = var0;
                }
                if var6 & 1i32 != 0 {
                    let var8 = self.global74;
                    self.global74 = var8.wrapping_add(1i32);
                } else {
                    let var9 = self.wasm_sound_envelope_getChannelEnvelopeAddMode(imports, 4i32);
                    var0 = (var9 == 0) as i32;
                    let var10: i32;
                    if var0 != 0 {
                        let var11 = self.global74;
                        var10 = (var11 > 0i32) as i32;
                    } else {
                        var10 = var0;
                    }
                    if var10 & 1i32 != 0 {
                        let var12 = self.global74;
                        self.global74 = var12.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    fn wasm_sound_sound_updateFrameSequencer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global56;
        self.global56 = var1.wrapping_add(var0);
        let var2 = self.global56;
        let var3 = self.wasm_sound_sound_Sound_maxFrameSequenceCycles(imports);
        if (var2 >= var3) as i32 != 0 {
            let var4 = self.global56;
            let var5 = self.wasm_sound_sound_Sound_maxFrameSequenceCycles(imports);
            self.global56 = var4.wrapping_sub(var5);
            'label0: loop {
                'label1: loop {
                    'label2: loop {
                        'label3: loop {
                            'label4: loop {
                                'label5: loop {
                                    'label6: loop {
                                        let var6 = self.global57;
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
            let var7 = self.global57;
            self.global57 = var7.wrapping_add(1i32) & 255i32;
            let var8 = self.global57;
            if (var8 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global57 = 0i32;
            }
            return 1i32;
        }
        0i32
    }
    fn wasm_sound_registers_isChannelDacEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1: i32;
        if (var0 != 3i32) as i32 != 0 {
            let var2 = self.wasm_sound_registers_getRegister2OfChannel(imports, var0);
            let var3: i32;
            if ((var2 & 248i32) as u32 > 0i32 as u32) as i32 != 0 {
                var3 = 1i32;
            } else {
                var3 = 0i32;
            }
            var1 = var3;
        } else {
            let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65306i32);
            let var5 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var4);
            var1 = var5;
        }
        var1
    }
    fn wasm_sound_channel1_Channel1_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global75;
        self.global75 = var2.wrapping_add(var0);
        let var3 = self.global76;
        let var4 = self.global75;
        var1 = (var3.wrapping_sub(var4) > 0i32) as i32;
        let var5: i32;
        if var1 != 0 {
            let var6 = self.wasm_sound_registers_isChannelDacEnabled(imports, 1i32);
            var5 = var6;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel2_Channel2_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global77;
        self.global77 = var2.wrapping_add(var0);
        let var3 = self.global78;
        let var4 = self.global77;
        var1 = (var3.wrapping_sub(var4) > 0i32) as i32;
        let var5: i32;
        if var1 != 0 {
            let var6 = self.wasm_sound_registers_isChannelDacEnabled(imports, 2i32);
            var5 = var6;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel3_Channel3_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global79;
        self.global79 = var2.wrapping_add(var0);
        let var3 = self.global80;
        let var4 = self.global79;
        var1 = (var3.wrapping_sub(var4) > 0i32) as i32;
        let var5: i32;
        if var1 != 0 {
            let var6 = self.wasm_sound_registers_isChannelDacEnabled(imports, 3i32);
            var5 = var6;
        } else {
            var5 = var1;
        }
        var1 = var5 & 1i32;
        let var7: i32;
        if var1 != 0 {
            let var8 = self.global34;
            var7 = (var8 == 0) as i32;
        } else {
            var7 = var1;
        }
        if var7 & 1i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_channel4_Channel4_willChannelUpdate<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global81;
        self.global81 = var2.wrapping_add(var0);
        let var3 = self.global82;
        let var4 = self.global81;
        var1 = (var3.wrapping_sub(var4) > 0i32) as i32;
        let var5: i32;
        if var1 != 0 {
            let var6 = self.wasm_sound_registers_isChannelDacEnabled(imports, 4i32);
            var5 = var6;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            return 0i32;
        }
        1i32
    }
    fn wasm_sound_registers_getRegister3OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                    let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65299i32);
                    return var1;
                    break;
                }
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65304i32);
                return var2;
                break;
            }
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65309i32);
            return var3;
            break;
        }
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65314i32);
        var4
    }
    fn wasm_sound_frequency_getChannelFrequency<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister4OfChannel(imports, var0);
        let var2 = self.wasm_sound_registers_getRegister3OfChannel(imports, var0);
        ((var1 & 7i32).wrapping_shl(8i32 as u32) | var2) & 65535i32
    }
    fn wasm_sound_channel1_Channel1_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_sound_frequency_getChannelFrequency(imports, 1i32);
        self.global76 = 2048i32.wrapping_sub(var0).wrapping_mul(4i32);
        let var1 = self.global43;
        if var1 != 0 {
            let var2 = self.global76;
            self.global76 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_registers_getRegister1OfChannel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                    let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65297i32);
                    return var1;
                    break;
                }
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65302i32);
                return var2;
                break;
            }
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65307i32);
            return var3;
            break;
        }
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65312i32);
        var4
    }
    fn wasm_sound_duty_getChannelDuty<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister1OfChannel(imports, var0);
        (var1 as u32).wrapping_shr(6i32 as u32) as i32 & 3i32
    }
    fn wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = self.wasm_sound_duty_getChannelDuty(imports, var0);
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var3 = self.wasm_sound_duty_getChannelDuty(imports, var0);
                            match var3.wrapping_sub(1i32) {
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
                    let var4 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 129i32);
                    return var4;
                    break;
                }
                let var5 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 135i32);
                return var5;
                break;
            }
            let var6 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 126i32);
            return var6;
            break;
        }
        let var7 = self.wasm_helpers_index_checkBitOnByte(imports, var1, 1i32);
        var7
    }
    fn wasm_sound_channel1_Channel1_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global76;
        self.global76 = var2.wrapping_sub(var0);
        let var3 = self.global76;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global76;
            var0 = var4;
            var0 = if (var0 > 0i32) as i32 != 0 { var0 } else { 0i32.wrapping_sub(var0) };
            self.wasm_sound_channel1_Channel1_resetTimer(imports);
            let var5 = self.global76;
            self.global76 = var5.wrapping_sub(var0);
            let var6 = self.global84;
            self.global84 = var6.wrapping_add(1i32) & 255i32;
            let var7 = self.global84;
            if (var7 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global84 = 0i32;
            }
        }
        let var8 = self.global59;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.wasm_sound_registers_isChannelDacEnabled(imports, 1i32);
            var9 = var10;
        } else {
            let var11 = self.global59;
            var9 = var11;
        }
        if var9 & 1i32 != 0 {
            let var12 = self.global70;
            var0 = var12;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var13 = self.global84;
        let var14 = self.wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform(imports, 1i32, var13);
        if (var14 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        var1.wrapping_mul(var0).wrapping_add(15i32)
    }
    fn wasm_sound_channel1_Channel1_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global75;
        var0 = var1;
        self.global75 = 0i32;
        let var2 = self.wasm_sound_channel1_Channel1_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel2_Channel2_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_sound_frequency_getChannelFrequency(imports, 2i32);
        self.global78 = 2048i32.wrapping_sub(var0).wrapping_mul(4i32);
        let var1 = self.global43;
        if var1 != 0 {
            let var2 = self.global78;
            self.global78 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_channel2_Channel2_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global78;
        self.global78 = var2.wrapping_sub(var0);
        let var3 = self.global78;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global78;
            var0 = var4;
            var0 = if (var0 > 0i32) as i32 != 0 { var0 } else { 0i32.wrapping_sub(var0) };
            self.wasm_sound_channel2_Channel2_resetTimer(imports);
            let var5 = self.global78;
            self.global78 = var5.wrapping_sub(var0);
            let var6 = self.global86;
            self.global86 = var6.wrapping_add(1i32) & 255i32;
            let var7 = self.global86;
            if (var7 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global86 = 0i32;
            }
        }
        let var8 = self.global61;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.wasm_sound_registers_isChannelDacEnabled(imports, 2i32);
            var9 = var10;
        } else {
            let var11 = self.global61;
            var9 = var11;
        }
        if var9 & 1i32 != 0 {
            let var12 = self.global72;
            var0 = var12;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var13 = self.global86;
        let var14 = self.wasm_sound_duty_isDutyCycleClockPositiveOrNegativeForWaveform(imports, 1i32, var13);
        if (var14 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        var1.wrapping_mul(var0).wrapping_add(15i32)
    }
    fn wasm_sound_channel2_Channel2_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global77;
        var0 = var1;
        self.global77 = 0i32;
        let var2 = self.wasm_sound_channel2_Channel2_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel3_Channel3_resetTimer<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_sound_frequency_getChannelFrequency(imports, 3i32);
        self.global80 = 2048i32.wrapping_sub(var0).wrapping_mul(2i32);
        let var1 = self.global43;
        if var1 != 0 {
            let var2 = self.global80;
            self.global80 = var2.wrapping_mul(2i32);
        }
    }
    fn wasm_sound_channel3_Channel3_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global80;
        self.global80 = var3.wrapping_sub(var0);
        let var4 = self.global80;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global80;
            var1 = var5;
            var1 = if (var1 > 0i32) as i32 != 0 { var1 } else { 0i32.wrapping_sub(var1) };
            self.wasm_sound_channel3_Channel3_resetTimer(imports);
            let var6 = self.global80;
            self.global80 = var6.wrapping_sub(var1);
            let var7 = self.global88;
            self.global88 = var7.wrapping_add(1i32) & 65535i32;
            let var8 = self.global88;
            if (var8 as u32 >= 32i32 as u32) as i32 != 0 {
                self.global88 = 0i32;
            }
        }
        var1 = 0i32;
        let var9 = self.global89;
        var2 = var9;
        let var10 = self.global63;
        let var11: i32;
        if var10 != 0 {
            let var12 = self.wasm_sound_registers_isChannelDacEnabled(imports, 3i32);
            var11 = var12;
        } else {
            let var13 = self.global63;
            var11 = var13;
        }
        if var11 & 1i32 != 0 {
            let var14 = self.global34;
            if var14 != 0 {
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65308i32);
                self.global89 = (var15 as u32).wrapping_shr(5i32 as u32) as i32 & 15i32;
                self.global34 = 0i32;
            }
        } else {
            return 15i32;
        }
        let var16 = self.global88;
        let var17 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, ((var16 as u32 / 2i32 as u32) as i32 & 65535i32).wrapping_add(65328i32) & 65535i32);
        var0 = var17;
        let var18 = self.global88;
        let var19: i32;
        if (var18 as u32).wrapping_rem(2i32 as u32) as i32 != 0 {
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
            var20 = (var0 / var1).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        } else {
            var20 = 0i32;
        }
        var0 = var20;
        var0.wrapping_add(15i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32)
    }
    fn wasm_sound_channel3_Channel3_getSampleFromCycleCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global79;
        var0 = var1;
        self.global79 = 0i32;
        let var2 = self.wasm_sound_channel3_Channel3_getSample(imports, var0);
        var2
    }
    fn wasm_sound_channel4_Channel4_getNoiseChannelDivisorFromDivisorCode<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65314i32);
        var1 = var2 & 7i32;
        if var1 != 0 {
            if (var1 == 1i32) as i32 != 0 {
                var0 = 16i32;
            } else {
                if (var1 == 2i32) as i32 != 0 {
                    var0 = 32i32;
                } else {
                    if (var1 == 3i32) as i32 != 0 {
                        var0 = 48i32;
                    } else {
                        if (var1 == 4i32) as i32 != 0 {
                            var0 = 64i32;
                        } else {
                            if (var1 == 5i32) as i32 != 0 {
                                var0 = 80i32;
                            } else {
                                if (var1 == 6i32) as i32 != 0 {
                                    var0 = 96i32;
                                } else {
                                    if (var1 == 7i32) as i32 != 0 {
                                        var0 = 112i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            var0 = 8i32;
        }
        var0
    }
    fn wasm_sound_channel4_Channel4_getNoiseChannelClockShift<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65314i32);
        (var0 as u32).wrapping_shr(4i32 as u32) as i32
    }
    fn wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.wasm_sound_channel4_Channel4_getNoiseChannelDivisorFromDivisorCode(imports);
        let var2 = self.wasm_sound_channel4_Channel4_getNoiseChannelClockShift(imports);
        var0 = var1.wrapping_shl(var2 as u32) & 65535i32;
        let var3 = self.global43;
        if var3 != 0 {
            var0 = var0.wrapping_mul(2i32) & 65535i32;
        }
        var0
    }
    fn wasm_sound_channel4_Channel4_isNoiseChannelWidthModeSet<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65314i32);
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var0);
        var1
    }
    fn wasm_sound_channel4_Channel4_getSample<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global82;
        self.global82 = var2.wrapping_sub(var0);
        let var3 = self.global82;
        if (var3 <= 0i32) as i32 != 0 {
            let var4 = self.global82;
            var0 = var4;
            var0 = if (var0 > 0i32) as i32 != 0 { var0 } else { 0i32.wrapping_sub(var0) };
            let var5 = self.wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod(imports);
            self.global82 = var5;
            let var6 = self.global82;
            self.global82 = var6.wrapping_sub(var0);
            let var7 = self.global91;
            let var8 = self.global91;
            var1 = var7 & 1i32 ^ (var8 as u32).wrapping_shr(1i32 as u32) as i32 & 1i32;
            let var9 = self.global91;
            self.global91 = (var9 as u32).wrapping_shr(1i32 as u32) as i32;
            let var10 = self.global91;
            self.global91 = (var10 | var1.wrapping_shl(14i32 as u32) & 65535i32) & 65535i32;
            let var11 = self.wasm_sound_channel4_Channel4_isNoiseChannelWidthModeSet(imports);
            if var11 != 0 {
                let var12 = self.global91;
                self.global91 = var12 & 65471i32;
                let var13 = self.global91;
                self.global91 = (var13 | var1.wrapping_shl(6i32 as u32) & 65535i32) & 65535i32;
            }
        }
        let var14 = self.global65;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.wasm_sound_registers_isChannelDacEnabled(imports, 4i32);
            var15 = var16;
        } else {
            let var17 = self.global65;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global74;
            var1 = var18;
        } else {
            return 15i32;
        }
        let var19 = self.global91;
        let var20 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var19 & 255i32);
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
        let var1 = self.global81;
        var0 = var1;
        self.global81 = 0i32;
        let var2 = self.wasm_sound_channel4_Channel4_getSample(imports, var0);
        var2
    }
    fn wasm_sound_registers_isChannelEnabledOnLeftOutput<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65317i32);
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, ((var0 & 255i32).wrapping_sub(1i32) & 255i32).wrapping_add(4i32) & 255i32, var1);
        var2
    }
    fn wasm_sound_registers_isChannelEnabledOnRightOutput<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65317i32);
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, (var0 & 255i32).wrapping_sub(1i32) & 255i32, var1);
        var2
    }
    fn wasm_sound_sound_getSampleAsUnsignedByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        if (var0 == 60i32) as i32 != 0 {
            return 127i32;
        }
        var2 = 100000i32;
        (var0.wrapping_sub(60i32).wrapping_mul(var2).wrapping_mul(var1) / 8i32 / 100000i32).wrapping_add(60i32).wrapping_mul(100000i32) / 47244i32 & 255i32
    }
    fn wasm_helpers_index_concatenateBytes<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (var0 & 255i32).wrapping_shl(8i32 as u32) | var1 & 255i32
    }
    fn wasm_sound_sound_mixChannelSamples<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let var12 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65316i32);
        var6 = var12;
        var7 = (var6 as u32).wrapping_shr(4i32 as u32) as i32 & 7i32;
        var6 = var6 & 7i32;
        self.global35 = 0i32;
        var8 = 1i32;
        let var13 = self.wasm_sound_registers_isChannelEnabledOnLeftOutput(imports, var8);
        let var14: i32;
        if var13 != 0 {
            var14 = 0i32.wrapping_add(var0);
        } else {
            var14 = 15i32;
        }
        var4 = var14;
        var9 = 2i32;
        let var15 = self.wasm_sound_registers_isChannelEnabledOnLeftOutput(imports, var9);
        let var16: i32;
        if var15 != 0 {
            var16 = var4.wrapping_add(var1);
        } else {
            var16 = var4.wrapping_add(15i32);
        }
        var4 = var16;
        var10 = 3i32;
        let var17 = self.wasm_sound_registers_isChannelEnabledOnLeftOutput(imports, var10);
        let var18: i32;
        if var17 != 0 {
            var18 = var4.wrapping_add(var2);
        } else {
            var18 = var4.wrapping_add(15i32);
        }
        var4 = var18;
        var11 = 4i32;
        let var19 = self.wasm_sound_registers_isChannelEnabledOnLeftOutput(imports, var11);
        let var20: i32;
        if var19 != 0 {
            var20 = var4.wrapping_add(var3);
        } else {
            var20 = var4.wrapping_add(15i32);
        }
        var4 = var20;
        let var21 = self.wasm_sound_registers_isChannelEnabledOnRightOutput(imports, 1i32);
        let var22: i32;
        if var21 != 0 {
            var22 = 0i32.wrapping_add(var0);
        } else {
            var22 = 15i32;
        }
        var5 = var22;
        let var23 = self.wasm_sound_registers_isChannelEnabledOnRightOutput(imports, 2i32);
        let var24: i32;
        if var23 != 0 {
            var24 = var5.wrapping_add(var1);
        } else {
            var24 = var5.wrapping_add(15i32);
        }
        var5 = var24;
        let var25 = self.wasm_sound_registers_isChannelEnabledOnRightOutput(imports, 3i32);
        let var26: i32;
        if var25 != 0 {
            var26 = var5.wrapping_add(var2);
        } else {
            var26 = var5.wrapping_add(15i32);
        }
        var5 = var26;
        let var27 = self.wasm_sound_registers_isChannelEnabledOnRightOutput(imports, 4i32);
        let var28: i32;
        if var27 != 0 {
            var28 = var5.wrapping_add(var3);
        } else {
            var28 = var5.wrapping_add(15i32);
        }
        var5 = var28;
        self.global36 = 0i32;
        let var29 = self.wasm_sound_sound_getSampleAsUnsignedByte(imports, var4, var7.wrapping_add(1i32));
        var0 = var29;
        let var30 = self.wasm_sound_sound_getSampleAsUnsignedByte(imports, var5, var6.wrapping_add(1i32));
        var1 = var30;
        self.global92 = var0;
        self.global93 = var1;
        let var31 = self.wasm_helpers_index_concatenateBytes(imports, var0, var1);
        var31
    }
    fn wasm_cpu_cpu_Cpu_CLOCK_SPEED<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 8388608i32;
        }
        4194304i32
    }
    fn wasm_sound_sound_Sound_maxDownSampleCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_cpu_cpu_Cpu_CLOCK_SPEED(imports);
        var0
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
        let var6 = self.wasm_sound_channel2_Channel2_willChannelUpdate(imports, var0);
        var2 = var6;
        let var7 = self.wasm_sound_channel3_Channel3_willChannelUpdate(imports, var0);
        var3 = var7;
        let var8 = self.wasm_sound_channel4_Channel4_willChannelUpdate(imports, var0);
        var4 = var8;
        if var1 != 0 {
            let var9 = self.wasm_sound_channel1_Channel1_getSampleFromCycleCounter(imports);
            self.global83 = var9;
        }
        if var2 != 0 {
            let var10 = self.wasm_sound_channel2_Channel2_getSampleFromCycleCounter(imports);
            self.global85 = var10;
        }
        if var3 != 0 {
            let var11 = self.wasm_sound_channel3_Channel3_getSampleFromCycleCounter(imports);
            self.global87 = var11;
        }
        if var4 != 0 {
            let var12 = self.wasm_sound_channel4_Channel4_getSampleFromCycleCounter(imports);
            self.global90 = var12;
        }
        let var13: i32;
        if var1 != 0 {
            var13 = var1;
        } else {
            var13 = var2;
        }
        var1 = var13 & 1i32;
        let var14: i32;
        if var1 != 0 {
            var14 = var1;
        } else {
            var14 = var3;
        }
        var1 = var14 & 1i32;
        let var15: i32;
        if var1 != 0 {
            var15 = var1;
        } else {
            var15 = var4;
        }
        if var15 & 1i32 != 0 {
            let var16 = self.global83;
            let var17 = self.global85;
            let var18 = self.global87;
            let var19 = self.global90;
            let var20 = self.wasm_sound_sound_mixChannelSamples(imports, var16, var17, var18, var19);
        }
        let var21 = self.global94;
        let var22 = self.global95;
        self.global94 = var21.wrapping_add(var0.wrapping_mul(var22));
        let var23 = self.global94;
        let var24 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
        if (var23 >= var24) as i32 != 0 {
            let var25 = self.global94;
            let var26 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
            self.global94 = var25.wrapping_sub(var26);
            let var27 = self.global35;
            let var28: i32;
            if var27 != 0 {
                let var29 = self.global35;
                var28 = var29;
            } else {
                let var30 = self.global36;
                var28 = var30;
            }
            if var28 & 1i32 != 0 {
                let var31 = self.global83;
                let var32 = self.global85;
                let var33 = self.global87;
                let var34 = self.global90;
                let var35 = self.wasm_sound_sound_mixChannelSamples(imports, var31, var32, var33, var34);
            }
            let var36 = self.global92;
            let var37 = self.global93;
            let var38 = self.global96;
            self.wasm_memory_memory_setLeftAndRightOutputForAudioQueue(imports, var36.wrapping_add(1i32) & 255i32, var37.wrapping_add(1i32) & 255i32, var38);
            let var39 = self.global96;
            self.global96 = var39.wrapping_add(1i32);
            let var40 = self.global96;
            let var41 = self.global97;
            if (var40 >= (var41 / 2i32).wrapping_sub(1i32)) as i32 != 0 {
                let var42 = self.global96;
                self.global96 = var42.wrapping_sub(1i32);
            }
        }
    }
    fn wasm_helpers_index_splitHighByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        ((var0 & 65280i32) as u32).wrapping_shr(8i32 as u32) as i32
    }
    fn wasm_helpers_index_splitLowByte<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        var0 & 255i32
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
        self.global83 = var1;
        self.global85 = var2;
        self.global87 = var3;
        self.global90 = var4;
        let var9 = self.global94;
        let var10 = self.global95;
        self.global94 = var9.wrapping_add(var0.wrapping_mul(var10));
        let var11 = self.global94;
        let var12 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
        if (var11 >= var12) as i32 != 0 {
            let var13 = self.global94;
            let var14 = self.wasm_sound_sound_Sound_maxDownSampleCycles(imports);
            self.global94 = var13.wrapping_sub(var14);
            let var15 = self.wasm_sound_sound_mixChannelSamples(imports, var1, var2, var3, var4);
            var0 = var15;
            let var16 = self.wasm_helpers_index_splitHighByte(imports, var0);
            let var17 = self.wasm_helpers_index_splitLowByte(imports, var0);
            let var18 = self.global96;
            self.wasm_memory_memory_setLeftAndRightOutputForAudioQueue(imports, var16.wrapping_add(1i32) & 255i32, var17.wrapping_add(1i32) & 255i32, var18);
            let var19 = self.global96;
            self.global96 = var19.wrapping_add(1i32);
            let var20 = self.global96;
            let var21 = self.global97;
            if (var20 >= (var21 / 2i32).wrapping_sub(1i32)) as i32 != 0 {
                let var22 = self.global96;
                self.global96 = var22.wrapping_sub(1i32);
            }
        }
    }
    fn wasm_sound_sound_updateSound<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.wasm_sound_sound_updateFrameSequencer(imports, var0);
        var1 = var2;
        let var3 = self.global41;
        let var4: i32;
        if var3 != 0 {
            var4 = (var1 == 0) as i32;
        } else {
            let var5 = self.global41;
            var4 = var5;
        }
        if var4 & 1i32 != 0 {
            self.wasm_sound_sound_accumulateSound(imports, var0);
        } else {
            self.wasm_sound_sound_calculateSound(imports, var0);
        }
    }
    fn wasm_sound_sound_batchProcessAudio<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global55;
        let var1 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
        if ((var0) < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global55;
            let var3 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
                self.wasm_sound_sound_updateSound(imports, var4);
                let var5 = self.global55;
                let var6 = self.wasm_sound_sound_Sound_batchProcessCycles(imports);
                self.global55 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_sound_registers_handleReadToSoundRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        if (var0 == 65318i32) as i32 != 0 {
            let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65318i32);
            var1 = var2 & 128i32;
            let var3 = self.global59;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var1);
                var4 = var5;
            } else {
                let var6 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var1);
                var4 = var6;
            }
            let var7 = self.global61;
            let var8: i32;
            if var7 != 0 {
                let var9 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var1);
                var8 = var9;
            } else {
                let var10 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var1);
                var8 = var10;
            }
            let var11 = self.global63;
            let var12: i32;
            if var11 != 0 {
                let var13 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var1);
                var12 = var13;
            } else {
                let var14 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var1);
                var12 = var14;
            }
            let var15 = self.global65;
            let var16: i32;
            if var15 != 0 {
                let var17 = self.wasm_helpers_index_setBitOnByte(imports, 3i32, var1);
                var16 = var17;
            } else {
                let var18 = self.wasm_helpers_index_resetBitOnByte(imports, 3i32, var1);
                var16 = var18;
            }
            return var1 | 112i32;
        }
        -1i32
    }
    fn wasm_memory_readTraps_checkReadTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        var1 = 32768i32;
        if ((var0 as u32) < var1 as u32) as i32 != 0 {
            return -1i32;
        }
        var1 = (var0 as u32 >= 32768i32 as u32) as i32;
        let var2: i32;
        if var1 != 0 {
            var2 = ((var0 as u32) < 40960i32 as u32) as i32;
        } else {
            var2 = var1;
        }
        var1 = (var0 as u32 >= 57344i32 as u32) as i32;
        let var3: i32;
        if var1 != 0 {
            var3 = ((var0 as u32) < 65024i32 as u32) as i32;
        } else {
            var3 = var1;
        }
        if var3 & 1i32 != 0 {
            let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var0.wrapping_sub(8192i32) & 65535i32);
            return var4;
        }
        var1 = (var0 as u32 >= 65024i32 as u32) as i32;
        let var5: i32;
        if var1 != 0 {
            var5 = (var0 as u32 <= 65183i32 as u32) as i32;
        } else {
            var5 = var1;
        }
        if var5 & 1i32 != 0 {
            let var6 = self.global46;
            if ((var6 as u32) < 2i32 as u32) as i32 != 0 {
                return 255i32;
            }
        }
        if (var0 == 65280i32) as i32 != 0 {
            let var7 = self.wasm_joypad_index_getJoypadState(imports);
            return var7;
        }
        var1 = (var0 as u32 >= 65296i32 as u32) as i32;
        let var8: i32;
        if var1 != 0 {
            var8 = (var0 as u32 <= 65318i32 as u32) as i32;
        } else {
            var8 = var1;
        }
        if var8 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
            let var9 = self.wasm_sound_registers_handleReadToSoundRegister(imports, var0);
            var1 = var9;
            if ((var1) < 0i32) as i32 != 0 {
                return -1i32;
            }
            return var1 & 255i32;
        }
        var1 = (var0 as u32 >= 65328i32 as u32) as i32;
        let var10: i32;
        if var1 != 0 {
            var10 = (var0 as u32 <= 65343i32 as u32) as i32;
        } else {
            var10 = var1;
        }
        if var10 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
        }
        -1i32
    }
    fn wasm_memory_load_eightBitLoadFromGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.wasm_memory_readTraps_checkReadTraps(imports, var0);
        var1 = var2;
        if (var1 == -1i32) as i32 != 0 {
            let var3 = self.wasm_memory_load__eightBitLoadFromWasmBoyMemory(imports, var0);
            return var3;
        }
        var1 & 255i32
    }
    fn wasm_memory_banking_handleBanking<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global30;
        if var4 != 0 {
            return;
        }
        if (var0 as u32 <= 8191i32 as u32) as i32 != 0 {
            let var5 = self.global32;
            let var6: i32;
            if var5 != 0 {
                let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var1 & 255i32);
                var6 = (var7 == 0) as i32;
            } else {
                let var8 = self.global32;
                var6 = var8;
            }
            if (var6 & 1i32 == 0) as i32 != 0 {
                var2 = var1 & 15i32;
                if var2 != 0 {
                    if (var2 == 10i32) as i32 != 0 {
                        self.global98 = 1i32;
                    }
                } else {
                    self.global98 = 0i32;
                }
            }
        } else {
            if (var0 as u32 <= 16383i32 as u32) as i32 != 0 {
                let var9 = self.global15;
                var2 = (var9 == 0) as i32;
                let var10: i32;
                if var2 != 0 {
                    var10 = var2;
                } else {
                    var10 = (var0 as u32 <= 12287i32 as u32) as i32;
                }
                if var10 & 1i32 != 0 {
                    let var11 = self.global32;
                    if var11 != 0 {
                        self.global14 = var1 & 15i32;
                    }
                    var2 = var1;
                    let var12 = self.global31;
                    if var12 != 0 {
                        var2 = var2 & 31i32;
                        let var13 = self.global14;
                        self.global14 = var13 & 224i32;
                    } else {
                        let var14 = self.global33;
                        if var14 != 0 {
                            var2 = var2 & 127i32;
                            let var15 = self.global14;
                            self.global14 = var15 & 128i32;
                        } else {
                            let var16 = self.global15;
                            if var16 != 0 {
                                let var17 = self.global14;
                                self.global14 = var17 & 0i32;
                            }
                        }
                    }
                    let var18 = self.global14;
                    self.global14 = (var18 | var2) & 65535i32;
                } else {
                    var2 = 0i32;
                    let var19 = self.global14;
                    let var20 = self.wasm_helpers_index_splitLowByte(imports, var19);
                    var3 = var20;
                    if (var1 as u32 > 0i32 as u32) as i32 != 0 {
                        var2 = 1i32;
                    }
                    let var21 = self.wasm_helpers_index_concatenateBytes(imports, var2, var3);
                    self.global14 = var21;
                }
            } else {
                let var22 = self.global32;
                var3 = (var22 == 0) as i32;
                let var23: i32;
                if var3 != 0 {
                    var23 = (var0 as u32 <= 24575i32 as u32) as i32;
                } else {
                    var23 = var3;
                }
                if var23 & 1i32 != 0 {
                    let var24 = self.global31;
                    let var25: i32;
                    if var24 != 0 {
                        let var26 = self.global99;
                        var25 = var26;
                    } else {
                        let var27 = self.global31;
                        var25 = var27;
                    }
                    if var25 & 1i32 != 0 {
                        let var28 = self.global14;
                        self.global14 = var28 & 31i32;
                        let var29 = self.global14;
                        self.global14 = (var29 | var1 & 224i32) & 65535i32;
                        return;
                    }
                    let var30 = self.global33;
                    if var30 != 0 {
                        var3 = (var1 as u32 >= 8i32 as u32) as i32;
                        let var31: i32;
                        if var3 != 0 {
                            var31 = (var1 as u32 <= 12i32 as u32) as i32;
                        } else {
                            var31 = var3;
                        }
                    }
                    var3 = var1;
                    let var32 = self.global15;
                    let var33: i32;
                    if var32 != 0 {
                        var33 = var3 & 15i32;
                    } else {
                        var33 = var3 & 3i32;
                    }
                    var3 = var33;
                    self.global18 = var3;
                } else {
                    let var34 = self.global32;
                    var3 = (var34 == 0) as i32;
                    let var35: i32;
                    if var3 != 0 {
                        var35 = (var0 as u32 <= 32767i32 as u32) as i32;
                    } else {
                        var35 = var3;
                    }
                    if var35 & 1i32 != 0 {
                        let var36 = self.global31;
                        if var36 != 0 {
                            let var37 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var1 & 255i32);
                            if var37 != 0 {
                                self.global99 = 1i32;
                            } else {
                                self.global99 = 0i32;
                            }
                        }
                    }
                }
            }
        }
    }
    fn wasm_memory_store_sixteenBitStoreIntoGBMemorySkipTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.wasm_helpers_index_splitHighByte(imports, var1);
        var2 = var3;
        let var4 = self.wasm_helpers_index_splitLowByte(imports, var1);
        self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0, var4);
        self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0.wrapping_add(1i32) & 65535i32, var2);
    }
    fn wasm_timers_index_Timers_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 140448i32;
        }
        255i32
    }
    fn wasm_timers_index__checkDividerRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global103;
        self.global103 = var1.wrapping_add(var0);
        let var2 = self.global103;
        let var3 = self.wasm_timers_index_Timers_batchProcessCycles(imports);
        if (var2 >= var3) as i32 != 0 {
            let var4 = self.global103;
            let var5 = self.wasm_timers_index_Timers_batchProcessCycles(imports);
            self.global103 = var4.wrapping_sub(var5);
            let var6 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65284i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65284i32, var6.wrapping_add(1i32) & 255i32);
        }
    }
    fn wasm_interrupts_index__requestInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65295i32);
        let var2 = self.wasm_helpers_index_setBitOnByte(imports, var0, var1);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65295i32, var2);
    }
    fn wasm_interrupts_index_requestTimerInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_interrupts_index__requestInterrupt(imports, 2i32);
    }
    fn wasm_timers_index_updateTimers<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        self.wasm_timers_index__checkDividerRegister(imports, var0);
        let var2 = self.global100;
        if (var2 == 0) as i32 != 0 {
            return;
        }
        let var3 = self.global104;
        self.global104 = var3.wrapping_add(var0);
        'label0: loop {
            let var4 = self.global104;
            let var5 = self.global101;
            if (var4 >= var5) as i32 != 0 {
                let var6 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65285i32);
                var1 = var6;
                let var7 = self.global104;
                let var8 = self.global101;
                self.global104 = var7.wrapping_sub(var8);
                if (var1 as u32 >= 255i32 as u32) as i32 != 0 {
                    let var9 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65286i32);
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65285i32, var9);
                    self.wasm_interrupts_index_requestTimerInterrupt(imports);
                } else {
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65285i32, var1.wrapping_add(1i32) & 255i32);
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
        let var2 = self.global100;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.global101;
            var3 = ((var4) < var0) as i32;
        } else {
            let var5 = self.global100;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global101;
            var0 = var6;
        }
        let var7 = self.global102;
        if ((var7) < var0) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var8 = self.global102;
            if (var8 >= var0) as i32 != 0 {
                self.wasm_timers_index_updateTimers(imports, var0);
                let var9 = self.global102;
                self.global102 = var9.wrapping_sub(var0);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_timers_index_handleTIMCWrite<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var0);
        self.global100 = var2;
        let var3 = self.global100;
        if (var3 == 0) as i32 != 0 {
            return;
        }
        var0 = var0 & 3i32;
        var1 = 256i32;
        let var4 = self.global43;
        if var4 != 0 {
            var1 = 512i32;
        }
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            match var0 {
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
                    var1 = 1024i32;
                    let var5 = self.global43;
                    if var5 != 0 {
                        var1 = 2048i32;
                    }
                    break 'label0;
                    break;
                }
                var1 = 16i32;
                let var6 = self.global43;
                if var6 != 0 {
                    var1 = 32i32;
                }
                break 'label0;
                break;
            }
            var1 = 64i32;
            let var7 = self.global43;
            if var7 != 0 {
                var1 = 126i32;
            }
            break;
        }
        self.global104 = 0i32;
        self.global101 = var1;
    }
    fn wasm_sound_length_setChannelLengthCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.wasm_sound_registers_getRegister1OfChannel(imports, var0);
        var1 = var3 & 63i32;
        var2 = 64i32;
        let var4: i32;
        if (var0 == 3i32) as i32 != 0 {
            var4 = (255i32.wrapping_sub(var1) & 255i32).wrapping_add(1i32) & 255i32;
        } else {
            var4 = 64i32.wrapping_sub(var1) & 255i32;
        }
        var1 = var4;
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
                        self.global58 = var1;
                        break 'label0;
                        break;
                    }
                    self.global60 = var1;
                    break 'label0;
                    break;
                }
                self.global62 = var1;
                break 'label0;
                break;
            }
            self.global64 = var1;
            break;
        }
    }
    fn wasm_sound_registers_getChannelStartingVolume<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_sound_registers_getRegister2OfChannel(imports, var0);
        (var1 as u32).wrapping_shr(4i32 as u32) as i32 & 15i32
    }
    fn wasm_sound_channel1_Channel1_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        self.global59 = 1i32;
        let var1 = self.global58;
        if (var1 == 0) as i32 != 0 {
            self.global58 = 64i32;
        }
        self.wasm_sound_channel1_Channel1_resetTimer(imports);
        let var2 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 1i32);
        self.global69 = var2;
        let var3 = self.wasm_sound_registers_getChannelStartingVolume(imports, 1i32);
        self.global70 = var3;
        let var4 = self.wasm_sound_frequency_getChannelFrequency(imports, 1i32);
        self.global68 = var4;
        let var5 = self.wasm_sound_channel1_getSweepPeriod(imports);
        self.global66 = var5;
        let var6 = self.wasm_sound_channel1_getSweepPeriod(imports);
        var0 = (var6 as u32 > 0i32 as u32) as i32;
        let var7: i32;
        if var0 != 0 {
            let var8 = self.wasm_sound_channel1_getSweepShift(imports);
            var7 = (var8 as u32 > 0i32 as u32) as i32;
        } else {
            var7 = var0;
        }
        if var7 & 1i32 != 0 {
            self.global67 = 1i32;
        } else {
            self.global67 = 0i32;
        }
        let var9 = self.wasm_sound_channel1_getSweepShift(imports);
        if (var9 as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_sound_channel1_calculateSweepAndCheckOverflow(imports);
        }
        let var10 = self.wasm_sound_registers_isChannelDacEnabled(imports, 1i32);
        if (var10 == 0) as i32 != 0 {
            self.global59 = 0i32;
        }
    }
    fn wasm_sound_channel2_Channel2_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global61 = 1i32;
        let var0 = self.global60;
        if (var0 == 0) as i32 != 0 {
            self.global60 = 64i32;
        }
        self.wasm_sound_channel2_Channel2_resetTimer(imports);
        let var1 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 2i32);
        self.global71 = var1;
        let var2 = self.wasm_sound_registers_getChannelStartingVolume(imports, 2i32);
        self.global72 = var2;
        let var3 = self.wasm_sound_registers_isChannelDacEnabled(imports, 2i32);
        if (var3 == 0) as i32 != 0 {
            self.global61 = 0i32;
        }
    }
    fn wasm_sound_channel3_Channel3_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global63 = 1i32;
        let var0 = self.global62;
        if (var0 == 0) as i32 != 0 {
            self.global62 = 256i32;
        }
        self.wasm_sound_channel3_Channel3_resetTimer(imports);
        self.global88 = 0i32;
        let var1 = self.wasm_sound_registers_isChannelDacEnabled(imports, 3i32);
        if (var1 == 0) as i32 != 0 {
            self.global63 = 0i32;
        }
    }
    fn wasm_sound_channel4_Channel4_trigger<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global65 = 1i32;
        let var0 = self.global64;
        if (var0 == 0) as i32 != 0 {
            self.global64 = 64i32;
        }
        let var1 = self.wasm_sound_channel4_Channel4_getNoiseChannelFrequencyPeriod(imports);
        self.global82 = var1;
        let var2 = self.wasm_sound_envelope_getChannelEnvelopePeriod(imports, 4i32);
        self.global73 = var2;
        let var3 = self.wasm_sound_registers_getChannelStartingVolume(imports, 4i32);
        self.global74 = var3;
        self.global91 = 32767i32;
        let var4 = self.wasm_sound_registers_isChannelDacEnabled(imports, 4i32);
        if (var4 == 0) as i32 != 0 {
            self.global65 = 0i32;
        }
    }
    fn wasm_sound_registers_handledWriteToSoundRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65318i32);
        var3 = var4;
        var2 = (var0 != 65318i32) as i32;
        let var5: i32;
        if var2 != 0 {
            let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var3);
            var5 = (var6 == 0) as i32;
        } else {
            var5 = var2;
        }
        if var5 & 1i32 != 0 {
            return 1i32;
        }
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        var2 = var0;
                        if (var2 != 65297i32) as i32 != 0 {
                            if (var2 == 65302i32) as i32 != 0 {
                                break 'label3;
                            }
                            if (var2 == 65307i32) as i32 != 0 {
                                break 'label2;
                            }
                            if (var2 == 65312i32) as i32 != 0 {
                                break 'label1;
                            }
                            break 'label0;
                        }
                        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                        self.wasm_sound_length_setChannelLengthCounter(imports, 1i32);
                        return 1i32;
                        break;
                    }
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                    self.wasm_sound_length_setChannelLengthCounter(imports, 2i32);
                    return 1i32;
                    break;
                }
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                self.wasm_sound_length_setChannelLengthCounter(imports, 3i32);
                return 1i32;
                break;
            }
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
            self.wasm_sound_length_setChannelLengthCounter(imports, 4i32);
            return 1i32;
            break;
        }
        if (var0 == 65308i32) as i32 != 0 {
            self.global34 = 1i32;
        }
        var2 = (var0 == 65300i32) as i32;
        let var7: i32;
        if var2 != 0 {
            let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1 & 255i32);
            var7 = var8;
        } else {
            var7 = var2;
        }
        if var7 & 1i32 != 0 {
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
            self.wasm_sound_channel1_Channel1_trigger(imports);
            return 1i32;
        } else {
            var2 = (var0 == 65305i32) as i32;
            let var9: i32;
            if var2 != 0 {
                let var10 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1 & 255i32);
                var9 = var10;
            } else {
                var9 = var2;
            }
            if var9 & 1i32 != 0 {
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                self.wasm_sound_channel2_Channel2_trigger(imports);
                return 1i32;
            } else {
                var2 = (var0 == 65310i32) as i32;
                let var11: i32;
                if var2 != 0 {
                    let var12 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1 & 255i32);
                    var11 = var12;
                } else {
                    var11 = var2;
                }
                if var11 & 1i32 != 0 {
                    self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                    self.wasm_sound_channel3_Channel3_trigger(imports);
                    return 1i32;
                } else {
                    var2 = (var0 == 65315i32) as i32;
                    let var13: i32;
                    if var2 != 0 {
                        let var14 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1 & 255i32);
                        var13 = var14;
                    } else {
                        var13 = var2;
                    }
                    if var13 & 1i32 != 0 {
                        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
                        self.wasm_sound_channel4_Channel4_trigger(imports);
                        return 1i32;
                    }
                }
            }
        }
        if (var0 == 65316i32) as i32 != 0 {
            self.global35 = 1i32;
        }
        if (var0 == 65316i32) as i32 != 0 {
            self.global36 = 1i32;
        }
        if (var0 == 65318i32) as i32 != 0 {
            let var15 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1 & 255i32);
            if (var15 == 0) as i32 != 0 {
                var2 = 65296i32;
                'label4: loop {
                    if ((var2 as u32) < 65318i32 as u32) as i32 != 0 {
                        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var2, 0i32);
                        var2 = var2.wrapping_add(1i32) & 65535i32;
                        continue 'label4;
                    }
                    break;
                }
            }
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, var1 & 255i32);
            return 1i32;
        }
        0i32
    }
    fn wasm_memory_dma_startDmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        var0 = var0.wrapping_shl(8i32 as u32) & 65535i32;
        'label0: loop {
            if (var1 as u32 <= 159i32 as u32) as i32 != 0 {
                let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var0.wrapping_add(var1) & 65535i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var1.wrapping_add(65024i32) & 65535i32, var2);
                var1 = var1.wrapping_add(1i32) & 65535i32;
                continue 'label0;
            }
            break;
        }
        self.global105 = 644i32;
    }
    fn wasm_memory_dma_getHdmaSource<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global112;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var1);
        let var3 = self.global113;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3);
        let var5 = self.wasm_helpers_index_concatenateBytes(imports, var2, var4);
        var0 = var5 & 65520i32;
        self.global110 = var0;
        var0
    }
    fn wasm_memory_dma_getHdmaDestination<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global114;
        let var2 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var1);
        let var3 = self.global115;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3);
        let var5 = self.wasm_helpers_index_concatenateBytes(imports, var2, var4);
        var0 = (var5 & 8176i32).wrapping_add(32768i32) & 65535i32;
        self.global111 = var0;
        var0
    }
    fn wasm_memory_dma_hdmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        'label0: loop {
            if ((var3 as u32) < (var2 & 65535i32) as u32) as i32 != 0 {
                let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0.wrapping_add(var3) & 65535i32);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var1.wrapping_add(var3) & 65535i32, var4);
                var3 = var3.wrapping_add(1i32) & 65535i32;
                continue 'label0;
            }
            break;
        }
        var0 = 32i32;
        let var5 = self.global43;
        if var5 != 0 {
            var0 = 64i32;
        }
        let var6 = self.global105;
        self.global105 = var6.wrapping_add(var0.wrapping_mul(var2 / 16i32));
    }
    fn wasm_memory_dma_startHdmaTransfer<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global16;
        if (var4 == 0) as i32 != 0 {
            return;
        }
        let var5 = self.global107;
        let var6: i32;
        if var5 != 0 {
            let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
            var6 = (var7 == 0) as i32;
        } else {
            let var8 = self.global107;
            var6 = var8;
        }
        if var6 & 1i32 != 0 {
            self.global107 = 0i32;
            self.global108 = 0i32;
            self.global109 = 0i32;
            self.global110 = 0i32;
            self.global111 = 0i32;
            let var9 = self.global106;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var9, 255i32);
            return;
        }
        let var10 = self.wasm_memory_dma_getHdmaSource(imports);
        var1 = var10;
        let var11 = self.wasm_memory_dma_getHdmaDestination(imports);
        var2 = var11;
        let var12 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, var0);
        var3 = var12.wrapping_add(1i32).wrapping_mul(16i32);
        let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        if var13 != 0 {
            self.global107 = 1i32;
            self.global108 = 0i32;
            self.global109 = var3;
            self.global110 = var1;
            self.global111 = var2;
            let var14 = self.global106;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var14, var0);
        } else {
            self.wasm_memory_dma_hdmaTransfer(imports, var1, var2, var3);
            let var15 = self.global106;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var15, 255i32);
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
            let var3 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, var0.wrapping_add(1i32) & 255i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var1, var3);
        }
    }
    fn wasm_graphics_palette_writeColorPaletteToMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global118;
        var2 = (var0 == var4) as i32;
        let var5: i32;
        if var2 != 0 {
            var5 = var2;
        } else {
            let var6 = self.global117;
            var5 = (var0 == var6) as i32;
        }
        if var5 & 1i32 != 0 {
            let var7 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var0.wrapping_sub(1i32) & 65535i32);
            let var8 = self.wasm_helpers_index_resetBitOnByte(imports, 6i32, var7);
            var2 = var8;
            let var9 = self.global117;
            if (var0 == var9) as i32 != 0 {
                var3 = 1i32;
            }
            self.wasm_memory_memory_storePaletteByteInWasmMemory(imports, var2, var1 & 255i32, var3);
            self.wasm_graphics_palette_incrementPaletteIndexIfSet(imports, var2, var0.wrapping_sub(1i32) & 65535i32);
        }
    }
    fn wasm_memory_writeTraps_checkWriteTraps<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        var3 = 32768i32;
        if ((var0 as u32) < var3 as u32) as i32 != 0 {
            self.wasm_memory_banking_handleBanking(imports, var0, var1);
            return 0i32;
        }
        var3 = (var0 as u32 >= 32768i32 as u32) as i32;
        let var5: i32;
        if var3 != 0 {
            var5 = ((var0 as u32) < 40960i32 as u32) as i32;
        } else {
            var5 = var3;
        }
        if var5 & 1i32 != 0 {
            return 1i32;
        }
        var4 = 65024i32;
        var3 = (var0 as u32 >= 57344i32 as u32) as i32;
        let var6: i32;
        if var3 != 0 {
            var6 = ((var0 as u32) < 65024i32 as u32) as i32;
        } else {
            var6 = var3;
        }
        if var6 & 1i32 != 0 {
            var3 = var0.wrapping_sub(8192i32) & 65535i32;
            if var2 != 0 {
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var3, var1 & 255i32);
            } else {
                self.wasm_memory_store_sixteenBitStoreIntoGBMemorySkipTraps(imports, var3, var1);
            }
            return 1i32;
        }
        var3 = (var0 as u32 >= 65024i32 as u32) as i32;
        let var7: i32;
        if var3 != 0 {
            var7 = (var0 as u32 <= 65183i32 as u32) as i32;
        } else {
            var7 = var3;
        }
        if var7 & 1i32 != 0 {
            let var8 = self.global46;
            if ((var8 as u32) < 2i32 as u32) as i32 != 0 {
                return 0i32;
            }
            return 1i32;
        }
        var3 = (var0 as u32 >= 65184i32 as u32) as i32;
        let var9: i32;
        if var3 != 0 {
            var9 = (var0 as u32 <= 65279i32 as u32) as i32;
        } else {
            var9 = var3;
        }
        if var9 & 1i32 != 0 {
            return 0i32;
        }
        var3 = (var0 as u32 >= 65284i32 as u32) as i32;
        let var10: i32;
        if var3 != 0 {
            var10 = (var0 as u32 <= 65287i32 as u32) as i32;
        } else {
            var10 = var3;
        }
        if var10 & 1i32 != 0 {
            self.wasm_timers_index_batchProcessTimers(imports);
            if (var0 == 65284i32) as i32 != 0 {
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, 0i32);
                return 0i32;
            }
            if (var0 == 65287i32) as i32 != 0 {
                self.wasm_timers_index_handleTIMCWrite(imports, var1 & 255i32);
                return 1i32;
            }
            return 1i32;
        }
        var3 = (var0 as u32 >= 65296i32 as u32) as i32;
        let var11: i32;
        if var3 != 0 {
            var11 = (var0 as u32 <= 65318i32 as u32) as i32;
        } else {
            var11 = var3;
        }
        if var11 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
            let var12 = self.wasm_sound_registers_handledWriteToSoundRegister(imports, var0, var1);
            if var12 != 0 {
                return 0i32;
            }
        }
        var3 = (var0 as u32 >= 65328i32 as u32) as i32;
        let var13: i32;
        if var3 != 0 {
            var13 = (var0 as u32 <= 65343i32 as u32) as i32;
        } else {
            var13 = var3;
        }
        if var13 & 1i32 != 0 {
            self.wasm_sound_sound_batchProcessAudio(imports);
        }
        var3 = (var0 as u32 >= 65344i32 as u32) as i32;
        let var14: i32;
        if var3 != 0 {
            var14 = (var0 as u32 <= 65355i32 as u32) as i32;
        } else {
            var14 = var3;
        }
        if var14 & 1i32 != 0 {
            if (var0 == 65348i32) as i32 != 0 {
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var0, 0i32);
                return 0i32;
            }
            if (var0 == 65350i32) as i32 != 0 {
                self.wasm_memory_dma_startDmaTransfer(imports, var1 & 255i32);
                return 1i32;
            }
            return 1i32;
        }
        let var15 = self.global106;
        if (var0 == var15) as i32 != 0 {
            self.wasm_memory_dma_startHdmaTransfer(imports, var1 & 255i32);
            return 0i32;
        }
        let var16 = self.global19;
        var3 = (var0 == var16) as i32;
        let var17: i32;
        if var3 != 0 {
            var17 = var3;
        } else {
            let var18 = self.global17;
            var17 = (var0 == var18) as i32;
        }
        if var17 & 1i32 != 0 {
            let var19 = self.global107;
            if var19 != 0 {
                let var20 = self.global110;
                var3 = (var20 as u32 >= 16384i32 as u32) as i32;
                let var21: i32;
                if var3 != 0 {
                    let var22 = self.global110;
                    var21 = (var22 as u32 <= 32767i32 as u32) as i32;
                } else {
                    var21 = var3;
                }
                var3 = var21 & 1i32;
                let var23: i32;
                if var3 != 0 {
                    var23 = var3;
                } else {
                    let var24 = self.global110;
                    var3 = (var24 as u32 >= 53248i32 as u32) as i32;
                    let var25: i32;
                    if var3 != 0 {
                        let var26 = self.global110;
                        var25 = (var26 as u32 <= 57343i32 as u32) as i32;
                    } else {
                        var25 = var3;
                    }
                    var23 = var25 & 1i32;
                }
                if var23 & 1i32 != 0 {
                    return 0i32;
                }
            }
        }
        let var27 = self.global116;
        var3 = (var0 as u32 >= var27 as u32) as i32;
        let var28: i32;
        if var3 != 0 {
            let var29 = self.global117;
            var28 = (var0 as u32 <= var29 as u32) as i32;
        } else {
            var28 = var3;
        }
        if var28 & 1i32 != 0 {
            self.wasm_graphics_palette_writeColorPaletteToMemory(imports, var0, var1);
            return 1i32;
        }
        1i32
    }
    fn wasm_memory_store_eightBitStoreIntoGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var1, 1i32);
        if var2 != 0 {
            self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0, var1);
        }
    }
    fn wasm_cpu_flags_setFlagBit<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        var2 = 1i32.wrapping_shl(var0 as u32) & 255i32;
        if (var1 as u32 > 0i32 as u32) as i32 != 0 {
            let var3 = self.global21;
            self.global21 = (var3 | var2) & 255i32;
        } else {
            let var4 = self.global21;
            self.global21 = var4 & (var2 ^ 255i32);
        }
        let var5 = self.global21;
        var5
    }
    fn wasm_cpu_flags_setHalfCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 5i32, var0);
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
            if (((if (var2 > 0i32) as i32 != 0 { var2 } else { 0i32.wrapping_sub(var2) }) & 15i32) as u32 > (var0 & 15i32) as u32) as i32 != 0 {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            }
        }
    }
    fn wasm_cpu_flags_setZeroFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 7i32, var0);
    }
    fn wasm_cpu_flags_setSubtractFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 6i32, var0);
    }
    fn wasm_cpu_flags_setCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.wasm_cpu_flags_setFlagBit(imports, 4i32, var0);
    }
    fn wasm_helpers_index_rotateByteLeft<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        (var0.wrapping_shl(1i32 as u32) & 255i32 | (var0 as u32).wrapping_shr(7i32 as u32) as i32) & 255i32
    }
    fn wasm_memory_store_sixteenBitStoreIntoGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.wasm_helpers_index_splitHighByte(imports, var1);
        var2 = var3;
        let var4 = self.wasm_helpers_index_splitLowByte(imports, var1);
        var1 = var4;
        let var5 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var1, 0i32);
        if var5 != 0 {
            self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0, var1);
        }
        var0 = var0.wrapping_add(1i32) & 65535i32;
        let var6 = self.wasm_memory_writeTraps_checkWriteTraps(imports, var0, var2, 0i32);
        if var6 != 0 {
            self.wasm_memory_store__eightBitStoreIntoWasmBoyMemory(imports, var0, var2);
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
    fn wasm_cpu_opcodes_handleOpcode0x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                    let var4 = self.wasm_helpers_index_splitHighByte(imports, var3);
                                                                    self.global22 = var4;
                                                                    let var5 = self.wasm_helpers_index_splitLowByte(imports, var3);
                                                                    self.global23 = var5;
                                                                    let var6 = self.global28;
                                                                    self.global28 = var6.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global22;
                                                                let var8 = self.global23;
                                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                                let var10 = self.global20;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var9, var10);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var11 = self.global22;
                                                            let var12 = self.global23;
                                                            let var13 = self.wasm_helpers_index_concatenateBytes(imports, var11, var12);
                                                            var0 = var13.wrapping_add(1i32) & 65535i32;
                                                            let var14 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global22 = var14;
                                                            let var15 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global23 = var15;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var16 = self.global22;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var16, 1i32);
                                                        let var17 = self.global22;
                                                        self.global22 = var17.wrapping_add(1i32) & 255i32;
                                                        let var18 = self.global22;
                                                        if var18 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var19 = self.global22;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var19, -1i32);
                                                    let var20 = self.global22;
                                                    self.global22 = var20.wrapping_sub(1i32) & 255i32;
                                                    let var21 = self.global22;
                                                    if var21 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                self.global22 = var1;
                                                let var22 = self.global28;
                                                self.global28 = var22.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var23 = self.global20;
                                            if (var23 & 128i32 == 128i32) as i32 != 0 {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            } else {
                                                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
                                            }
                                            let var24 = self.global20;
                                            let var25 = self.wasm_helpers_index_rotateByteLeft(imports, var24);
                                            self.global20 = var25;
                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            return 4i32;
                                            break;
                                        }
                                        let var26 = self.global29;
                                        self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var3, var26);
                                        let var27 = self.global28;
                                        self.global28 = var27.wrapping_add(2i32) & 65535i32;
                                        return 20i32;
                                        break;
                                    }
                                    let var28 = self.global26;
                                    let var29 = self.global27;
                                    let var30 = self.wasm_helpers_index_concatenateBytes(imports, var28, var29);
                                    var0 = var30;
                                    let var31 = self.global22;
                                    let var32 = self.global23;
                                    let var33 = self.wasm_helpers_index_concatenateBytes(imports, var31, var32);
                                    var1 = var33;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var0, var1, 0i32);
                                    var0 = var0.wrapping_add(var1) & 65535i32;
                                    let var34 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                    self.global26 = var34;
                                    let var35 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                    self.global27 = var35;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var36 = self.global22;
                                let var37 = self.global23;
                                let var38 = self.wasm_helpers_index_concatenateBytes(imports, var36, var37);
                                let var39 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var38);
                                self.global20 = var39;
                                return 8i32;
                                break;
                            }
                            let var40 = self.global22;
                            let var41 = self.global23;
                            let var42 = self.wasm_helpers_index_concatenateBytes(imports, var40, var41);
                            var0 = var42.wrapping_sub(1i32) & 65535i32;
                            let var43 = self.wasm_helpers_index_splitHighByte(imports, var0);
                            self.global22 = var43;
                            let var44 = self.wasm_helpers_index_splitLowByte(imports, var0);
                            self.global23 = var44;
                            return 8i32;
                            break;
                        }
                        let var45 = self.global23;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var45, 1i32);
                        let var46 = self.global23;
                        self.global23 = var46.wrapping_add(1i32) & 255i32;
                        let var47 = self.global23;
                        if var47 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var48 = self.global23;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var48, -1i32);
                    let var49 = self.global23;
                    self.global23 = var49.wrapping_sub(1i32) & 255i32;
                    let var50 = self.global23;
                    if var50 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                self.global23 = var1;
                let var51 = self.global28;
                self.global28 = var51.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var52 = self.global20;
            if ((var52 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
            let var53 = self.global20;
            let var54 = self.wasm_helpers_index_rotateByteRight(imports, var53);
            self.global20 = var54;
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_flags_getCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global21;
        (var0 as u32).wrapping_shr(4i32 as u32) as i32 & 1i32
    }
    fn wasm_helpers_index_rotateByteLeftThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_cpu_flags_getCarryFlag(imports);
        (var0.wrapping_shl(1i32 as u32) & 255i32 | var1) & 255i32
    }
    fn wasm_cpu_instructions_relativeJump<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global28;
        self.global28 = var1.wrapping_add(var0.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32)) & 65535i32;
        let var2 = self.global28;
        self.global28 = var2.wrapping_add(1i32) & 65535i32;
    }
    fn wasm_helpers_index_rotateByteRightThroughCarry<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.wasm_cpu_flags_getCarryFlag(imports);
        ((var0 as u32).wrapping_shr(1i32 as u32) as i32 | var1.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    fn wasm_cpu_opcodes_handleOpcode1x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global16;
                                                                        if var4 != 0 {
                                                                            let var5 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, 65357i32);
                                                                            var0 = var5;
                                                                            let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var0);
                                                                            if var6 != 0 {
                                                                                let var7 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var0);
                                                                                var0 = var7;
                                                                                let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
                                                                                let var9: i32;
                                                                                if var8 != 0 {
                                                                                    self.global43 = 0i32;
                                                                                    let var10 = self.wasm_helpers_index_resetBitOnByte(imports, 7i32, var0);
                                                                                    var9 = var10;
                                                                                } else {
                                                                                    self.global43 = 1i32;
                                                                                    let var11 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, var0);
                                                                                    var9 = var11;
                                                                                }
                                                                                var0 = var9;
                                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, 65357i32, var0);
                                                                                return 76i32;
                                                                            }
                                                                        }
                                                                        let var12 = self.global28;
                                                                        self.global28 = var12.wrapping_add(1i32) & 65535i32;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var13 = self.wasm_helpers_index_splitHighByte(imports, var3);
                                                                    self.global24 = var13;
                                                                    let var14 = self.wasm_helpers_index_splitLowByte(imports, var3);
                                                                    self.global25 = var14;
                                                                    let var15 = self.global28;
                                                                    self.global28 = var15.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var16 = self.global24;
                                                                let var17 = self.global25;
                                                                let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                                                                let var19 = self.global20;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var18, var19);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var20 = self.global24;
                                                            let var21 = self.global25;
                                                            let var22 = self.wasm_helpers_index_concatenateBytes(imports, var20, var21);
                                                            var0 = var22.wrapping_add(1i32) & 65535i32;
                                                            let var23 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global24 = var23;
                                                            let var24 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global25 = var24;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var25 = self.global24;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var25, 1i32);
                                                        let var26 = self.global24;
                                                        self.global24 = var26.wrapping_add(1i32) & 255i32;
                                                        let var27 = self.global24;
                                                        if var27 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var28 = self.global24;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var28, -1i32);
                                                    let var29 = self.global24;
                                                    self.global24 = var29.wrapping_sub(1i32) & 255i32;
                                                    let var30 = self.global24;
                                                    if var30 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                self.global24 = var1;
                                                let var31 = self.global28;
                                                self.global28 = var31.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            var0 = 0i32;
                                            let var32 = self.global20;
                                            if (var32 & 128i32 == 128i32) as i32 != 0 {
                                                var0 = 1i32;
                                            }
                                            let var33 = self.global20;
                                            let var34 = self.wasm_helpers_index_rotateByteLeftThroughCarry(imports, var33);
                                            self.global20 = var34;
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
                                        self.wasm_cpu_instructions_relativeJump(imports, var1);
                                        return 12i32;
                                        break;
                                    }
                                    let var35 = self.global26;
                                    let var36 = self.global27;
                                    let var37 = self.wasm_helpers_index_concatenateBytes(imports, var35, var36);
                                    var0 = var37;
                                    let var38 = self.global24;
                                    let var39 = self.global25;
                                    let var40 = self.wasm_helpers_index_concatenateBytes(imports, var38, var39);
                                    var1 = var40;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var0, var1, 0i32);
                                    var0 = var0.wrapping_add(var1) & 65535i32;
                                    let var41 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                    self.global26 = var41;
                                    let var42 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                    self.global27 = var42;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var43 = self.global24;
                                let var44 = self.global25;
                                let var45 = self.wasm_helpers_index_concatenateBytes(imports, var43, var44);
                                let var46 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var45);
                                self.global20 = var46;
                                return 8i32;
                                break;
                            }
                            let var47 = self.global24;
                            let var48 = self.global25;
                            let var49 = self.wasm_helpers_index_concatenateBytes(imports, var47, var48);
                            var0 = var49.wrapping_sub(1i32) & 65535i32;
                            let var50 = self.wasm_helpers_index_splitHighByte(imports, var0);
                            self.global24 = var50;
                            let var51 = self.wasm_helpers_index_splitLowByte(imports, var0);
                            self.global25 = var51;
                            return 8i32;
                            break;
                        }
                        let var52 = self.global25;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var52, 1i32);
                        let var53 = self.global25;
                        self.global25 = var53.wrapping_add(1i32) & 255i32;
                        let var54 = self.global25;
                        if var54 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var55 = self.global25;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var55, -1i32);
                    let var56 = self.global25;
                    self.global25 = var56.wrapping_sub(1i32) & 255i32;
                    let var57 = self.global25;
                    if var57 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                self.global25 = var1;
                let var58 = self.global28;
                self.global28 = var58.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            var0 = 0i32;
            let var59 = self.global20;
            if (var59 & 1i32 == 1i32) as i32 != 0 {
                var0 = 1i32;
            }
            let var60 = self.global20;
            let var61 = self.wasm_helpers_index_rotateByteRightThroughCarry(imports, var60);
            self.global20 = var61;
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
        let var0 = self.global21;
        (var0 as u32).wrapping_shr(7i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_flags_getHalfCarryFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global21;
        (var0 as u32).wrapping_shr(5i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_flags_getSubtractFlag<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global21;
        (var0 as u32).wrapping_shr(6i32 as u32) as i32 & 1i32
    }
    fn wasm_cpu_opcodes_handleOpcode2x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                        if var4 != 0 {
                                                                            let var5 = self.global28;
                                                                            self.global28 = var5.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            self.wasm_cpu_instructions_relativeJump(imports, var1);
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    var0 = var3;
                                                                    let var6 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                    self.global26 = var6;
                                                                    let var7 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                    self.global27 = var7;
                                                                    let var8 = self.global28;
                                                                    self.global28 = var8.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var9 = self.global26;
                                                                let var10 = self.global27;
                                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                                var0 = var11;
                                                                let var12 = self.global20;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var12);
                                                                var0 = var0.wrapping_add(1i32) & 65535i32;
                                                                let var13 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                self.global26 = var13;
                                                                let var14 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                self.global27 = var14;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var15 = self.global26;
                                                            let var16 = self.global27;
                                                            let var17 = self.wasm_helpers_index_concatenateBytes(imports, var15, var16);
                                                            var0 = var17.wrapping_add(1i32) & 65535i32;
                                                            let var18 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                            self.global26 = var18;
                                                            let var19 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                            self.global27 = var19;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var20 = self.global26;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var20, 1i32);
                                                        let var21 = self.global26;
                                                        self.global26 = var21.wrapping_add(1i32) & 255i32;
                                                        let var22 = self.global26;
                                                        if var22 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var23 = self.global26;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var23, -1i32);
                                                    let var24 = self.global26;
                                                    self.global26 = var24.wrapping_sub(1i32) & 255i32;
                                                    let var25 = self.global26;
                                                    if var25 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    return 4i32;
                                                    break;
                                                }
                                                self.global26 = var1;
                                                let var26 = self.global28;
                                                self.global28 = var26.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            var1 = 0i32;
                                            let var27 = self.wasm_cpu_flags_getHalfCarryFlag(imports);
                                            if (var27 as u32 > 0i32 as u32) as i32 != 0 {
                                                var1 = 6i32;
                                            }
                                            let var28 = self.wasm_cpu_flags_getCarryFlag(imports);
                                            if (var28 as u32 > 0i32 as u32) as i32 != 0 {
                                                var1 = (var1 | 96i32) & 255i32;
                                            }
                                            let var29 = self.wasm_cpu_flags_getSubtractFlag(imports);
                                            let var30: i32;
                                            if (var29 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var31 = self.global20;
                                                var30 = var31.wrapping_sub(var1) & 255i32;
                                            } else {
                                                let var32 = self.global20;
                                                if ((var32 & 15i32) as u32 > 9i32 as u32) as i32 != 0 {
                                                    var1 = (var1 | 6i32) & 255i32;
                                                }
                                                let var33 = self.global20;
                                                if (var33 as u32 > 153i32 as u32) as i32 != 0 {
                                                    var1 = (var1 | 96i32) & 255i32;
                                                }
                                                let var34 = self.global20;
                                                var30 = var34.wrapping_add(var1) & 255i32;
                                            }
                                            var0 = var30;
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
                                            self.global20 = var0;
                                            return 4i32;
                                            break;
                                        }
                                        let var35 = self.wasm_cpu_flags_getZeroFlag(imports);
                                        if (var35 as u32 > 0i32 as u32) as i32 != 0 {
                                            self.wasm_cpu_instructions_relativeJump(imports, var1);
                                            return 12i32;
                                        } else {
                                            let var36 = self.global28;
                                            self.global28 = var36.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var37 = self.global26;
                                    let var38 = self.global27;
                                    let var39 = self.wasm_helpers_index_concatenateBytes(imports, var37, var38);
                                    var1 = var39;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var1, var1, 0i32);
                                    var1 = var1.wrapping_mul(2i32) & 65535i32;
                                    let var40 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                    self.global26 = var40;
                                    let var41 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                    self.global27 = var41;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var42 = self.global26;
                                let var43 = self.global27;
                                let var44 = self.wasm_helpers_index_concatenateBytes(imports, var42, var43);
                                var1 = var44;
                                let var45 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var1);
                                self.global20 = var45;
                                var1 = var1.wrapping_add(1i32) & 65535i32;
                                let var46 = self.wasm_helpers_index_splitHighByte(imports, var1);
                                self.global26 = var46;
                                let var47 = self.wasm_helpers_index_splitLowByte(imports, var1);
                                self.global27 = var47;
                                return 8i32;
                                break;
                            }
                            let var48 = self.global26;
                            let var49 = self.global27;
                            let var50 = self.wasm_helpers_index_concatenateBytes(imports, var48, var49);
                            var1 = var50.wrapping_sub(1i32) & 65535i32;
                            let var51 = self.wasm_helpers_index_splitHighByte(imports, var1);
                            self.global26 = var51;
                            let var52 = self.wasm_helpers_index_splitLowByte(imports, var1);
                            self.global27 = var52;
                            return 8i32;
                            break;
                        }
                        let var53 = self.global27;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var53, 1i32);
                        let var54 = self.global27;
                        self.global27 = var54.wrapping_add(1i32) & 255i32;
                        let var55 = self.global27;
                        if var55 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var56 = self.global27;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var56, -1i32);
                    let var57 = self.global27;
                    self.global27 = var57.wrapping_sub(1i32) & 255i32;
                    let var58 = self.global27;
                    if var58 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                self.global27 = var1;
                let var59 = self.global28;
                self.global28 = var59.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var60 = self.global20;
            self.global20 = (var60 ^ -1i32) & 255i32;
            self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode3x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                                        if var4 != 0 {
                                                                            let var5 = self.global28;
                                                                            self.global28 = var5.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            self.wasm_cpu_instructions_relativeJump(imports, var1);
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    self.global29 = var3;
                                                                    let var6 = self.global28;
                                                                    self.global28 = var6.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global26;
                                                                let var8 = self.global27;
                                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                                var0 = var9;
                                                                let var10 = self.global20;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var10);
                                                                var0 = var0.wrapping_sub(1i32) & 65535i32;
                                                                let var11 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                                self.global26 = var11;
                                                                let var12 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                                self.global27 = var12;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var13 = self.global29;
                                                            self.global29 = var13.wrapping_add(1i32) & 65535i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var14 = self.global26;
                                                        let var15 = self.global27;
                                                        let var16 = self.wasm_helpers_index_concatenateBytes(imports, var14, var15);
                                                        var0 = var16;
                                                        let var17 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var0);
                                                        var1 = var17;
                                                        var2 = 1i32;
                                                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, var2);
                                                        var1 = var1.wrapping_add(1i32) & 255i32;
                                                        if var1 != 0 {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                        } else {
                                                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                        }
                                                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                                        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var0, var1);
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var18 = self.global26;
                                                    let var19 = self.global27;
                                                    let var20 = self.wasm_helpers_index_concatenateBytes(imports, var18, var19);
                                                    var2 = var20;
                                                    let var21 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var2);
                                                    var1 = var21;
                                                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, -1i32);
                                                    var1 = var1.wrapping_sub(1i32) & 255i32;
                                                    if var1 != 0 {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                                    } else {
                                                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                                                    }
                                                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var2, var1);
                                                    return 12i32;
                                                    break;
                                                }
                                                let var22 = self.global26;
                                                let var23 = self.global27;
                                                let var24 = self.wasm_helpers_index_concatenateBytes(imports, var22, var23);
                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var24, var1);
                                                let var25 = self.global28;
                                                self.global28 = var25.wrapping_add(1i32) & 65535i32;
                                                return 12i32;
                                                break;
                                            }
                                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
                                            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
                                            return 4i32;
                                            break;
                                        }
                                        let var26 = self.wasm_cpu_flags_getCarryFlag(imports);
                                        if (var26 == 1i32) as i32 != 0 {
                                            self.wasm_cpu_instructions_relativeJump(imports, var1);
                                            return 12i32;
                                        } else {
                                            let var27 = self.global28;
                                            self.global28 = var27.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var28 = self.global26;
                                    let var29 = self.global27;
                                    let var30 = self.wasm_helpers_index_concatenateBytes(imports, var28, var29);
                                    var1 = var30;
                                    let var31 = self.global29;
                                    self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var1, var31, 0i32);
                                    let var32 = self.global29;
                                    var2 = var1.wrapping_add(var32) & 65535i32;
                                    let var33 = self.wasm_helpers_index_splitHighByte(imports, var2);
                                    self.global26 = var33;
                                    let var34 = self.wasm_helpers_index_splitLowByte(imports, var2);
                                    self.global27 = var34;
                                    self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                    return 8i32;
                                    break;
                                }
                                let var35 = self.global26;
                                let var36 = self.global27;
                                let var37 = self.wasm_helpers_index_concatenateBytes(imports, var35, var36);
                                var2 = var37;
                                let var38 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var2);
                                self.global20 = var38;
                                var2 = var2.wrapping_sub(1i32) & 65535i32;
                                let var39 = self.wasm_helpers_index_splitHighByte(imports, var2);
                                self.global26 = var39;
                                let var40 = self.wasm_helpers_index_splitLowByte(imports, var2);
                                self.global27 = var40;
                                return 8i32;
                                break;
                            }
                            let var41 = self.global29;
                            self.global29 = var41.wrapping_sub(1i32) & 65535i32;
                            return 8i32;
                            break;
                        }
                        let var42 = self.global20;
                        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var42, 1i32);
                        let var43 = self.global20;
                        self.global20 = var43.wrapping_add(1i32) & 255i32;
                        let var44 = self.global20;
                        if var44 != 0 {
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                        } else {
                            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                        }
                        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                        return 4i32;
                        break;
                    }
                    let var45 = self.global20;
                    self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var45, -1i32);
                    let var46 = self.global20;
                    self.global20 = var46.wrapping_sub(1i32) & 255i32;
                    let var47 = self.global20;
                    if var47 != 0 {
                        self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                    } else {
                        self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
                    }
                    self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
                    return 4i32;
                    break;
                }
                self.global20 = var1;
                let var48 = self.global28;
                self.global28 = var48.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
            let var49 = self.wasm_cpu_flags_getCarryFlag(imports);
            if (var49 as u32 > 0i32 as u32) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            }
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode4x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                    let var4 = self.global23;
                                                                    self.global22 = var4;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var5 = self.global24;
                                                                self.global22 = var5;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var6 = self.global25;
                                                            self.global22 = var6;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var7 = self.global26;
                                                        self.global22 = var7;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var8 = self.global27;
                                                    self.global22 = var8;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var9 = self.global26;
                                                let var10 = self.global27;
                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                let var12 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var11);
                                                self.global22 = var12;
                                                return 8i32;
                                                break;
                                            }
                                            let var13 = self.global20;
                                            self.global22 = var13;
                                            return 4i32;
                                            break;
                                        }
                                        let var14 = self.global22;
                                        self.global23 = var14;
                                        return 4i32;
                                        break;
                                    }
                                    return 4i32;
                                    break;
                                }
                                let var15 = self.global24;
                                self.global23 = var15;
                                return 4i32;
                                break;
                            }
                            let var16 = self.global25;
                            self.global23 = var16;
                            return 4i32;
                            break;
                        }
                        let var17 = self.global26;
                        self.global23 = var17;
                        return 4i32;
                        break;
                    }
                    let var18 = self.global27;
                    self.global23 = var18;
                    return 4i32;
                    break;
                }
                let var19 = self.global26;
                let var20 = self.global27;
                let var21 = self.wasm_helpers_index_concatenateBytes(imports, var19, var20);
                let var22 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var21);
                self.global23 = var22;
                return 8i32;
                break;
            }
            let var23 = self.global20;
            self.global23 = var23;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode5x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.global24 = var4;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.global24 = var5;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var6 = self.global25;
                                                            self.global24 = var6;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var7 = self.global26;
                                                        self.global24 = var7;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var8 = self.global27;
                                                    self.global24 = var8;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var9 = self.global26;
                                                let var10 = self.global27;
                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                let var12 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var11);
                                                self.global24 = var12;
                                                return 8i32;
                                                break;
                                            }
                                            let var13 = self.global20;
                                            self.global24 = var13;
                                            return 4i32;
                                            break;
                                        }
                                        let var14 = self.global22;
                                        self.global25 = var14;
                                        return 4i32;
                                        break;
                                    }
                                    let var15 = self.global23;
                                    self.global25 = var15;
                                    return 4i32;
                                    break;
                                }
                                let var16 = self.global24;
                                self.global25 = var16;
                                return 4i32;
                                break;
                            }
                            return 4i32;
                            break;
                        }
                        let var17 = self.global26;
                        self.global25 = var17;
                        return 4i32;
                        break;
                    }
                    let var18 = self.global27;
                    self.global25 = var18;
                    return 4i32;
                    break;
                }
                let var19 = self.global26;
                let var20 = self.global27;
                let var21 = self.wasm_helpers_index_concatenateBytes(imports, var19, var20);
                let var22 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var21);
                self.global25 = var22;
                return 4i32;
                break;
            }
            let var23 = self.global20;
            self.global25 = var23;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode6x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.global26 = var4;
                                                                        return 8i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.global26 = var5;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global24;
                                                                self.global26 = var6;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global25;
                                                            self.global26 = var7;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var8 = self.global27;
                                                    self.global26 = var8;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var9 = self.global26;
                                                let var10 = self.global27;
                                                let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                let var12 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var11);
                                                self.global26 = var12;
                                                return 8i32;
                                                break;
                                            }
                                            let var13 = self.global20;
                                            self.global26 = var13;
                                            return 4i32;
                                            break;
                                        }
                                        let var14 = self.global22;
                                        self.global27 = var14;
                                        return 4i32;
                                        break;
                                    }
                                    let var15 = self.global23;
                                    self.global27 = var15;
                                    return 4i32;
                                    break;
                                }
                                let var16 = self.global24;
                                self.global27 = var16;
                                return 4i32;
                                break;
                            }
                            let var17 = self.global25;
                            self.global27 = var17;
                            return 4i32;
                            break;
                        }
                        let var18 = self.global26;
                        self.global27 = var18;
                        return 4i32;
                        break;
                    }
                    return 4i32;
                    break;
                }
                let var19 = self.global26;
                let var20 = self.global27;
                let var21 = self.wasm_helpers_index_concatenateBytes(imports, var19, var20);
                let var22 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var21);
                self.global27 = var22;
                return 8i32;
                break;
            }
            let var23 = self.global20;
            self.global27 = var23;
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcode7x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global26;
                                                                        let var5 = self.global27;
                                                                        let var6 = self.wasm_helpers_index_concatenateBytes(imports, var4, var5);
                                                                        let var7 = self.global22;
                                                                        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var6, var7);
                                                                        return 8i32;
                                                                        break;
                                                                    }
                                                                    let var8 = self.global26;
                                                                    let var9 = self.global27;
                                                                    let var10 = self.wasm_helpers_index_concatenateBytes(imports, var8, var9);
                                                                    let var11 = self.global23;
                                                                    self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var10, var11);
                                                                    return 8i32;
                                                                    break;
                                                                }
                                                                let var12 = self.global26;
                                                                let var13 = self.global27;
                                                                let var14 = self.wasm_helpers_index_concatenateBytes(imports, var12, var13);
                                                                let var15 = self.global24;
                                                                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var14, var15);
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var16 = self.global26;
                                                            let var17 = self.global27;
                                                            let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                                                            let var19 = self.global25;
                                                            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var18, var19);
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var20 = self.global26;
                                                        let var21 = self.global27;
                                                        let var22 = self.wasm_helpers_index_concatenateBytes(imports, var20, var21);
                                                        let var23 = self.global26;
                                                        self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var22, var23);
                                                        return 8i32;
                                                        break;
                                                    }
                                                    let var24 = self.global26;
                                                    let var25 = self.global27;
                                                    let var26 = self.wasm_helpers_index_concatenateBytes(imports, var24, var25);
                                                    let var27 = self.global27;
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var26, var27);
                                                    return 8i32;
                                                    break;
                                                }
                                                let var28 = self.global107;
                                                if (var28 == 0) as i32 != 0 {
                                                    self.global44 = 1i32;
                                                }
                                                return 4i32;
                                                break;
                                            }
                                            let var29 = self.global26;
                                            let var30 = self.global27;
                                            let var31 = self.wasm_helpers_index_concatenateBytes(imports, var29, var30);
                                            let var32 = self.global20;
                                            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var31, var32);
                                            return 8i32;
                                            break;
                                        }
                                        let var33 = self.global22;
                                        self.global20 = var33;
                                        return 4i32;
                                        break;
                                    }
                                    let var34 = self.global23;
                                    self.global20 = var34;
                                    return 4i32;
                                    break;
                                }
                                let var35 = self.global24;
                                self.global20 = var35;
                                return 4i32;
                                break;
                            }
                            let var36 = self.global25;
                            self.global20 = var36;
                            return 4i32;
                            break;
                        }
                        let var37 = self.global26;
                        self.global20 = var37;
                        return 4i32;
                        break;
                    }
                    let var38 = self.global27;
                    self.global20 = var38;
                    return 4i32;
                    break;
                }
                let var39 = self.global26;
                let var40 = self.global27;
                let var41 = self.wasm_helpers_index_concatenateBytes(imports, var39, var40);
                let var42 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var41);
                self.global20 = var42;
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
            if ((if (var2 > 0i32) as i32 != 0 { var2 } else { 0i32.wrapping_sub(var2) }) > var0) as i32 != 0 {
                self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
            } else {
                self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
            }
        }
    }
    fn wasm_cpu_instructions_addARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global20;
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var1, var0);
        let var2 = self.global20;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var2, var0);
        let var3 = self.global20;
        self.global20 = var3.wrapping_add(var0) & 255i32;
        let var4 = self.global20;
        if var4 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
    }
    fn wasm_cpu_instructions_addAThroughCarryRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global20;
        let var3 = self.wasm_cpu_flags_getCarryFlag(imports);
        var1 = var2.wrapping_add(var0).wrapping_add(var3) & 255i32;
        let var4 = self.global20;
        if (var4 ^ var0 ^ var1) & 16i32 != 0 {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        }
        let var5 = self.global20;
        let var6 = self.wasm_cpu_flags_getCarryFlag(imports);
        if ((var5.wrapping_add(var0).wrapping_add(var6) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        self.global20 = var1;
        let var7 = self.global20;
        if var7 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
    }
    fn wasm_cpu_opcodes_handleOpcode8x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.wasm_cpu_instructions_addARegister(imports, var4);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.wasm_cpu_instructions_addARegister(imports, var5);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global24;
                                                                self.wasm_cpu_instructions_addARegister(imports, var6);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global25;
                                                            self.wasm_cpu_instructions_addARegister(imports, var7);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global26;
                                                        self.wasm_cpu_instructions_addARegister(imports, var8);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global27;
                                                    self.wasm_cpu_instructions_addARegister(imports, var9);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global26;
                                                let var11 = self.global27;
                                                let var12 = self.wasm_helpers_index_concatenateBytes(imports, var10, var11);
                                                let var13 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var12);
                                                self.wasm_cpu_instructions_addARegister(imports, var13);
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global20;
                                            self.wasm_cpu_instructions_addARegister(imports, var14);
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global22;
                                        self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var15);
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global23;
                                    self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var16);
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global24;
                                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var17);
                                return 4i32;
                                break;
                            }
                            let var18 = self.global25;
                            self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var18);
                            return 4i32;
                            break;
                        }
                        let var19 = self.global26;
                        self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var19);
                        return 4i32;
                        break;
                    }
                    let var20 = self.global27;
                    self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var20);
                    return 4i32;
                    break;
                }
                let var21 = self.global26;
                let var22 = self.global27;
                let var23 = self.wasm_helpers_index_concatenateBytes(imports, var21, var22);
                let var24 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var23);
                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var24);
                return 8i32;
                break;
            }
            let var25 = self.global20;
            self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var25);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_subARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global20;
        var1 = var0.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var2, var1);
        let var3 = self.global20;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var3, var1);
        let var4 = self.global20;
        self.global20 = var4.wrapping_sub(var0) & 255i32;
        let var5 = self.global20;
        if var5 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_instructions_subAThroughCarryRegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global20;
        let var3 = self.wasm_cpu_flags_getCarryFlag(imports);
        var1 = var2.wrapping_sub(var0).wrapping_sub(var3) & 255i32;
        let var4 = self.global20;
        if (var4 ^ var0 ^ var1) & 16i32 != 0 {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        }
        let var5 = self.global20;
        let var6 = self.wasm_cpu_flags_getCarryFlag(imports);
        if ((var5.wrapping_sub(var0).wrapping_sub(var6) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.wasm_cpu_flags_setCarryFlag(imports, 1i32);
        } else {
            self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
        }
        self.global20 = var1;
        let var7 = self.global20;
        if var7 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_opcodes_handleOpcode9x<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.wasm_cpu_instructions_subARegister(imports, var4);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.wasm_cpu_instructions_subARegister(imports, var5);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global24;
                                                                self.wasm_cpu_instructions_subARegister(imports, var6);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global25;
                                                            self.wasm_cpu_instructions_subARegister(imports, var7);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global26;
                                                        self.wasm_cpu_instructions_subARegister(imports, var8);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global27;
                                                    self.wasm_cpu_instructions_subARegister(imports, var9);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global26;
                                                let var11 = self.global27;
                                                let var12 = self.wasm_helpers_index_concatenateBytes(imports, var10, var11);
                                                let var13 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var12);
                                                self.wasm_cpu_instructions_subARegister(imports, var13);
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global20;
                                            self.wasm_cpu_instructions_subARegister(imports, var14);
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global22;
                                        self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var15);
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global23;
                                    self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var16);
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global24;
                                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var17);
                                return 4i32;
                                break;
                            }
                            let var18 = self.global25;
                            self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var18);
                            return 4i32;
                            break;
                        }
                        let var19 = self.global26;
                        self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var19);
                        return 4i32;
                        break;
                    }
                    let var20 = self.global27;
                    self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var20);
                    return 4i32;
                    break;
                }
                let var21 = self.global26;
                let var22 = self.global27;
                let var23 = self.wasm_helpers_index_concatenateBytes(imports, var21, var22);
                let var24 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var23);
                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var24);
                return 8i32;
                break;
            }
            let var25 = self.global20;
            self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var25);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_andARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global20;
        self.global20 = var1 & var0 & 255i32;
        let var2 = self.global20;
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
        let var1 = self.global20;
        self.global20 = (var1 ^ var0) & 255i32;
        let var2 = self.global20;
        if var2 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
        self.wasm_cpu_flags_setHalfCarryFlag(imports, 0i32);
        self.wasm_cpu_flags_setCarryFlag(imports, 0i32);
    }
    fn wasm_cpu_opcodes_handleOpcodeAx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.wasm_cpu_instructions_andARegister(imports, var4);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.wasm_cpu_instructions_andARegister(imports, var5);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global24;
                                                                self.wasm_cpu_instructions_andARegister(imports, var6);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global25;
                                                            self.wasm_cpu_instructions_andARegister(imports, var7);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global26;
                                                        self.wasm_cpu_instructions_andARegister(imports, var8);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global27;
                                                    self.wasm_cpu_instructions_andARegister(imports, var9);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global26;
                                                let var11 = self.global27;
                                                let var12 = self.wasm_helpers_index_concatenateBytes(imports, var10, var11);
                                                let var13 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var12);
                                                self.wasm_cpu_instructions_andARegister(imports, var13);
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global20;
                                            self.wasm_cpu_instructions_andARegister(imports, var14);
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global22;
                                        self.wasm_cpu_instructions_xorARegister(imports, var15);
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global23;
                                    self.wasm_cpu_instructions_xorARegister(imports, var16);
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global24;
                                self.wasm_cpu_instructions_xorARegister(imports, var17);
                                return 4i32;
                                break;
                            }
                            let var18 = self.global25;
                            self.wasm_cpu_instructions_xorARegister(imports, var18);
                            return 4i32;
                            break;
                        }
                        let var19 = self.global26;
                        self.wasm_cpu_instructions_xorARegister(imports, var19);
                        return 4i32;
                        break;
                    }
                    let var20 = self.global27;
                    self.wasm_cpu_instructions_xorARegister(imports, var20);
                    return 4i32;
                    break;
                }
                let var21 = self.global26;
                let var22 = self.global27;
                let var23 = self.wasm_helpers_index_concatenateBytes(imports, var21, var22);
                let var24 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var23);
                self.wasm_cpu_instructions_xorARegister(imports, var24);
                return 8i32;
                break;
            }
            let var25 = self.global20;
            self.wasm_cpu_instructions_xorARegister(imports, var25);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_instructions_orARegister<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global20;
        self.global20 = (var1 | var0) & 255i32;
        let var2 = self.global20;
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
        let var2 = self.global20;
        var1 = var0.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        self.wasm_cpu_flags_checkAndSetEightBitHalfCarryFlag(imports, var2, var1);
        let var3 = self.global20;
        self.wasm_cpu_flags_checkAndSetEightBitCarryFlag(imports, var3, var1);
        let var4 = self.global20;
        if var4.wrapping_add(var1) & 65535i32 != 0 {
            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
        } else {
            self.wasm_cpu_flags_setZeroFlag(imports, 1i32);
        }
        self.wasm_cpu_flags_setSubtractFlag(imports, 1i32);
    }
    fn wasm_cpu_opcodes_handleOpcodeBx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                        let var4 = self.global22;
                                                                        self.wasm_cpu_instructions_orARegister(imports, var4);
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var5 = self.global23;
                                                                    self.wasm_cpu_instructions_orARegister(imports, var5);
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global24;
                                                                self.wasm_cpu_instructions_orARegister(imports, var6);
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global25;
                                                            self.wasm_cpu_instructions_orARegister(imports, var7);
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global26;
                                                        self.wasm_cpu_instructions_orARegister(imports, var8);
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global27;
                                                    self.wasm_cpu_instructions_orARegister(imports, var9);
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global26;
                                                let var11 = self.global27;
                                                let var12 = self.wasm_helpers_index_concatenateBytes(imports, var10, var11);
                                                let var13 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var12);
                                                self.wasm_cpu_instructions_orARegister(imports, var13);
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global20;
                                            self.wasm_cpu_instructions_orARegister(imports, var14);
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global22;
                                        self.wasm_cpu_instructions_cpARegister(imports, var15);
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global23;
                                    self.wasm_cpu_instructions_cpARegister(imports, var16);
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global24;
                                self.wasm_cpu_instructions_cpARegister(imports, var17);
                                return 4i32;
                                break;
                            }
                            let var18 = self.global25;
                            self.wasm_cpu_instructions_cpARegister(imports, var18);
                            return 4i32;
                            break;
                        }
                        let var19 = self.global26;
                        self.wasm_cpu_instructions_cpARegister(imports, var19);
                        return 4i32;
                        break;
                    }
                    let var20 = self.global27;
                    self.wasm_cpu_instructions_cpARegister(imports, var20);
                    return 4i32;
                    break;
                }
                let var21 = self.global26;
                let var22 = self.global27;
                let var23 = self.wasm_helpers_index_concatenateBytes(imports, var21, var22);
                let var24 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var23);
                self.wasm_cpu_instructions_cpARegister(imports, var24);
                return 8i32;
                break;
            }
            let var25 = self.global20;
            self.wasm_cpu_instructions_cpARegister(imports, var25);
            return 4i32;
            break;
        }
        -1i32
    }
    fn wasm_memory_load_sixteenBitLoadFromGBMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3: i32;
        'label0: loop {
            let var4 = self.wasm_memory_readTraps_checkReadTraps(imports, var0);
            var1 = var4;
            if (var1 == -1i32) as i32 != 0 {
                let var5 = self.wasm_memory_load__eightBitLoadFromWasmBoyMemory(imports, var0);
                var3 = var5;
                break 'label0;
            }
            var3 = var1 & 255i32;
            break;
        }
        var1 = var3;
        let var6: i32;
        'label1: loop {
            var0 = var0.wrapping_add(1i32) & 65535i32;
            let var7 = self.wasm_memory_readTraps_checkReadTraps(imports, var0);
            var2 = var7;
            if (var2 == -1i32) as i32 != 0 {
                let var8 = self.wasm_memory_load__eightBitLoadFromWasmBoyMemory(imports, var0);
                var6 = var8;
                break 'label1;
            }
            var6 = var2 & 255i32;
            break;
        }
        var0 = var6;
        let var9 = self.wasm_helpers_index_concatenateBytes(imports, var0, var1);
        var9
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
        if (var1 as u32 > 0i32 as u32) as i32 != 0 {
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
                                                var5 = (var0 as u32).wrapping_rem(8i32 as u32) as i32;
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
                                        let var6 = self.global22;
                                        var1 = var6;
                                        break 'label0;
                                        break;
                                    }
                                    let var7 = self.global23;
                                    var1 = var7;
                                    break 'label0;
                                    break;
                                }
                                let var8 = self.global24;
                                var1 = var8;
                                break 'label0;
                                break;
                            }
                            let var9 = self.global25;
                            var1 = var9;
                            break 'label0;
                            break;
                        }
                        let var10 = self.global26;
                        var1 = var10;
                        break 'label0;
                        break;
                    }
                    let var11 = self.global27;
                    var1 = var11;
                    break 'label0;
                    break;
                }
                let var12 = self.global26;
                let var13 = self.global27;
                let var14 = self.wasm_helpers_index_concatenateBytes(imports, var12, var13);
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var14);
                var1 = var15;
                break 'label0;
                break;
            }
            let var16 = self.global20;
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
                                                                                match ((var0 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32 {
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
                                                                        if (var0 as u32 <= 7i32 as u32) as i32 != 0 {
                                                                            let var17 = self.wasm_cpu_instructions_rotateRegisterLeft(imports, var1);
                                                                            var2 = var17;
                                                                            var3 = 1i32;
                                                                        } else {
                                                                            if (var0 as u32 <= 15i32 as u32) as i32 != 0 {
                                                                                let var18 = self.wasm_cpu_instructions_rotateRegisterRight(imports, var1);
                                                                                var2 = var18;
                                                                                var3 = 1i32;
                                                                            }
                                                                        }
                                                                        break 'label10;
                                                                        break;
                                                                    }
                                                                    if (var0 as u32 <= 23i32 as u32) as i32 != 0 {
                                                                        let var19 = self.wasm_cpu_instructions_rotateRegisterLeftThroughCarry(imports, var1);
                                                                        var2 = var19;
                                                                        var3 = 1i32;
                                                                    } else {
                                                                        if (var0 as u32 <= 31i32 as u32) as i32 != 0 {
                                                                            let var20 = self.wasm_cpu_instructions_rotateRegisterRightThroughCarry(imports, var1);
                                                                            var2 = var20;
                                                                            var3 = 1i32;
                                                                        }
                                                                    }
                                                                    break 'label10;
                                                                    break;
                                                                }
                                                                if (var0 as u32 <= 39i32 as u32) as i32 != 0 {
                                                                    let var21 = self.wasm_cpu_instructions_shiftLeftRegister(imports, var1);
                                                                    var2 = var21;
                                                                    var3 = 1i32;
                                                                } else {
                                                                    if (var0 as u32 <= 47i32 as u32) as i32 != 0 {
                                                                        let var22 = self.wasm_cpu_instructions_shiftRightArithmeticRegister(imports, var1);
                                                                        var2 = var22;
                                                                        var3 = 1i32;
                                                                    }
                                                                }
                                                                break 'label10;
                                                                break;
                                                            }
                                                            if (var0 as u32 <= 55i32 as u32) as i32 != 0 {
                                                                let var23 = self.wasm_cpu_instructions_swapNibblesOnRegister(imports, var1);
                                                                var2 = var23;
                                                                var3 = 1i32;
                                                            } else {
                                                                if (var0 as u32 <= 63i32 as u32) as i32 != 0 {
                                                                    let var24 = self.wasm_cpu_instructions_shiftRightLogicalRegister(imports, var1);
                                                                    var2 = var24;
                                                                    var3 = 1i32;
                                                                }
                                                            }
                                                            break 'label10;
                                                            break;
                                                        }
                                                        if (var0 as u32 <= 71i32 as u32) as i32 != 0 {
                                                            let var25 = self.wasm_cpu_instructions_testBitOnRegister(imports, 0i32, var1);
                                                            var2 = var25;
                                                            var3 = 1i32;
                                                        } else {
                                                            if (var0 as u32 <= 79i32 as u32) as i32 != 0 {
                                                                let var26 = self.wasm_cpu_instructions_testBitOnRegister(imports, 1i32, var1);
                                                                var2 = var26;
                                                                var3 = 1i32;
                                                            }
                                                        }
                                                        break 'label10;
                                                        break;
                                                    }
                                                    if (var0 as u32 <= 87i32 as u32) as i32 != 0 {
                                                        let var27 = self.wasm_cpu_instructions_testBitOnRegister(imports, 2i32, var1);
                                                        var2 = var27;
                                                        var3 = 1i32;
                                                    } else {
                                                        if (var0 as u32 <= 95i32 as u32) as i32 != 0 {
                                                            let var28 = self.wasm_cpu_instructions_testBitOnRegister(imports, 3i32, var1);
                                                            var2 = var28;
                                                            var3 = 1i32;
                                                        }
                                                    }
                                                    break 'label10;
                                                    break;
                                                }
                                                if (var0 as u32 <= 103i32 as u32) as i32 != 0 {
                                                    let var29 = self.wasm_cpu_instructions_testBitOnRegister(imports, 4i32, var1);
                                                    var2 = var29;
                                                    var3 = 1i32;
                                                } else {
                                                    if (var0 as u32 <= 111i32 as u32) as i32 != 0 {
                                                        let var30 = self.wasm_cpu_instructions_testBitOnRegister(imports, 5i32, var1);
                                                        var2 = var30;
                                                        var3 = 1i32;
                                                    }
                                                }
                                                break 'label10;
                                                break;
                                            }
                                            if (var0 as u32 <= 119i32 as u32) as i32 != 0 {
                                                let var31 = self.wasm_cpu_instructions_testBitOnRegister(imports, 6i32, var1);
                                                var2 = var31;
                                                var3 = 1i32;
                                            } else {
                                                if (var0 as u32 <= 127i32 as u32) as i32 != 0 {
                                                    let var32 = self.wasm_cpu_instructions_testBitOnRegister(imports, 7i32, var1);
                                                    var2 = var32;
                                                    var3 = 1i32;
                                                }
                                            }
                                            break 'label10;
                                            break;
                                        }
                                        if (var0 as u32 <= 135i32 as u32) as i32 != 0 {
                                            let var33 = self.wasm_cpu_instructions_setBitOnRegister(imports, 0i32, 0i32, var1);
                                            var2 = var33;
                                            var3 = 1i32;
                                        } else {
                                            if (var0 as u32 <= 143i32 as u32) as i32 != 0 {
                                                let var34 = self.wasm_cpu_instructions_setBitOnRegister(imports, 1i32, 0i32, var1);
                                                var2 = var34;
                                                var3 = 1i32;
                                            }
                                        }
                                        break 'label10;
                                        break;
                                    }
                                    if (var0 as u32 <= 151i32 as u32) as i32 != 0 {
                                        let var35 = self.wasm_cpu_instructions_setBitOnRegister(imports, 2i32, 0i32, var1);
                                        var2 = var35;
                                        var3 = 1i32;
                                    } else {
                                        if (var0 as u32 <= 159i32 as u32) as i32 != 0 {
                                            let var36 = self.wasm_cpu_instructions_setBitOnRegister(imports, 3i32, 0i32, var1);
                                            var2 = var36;
                                            var3 = 1i32;
                                        }
                                    }
                                    break 'label10;
                                    break;
                                }
                                if (var0 as u32 <= 167i32 as u32) as i32 != 0 {
                                    let var37 = self.wasm_cpu_instructions_setBitOnRegister(imports, 4i32, 0i32, var1);
                                    var2 = var37;
                                    var3 = 1i32;
                                } else {
                                    if (var0 as u32 <= 175i32 as u32) as i32 != 0 {
                                        let var38 = self.wasm_cpu_instructions_setBitOnRegister(imports, 5i32, 0i32, var1);
                                        var2 = var38;
                                        var3 = 1i32;
                                    }
                                }
                                break 'label10;
                                break;
                            }
                            if (var0 as u32 <= 183i32 as u32) as i32 != 0 {
                                let var39 = self.wasm_cpu_instructions_setBitOnRegister(imports, 6i32, 0i32, var1);
                                var2 = var39;
                                var3 = 1i32;
                            } else {
                                if (var0 as u32 <= 191i32 as u32) as i32 != 0 {
                                    let var40 = self.wasm_cpu_instructions_setBitOnRegister(imports, 7i32, 0i32, var1);
                                    var2 = var40;
                                    var3 = 1i32;
                                }
                            }
                            break 'label10;
                            break;
                        }
                        if (var0 as u32 <= 199i32 as u32) as i32 != 0 {
                            let var41 = self.wasm_cpu_instructions_setBitOnRegister(imports, 0i32, 1i32, var1);
                            var2 = var41;
                            var3 = 1i32;
                        } else {
                            if (var0 as u32 <= 207i32 as u32) as i32 != 0 {
                                let var42 = self.wasm_cpu_instructions_setBitOnRegister(imports, 1i32, 1i32, var1);
                                var2 = var42;
                                var3 = 1i32;
                            }
                        }
                        break 'label10;
                        break;
                    }
                    if (var0 as u32 <= 215i32 as u32) as i32 != 0 {
                        let var43 = self.wasm_cpu_instructions_setBitOnRegister(imports, 2i32, 1i32, var1);
                        var2 = var43;
                        var3 = 1i32;
                    } else {
                        if (var0 as u32 <= 223i32 as u32) as i32 != 0 {
                            let var44 = self.wasm_cpu_instructions_setBitOnRegister(imports, 3i32, 1i32, var1);
                            var2 = var44;
                            var3 = 1i32;
                        }
                    }
                    break 'label10;
                    break;
                }
                if (var0 as u32 <= 231i32 as u32) as i32 != 0 {
                    let var45 = self.wasm_cpu_instructions_setBitOnRegister(imports, 4i32, 1i32, var1);
                    var2 = var45;
                    var3 = 1i32;
                } else {
                    if (var0 as u32 <= 239i32 as u32) as i32 != 0 {
                        let var46 = self.wasm_cpu_instructions_setBitOnRegister(imports, 5i32, 1i32, var1);
                        var2 = var46;
                        var3 = 1i32;
                    }
                }
                break 'label10;
                break;
            }
            if (var0 as u32 <= 247i32 as u32) as i32 != 0 {
                let var47 = self.wasm_cpu_instructions_setBitOnRegister(imports, 6i32, 1i32, var1);
                var2 = var47;
                var3 = 1i32;
            } else {
                if (var0 as u32 <= 255i32 as u32) as i32 != 0 {
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
                                        self.global22 = var2;
                                        break 'label28;
                                        break;
                                    }
                                    self.global23 = var2;
                                    break 'label28;
                                    break;
                                }
                                self.global24 = var2;
                                break 'label28;
                                break;
                            }
                            self.global25 = var2;
                            break 'label28;
                            break;
                        }
                        self.global26 = var2;
                        break 'label28;
                        break;
                    }
                    self.global27 = var2;
                    break 'label28;
                    break;
                }
                let var49 = self.global26;
                let var50 = self.global27;
                let var51 = self.wasm_helpers_index_concatenateBytes(imports, var49, var50);
                self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var51, var2);
                break 'label28;
                break;
            }
            self.global20 = var2;
            break;
        }
        let var52 = self.global28;
        self.global28 = var52.wrapping_add(1i32) & 65535i32;
        if var3 != 0 {
            var4 = 8i32;
            if (var5 == 6i32) as i32 != 0 {
                var4 = 16i32;
            }
        }
        var4
    }
    fn wasm_cpu_opcodes_handleOpcodeCx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
        let mut var4: i32 = 0;
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
                                                                        let var5 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                        if var5 != 0 {
                                                                            return 8i32;
                                                                        } else {
                                                                            let var6 = self.global29;
                                                                            let var7 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var6);
                                                                            self.global28 = var7;
                                                                            let var8 = self.global29;
                                                                            self.global29 = var8.wrapping_add(2i32) & 65535i32;
                                                                            return 20i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var9 = self.global22;
                                                                    let var10 = self.global23;
                                                                    let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                                    let var12 = self.global29;
                                                                    let var13 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var12);
                                                                    var4 = var13;
                                                                    let var14 = self.global29;
                                                                    self.global29 = var14.wrapping_add(2i32) & 65535i32;
                                                                    let var15 = self.wasm_helpers_index_splitHighByte(imports, var4);
                                                                    self.global22 = var15;
                                                                    let var16 = self.wasm_helpers_index_splitLowByte(imports, var4);
                                                                    self.global23 = var16;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var17 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                                if var17 != 0 {
                                                                    let var18 = self.global28;
                                                                    self.global28 = var18.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                } else {
                                                                    self.global28 = var3;
                                                                    return 16i32;
                                                                }
                                                                unreachable!();
                                                                break;
                                                            }
                                                            self.global28 = var3;
                                                            return 16i32;
                                                            break;
                                                        }
                                                        let var19 = self.wasm_cpu_flags_getZeroFlag(imports);
                                                        if var19 != 0 {
                                                            let var20 = self.global28;
                                                            self.global28 = var20.wrapping_add(2i32) & 65535i32;
                                                            return 12i32;
                                                        } else {
                                                            let var21 = self.global29;
                                                            self.global29 = var21.wrapping_sub(2i32) & 65535i32;
                                                            let var22 = self.global29;
                                                            let var23 = self.global28;
                                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var22, var23.wrapping_add(2i32) & 65535i32);
                                                            self.global28 = var3;
                                                            return 24i32;
                                                        }
                                                        unreachable!();
                                                        break;
                                                    }
                                                    let var24 = self.global22;
                                                    let var25 = self.global23;
                                                    let var26 = self.wasm_helpers_index_concatenateBytes(imports, var24, var25);
                                                    var4 = var26;
                                                    let var27 = self.global29;
                                                    self.global29 = var27.wrapping_sub(2i32) & 65535i32;
                                                    let var28 = self.global29;
                                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var28, var4);
                                                    return 16i32;
                                                    break;
                                                }
                                                self.wasm_cpu_instructions_addARegister(imports, var1);
                                                let var29 = self.global28;
                                                self.global28 = var29.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var30 = self.global29;
                                            self.global29 = var30.wrapping_sub(2i32) & 65535i32;
                                            let var31 = self.global29;
                                            let var32 = self.global28;
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var31, var32);
                                            self.global28 = 0i32;
                                            return 16i32;
                                            break;
                                        }
                                        let var33 = self.wasm_cpu_flags_getZeroFlag(imports);
                                        if (var33 == 1i32) as i32 != 0 {
                                            let var34 = self.global29;
                                            let var35 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var34);
                                            self.global28 = var35;
                                            let var36 = self.global29;
                                            self.global29 = var36.wrapping_add(2i32) & 65535i32;
                                            return 20i32;
                                        } else {
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var37 = self.global29;
                                    let var38 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var37);
                                    self.global28 = var38;
                                    let var39 = self.global29;
                                    self.global29 = var39.wrapping_add(2i32) & 65535i32;
                                    return 16i32;
                                    break;
                                }
                                let var40 = self.wasm_cpu_flags_getZeroFlag(imports);
                                if (var40 == 1i32) as i32 != 0 {
                                    self.global28 = var3;
                                    return 16i32;
                                } else {
                                    let var41 = self.global28;
                                    self.global28 = var41.wrapping_add(2i32) & 65535i32;
                                    return 12i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var42 = self.wasm_cpu_cbOpcodes_handleCbOpcode(imports, var1);
                            var4 = var42;
                            if (var4 > 0i32) as i32 != 0 {
                                var4 = var4.wrapping_add(4i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            }
                            return var4;
                            break;
                        }
                        let var43 = self.wasm_cpu_flags_getZeroFlag(imports);
                        if (var43 == 1i32) as i32 != 0 {
                            let var44 = self.global29;
                            self.global29 = var44.wrapping_sub(2i32) & 65535i32;
                            let var45 = self.global29;
                            let var46 = self.global28;
                            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var45, var46.wrapping_add(2i32) & 65535i32);
                            self.global28 = var3;
                            return 24i32;
                        } else {
                            let var47 = self.global28;
                            self.global28 = var47.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var48 = self.global29;
                    self.global29 = var48.wrapping_sub(2i32) & 65535i32;
                    let var49 = self.global29;
                    let var50 = self.global28;
                    self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var49, var50.wrapping_add(2i32) & 65535i32);
                    self.global28 = var3;
                    return 24i32;
                    break;
                }
                self.wasm_cpu_instructions_addAThroughCarryRegister(imports, var1);
                let var51 = self.global28;
                self.global28 = var51.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var52 = self.global29;
            self.global29 = var52.wrapping_sub(2i32) & 65535i32;
            let var53 = self.global29;
            let var54 = self.global28;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var53, var54);
            self.global28 = 8i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_interrupts_index_setInterrupts<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.global119 = var0;
    }
    fn wasm_cpu_opcodes_handleOpcodeDx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
        let mut var4: i32 = 0;
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
                                                            let var5 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                            if var5 != 0 {
                                                                return 8i32;
                                                            } else {
                                                                let var6 = self.global29;
                                                                let var7 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var6);
                                                                self.global28 = var7;
                                                                let var8 = self.global29;
                                                                self.global29 = var8.wrapping_add(2i32) & 65535i32;
                                                                return 20i32;
                                                            }
                                                            unreachable!();
                                                            break;
                                                        }
                                                        let var9 = self.global24;
                                                        let var10 = self.global25;
                                                        let var11 = self.wasm_helpers_index_concatenateBytes(imports, var9, var10);
                                                        let var12 = self.global29;
                                                        let var13 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var12);
                                                        var4 = var13;
                                                        let var14 = self.global29;
                                                        self.global29 = var14.wrapping_add(2i32) & 65535i32;
                                                        let var15 = self.wasm_helpers_index_splitHighByte(imports, var4);
                                                        self.global24 = var15;
                                                        let var16 = self.wasm_helpers_index_splitLowByte(imports, var4);
                                                        self.global25 = var16;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var17 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                    if var17 != 0 {
                                                        let var18 = self.global28;
                                                        self.global28 = var18.wrapping_add(2i32) & 65535i32;
                                                        return 12i32;
                                                    } else {
                                                        self.global28 = var3;
                                                        return 16i32;
                                                    }
                                                    unreachable!();
                                                    break;
                                                }
                                                let var19 = self.wasm_cpu_flags_getCarryFlag(imports);
                                                if var19 != 0 {
                                                    let var20 = self.global28;
                                                    self.global28 = var20.wrapping_add(2i32) & 65535i32;
                                                    return 12i32;
                                                } else {
                                                    let var21 = self.global29;
                                                    self.global29 = var21.wrapping_sub(2i32) & 65535i32;
                                                    let var22 = self.global29;
                                                    let var23 = self.global28;
                                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var22, var23.wrapping_add(2i32) & 65535i32);
                                                    self.global28 = var3;
                                                    return 24i32;
                                                }
                                                unreachable!();
                                                break;
                                            }
                                            let var24 = self.global24;
                                            let var25 = self.global25;
                                            let var26 = self.wasm_helpers_index_concatenateBytes(imports, var24, var25);
                                            var4 = var26;
                                            let var27 = self.global29;
                                            self.global29 = var27.wrapping_sub(2i32) & 65535i32;
                                            let var28 = self.global29;
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var28, var4);
                                            return 16i32;
                                            break;
                                        }
                                        self.wasm_cpu_instructions_subARegister(imports, var1);
                                        let var29 = self.global28;
                                        self.global28 = var29.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var30 = self.global29;
                                    self.global29 = var30.wrapping_sub(2i32) & 65535i32;
                                    let var31 = self.global29;
                                    let var32 = self.global28;
                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var31, var32);
                                    self.global28 = 16i32;
                                    return 16i32;
                                    break;
                                }
                                let var33 = self.wasm_cpu_flags_getCarryFlag(imports);
                                if (var33 == 1i32) as i32 != 0 {
                                    let var34 = self.global29;
                                    let var35 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var34);
                                    self.global28 = var35;
                                    let var36 = self.global29;
                                    self.global29 = var36.wrapping_add(2i32) & 65535i32;
                                    return 20i32;
                                } else {
                                    return 8i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var37 = self.global29;
                            let var38 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var37);
                            self.global28 = var38;
                            self.wasm_interrupts_index_setInterrupts(imports, 1i32);
                            let var39 = self.global29;
                            self.global29 = var39.wrapping_add(2i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var40 = self.wasm_cpu_flags_getCarryFlag(imports);
                        if (var40 == 1i32) as i32 != 0 {
                            self.global28 = var3;
                            return 16i32;
                        } else {
                            let var41 = self.global28;
                            self.global28 = var41.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var42 = self.wasm_cpu_flags_getCarryFlag(imports);
                    if (var42 == 1i32) as i32 != 0 {
                        let var43 = self.global29;
                        self.global29 = var43.wrapping_sub(2i32) & 65535i32;
                        let var44 = self.global29;
                        let var45 = self.global28;
                        self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var44, var45.wrapping_add(2i32) & 65535i32);
                        self.global28 = var3;
                        return 24i32;
                    } else {
                        let var46 = self.global28;
                        self.global28 = var46.wrapping_add(2i32) & 65535i32;
                        return 12i32;
                    }
                    unreachable!();
                    break;
                }
                self.wasm_cpu_instructions_subAThroughCarryRegister(imports, var1);
                let var47 = self.global28;
                self.global28 = var47.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var48 = self.global29;
            self.global29 = var48.wrapping_sub(2i32) & 65535i32;
            let var49 = self.global29;
            let var50 = self.global28;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var49, var50);
            self.global28 = 24i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcodeEx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
        let mut var4: i32 = 0;
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
                                                    let var5 = self.global20;
                                                    self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var1.wrapping_add(65280i32) & 65535i32, var5);
                                                    let var6 = self.global28;
                                                    self.global28 = var6.wrapping_add(1i32) & 65535i32;
                                                    return 12i32;
                                                    break;
                                                }
                                                let var7 = self.global26;
                                                let var8 = self.global27;
                                                let var9 = self.wasm_helpers_index_concatenateBytes(imports, var7, var8);
                                                let var10 = self.global29;
                                                let var11 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var10);
                                                var4 = var11;
                                                let var12 = self.global29;
                                                self.global29 = var12.wrapping_add(2i32) & 65535i32;
                                                let var13 = self.wasm_helpers_index_splitHighByte(imports, var4);
                                                self.global26 = var13;
                                                let var14 = self.wasm_helpers_index_splitLowByte(imports, var4);
                                                self.global27 = var14;
                                                return 12i32;
                                                break;
                                            }
                                            let var15 = self.global23;
                                            let var16 = self.global20;
                                            self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var15.wrapping_add(65280i32) & 65535i32, var16);
                                            return 8i32;
                                            break;
                                        }
                                        let var17 = self.global26;
                                        let var18 = self.global27;
                                        let var19 = self.wasm_helpers_index_concatenateBytes(imports, var17, var18);
                                        var4 = var19;
                                        let var20 = self.global29;
                                        self.global29 = var20.wrapping_sub(2i32) & 65535i32;
                                        let var21 = self.global29;
                                        self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var21, var4);
                                        return 16i32;
                                        break;
                                    }
                                    self.wasm_cpu_instructions_andARegister(imports, var1);
                                    let var22 = self.global28;
                                    self.global28 = var22.wrapping_add(1i32) & 65535i32;
                                    return 8i32;
                                    break;
                                }
                                let var23 = self.global29;
                                self.global29 = var23.wrapping_sub(2i32) & 65535i32;
                                let var24 = self.global29;
                                let var25 = self.global28;
                                self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var24, var25);
                                self.global28 = 32i32;
                                return 16i32;
                                break;
                            }
                            let var26 = self.global29;
                            var4 = var1.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var26, var4, 1i32);
                            let var27 = self.global29;
                            self.global29 = var27.wrapping_add(var4) & 65535i32;
                            self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                            self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                            let var28 = self.global28;
                            self.global28 = var28.wrapping_add(1i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var29 = self.global26;
                        let var30 = self.global27;
                        let var31 = self.wasm_helpers_index_concatenateBytes(imports, var29, var30);
                        self.global28 = var31;
                        return 4i32;
                        break;
                    }
                    let var32 = self.global20;
                    self.wasm_memory_store_eightBitStoreIntoGBMemory(imports, var3, var32);
                    let var33 = self.global28;
                    self.global28 = var33.wrapping_add(2i32) & 65535i32;
                    return 16i32;
                    break;
                }
                self.wasm_cpu_instructions_xorARegister(imports, var1);
                let var34 = self.global28;
                self.global28 = var34.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var35 = self.global29;
            self.global29 = var35.wrapping_sub(2i32) & 65535i32;
            let var36 = self.global29;
            let var37 = self.global28;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var36, var37);
            self.global28 = 40i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_handleOpcodeFx<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                            let var4 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var1.wrapping_add(65280i32) & 65535i32);
                                                            self.global20 = var4;
                                                            let var5 = self.global28;
                                                            self.global28 = var5.wrapping_add(1i32) & 65535i32;
                                                            return 12i32;
                                                            break;
                                                        }
                                                        let var6 = self.global20;
                                                        let var7 = self.global21;
                                                        let var8 = self.wasm_helpers_index_concatenateBytes(imports, var6, var7);
                                                        let var9 = self.global29;
                                                        let var10 = self.wasm_memory_load_sixteenBitLoadFromGBMemory(imports, var9);
                                                        var0 = var10;
                                                        let var11 = self.global29;
                                                        self.global29 = var11.wrapping_add(2i32) & 65535i32;
                                                        let var12 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                                        self.global20 = var12;
                                                        let var13 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                                        self.global21 = var13;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var14 = self.global23;
                                                    let var15 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var14.wrapping_add(65280i32) & 65535i32);
                                                    self.global20 = var15;
                                                    return 8i32;
                                                    break;
                                                }
                                                self.wasm_interrupts_index_setInterrupts(imports, 0i32);
                                                return 4i32;
                                                break;
                                            }
                                            let var16 = self.global20;
                                            let var17 = self.global21;
                                            let var18 = self.wasm_helpers_index_concatenateBytes(imports, var16, var17);
                                            var0 = var18;
                                            let var19 = self.global29;
                                            self.global29 = var19.wrapping_sub(2i32) & 65535i32;
                                            let var20 = self.global29;
                                            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var20, var0);
                                            return 16i32;
                                            break;
                                        }
                                        self.wasm_cpu_instructions_orARegister(imports, var1);
                                        let var21 = self.global28;
                                        self.global28 = var21.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var22 = self.global29;
                                    self.global29 = var22.wrapping_sub(2i32) & 65535i32;
                                    let var23 = self.global29;
                                    let var24 = self.global28;
                                    self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var23, var24);
                                    self.global28 = 48i32;
                                    return 16i32;
                                    break;
                                }
                                self.wasm_cpu_flags_setZeroFlag(imports, 0i32);
                                self.wasm_cpu_flags_setSubtractFlag(imports, 0i32);
                                let var25 = self.global29;
                                var0 = var1.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                self.wasm_cpu_flags_checkAndSetSixteenBitFlagsAddOverflow(imports, var25, var0, 1i32);
                                let var26 = self.global29;
                                var0 = var26.wrapping_add(var0) & 65535i32;
                                let var27 = self.wasm_helpers_index_splitHighByte(imports, var0);
                                self.global26 = var27;
                                let var28 = self.wasm_helpers_index_splitLowByte(imports, var0);
                                self.global27 = var28;
                                let var29 = self.global28;
                                self.global28 = var29.wrapping_add(1i32) & 65535i32;
                                return 12i32;
                                break;
                            }
                            let var30 = self.global26;
                            let var31 = self.global27;
                            let var32 = self.wasm_helpers_index_concatenateBytes(imports, var30, var31);
                            self.global29 = var32;
                            return 8i32;
                            break;
                        }
                        let var33 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var3);
                        self.global20 = var33;
                        let var34 = self.global28;
                        self.global28 = var34.wrapping_add(2i32) & 65535i32;
                        return 16i32;
                        break;
                    }
                    self.wasm_interrupts_index_setInterrupts(imports, 1i32);
                    return 4i32;
                    break;
                }
                self.wasm_cpu_instructions_cpARegister(imports, var1);
                let var35 = self.global28;
                self.global28 = var35.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var36 = self.global29;
            self.global29 = var36.wrapping_sub(2i32) & 65535i32;
            let var37 = self.global29;
            let var38 = self.global28;
            self.wasm_memory_store_sixteenBitStoreIntoGBMemory(imports, var37, var38);
            self.global28 = 56i32;
            return 16i32;
            break;
        }
        -1i32
    }
    fn wasm_cpu_opcodes_executeOpcode<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let var4 = self.global28;
        self.global28 = var4.wrapping_add(1i32) & 65535i32;
        let var5 = self.wasm_helpers_index_concatenateBytes(imports, var2, var1);
        var3 = var5;
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
                                                                            match ((var0 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32 {
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
                                                                    let var6 = self.wasm_cpu_opcodes_handleOpcode0x(imports, var0, var1, var2, var3);
                                                                    return var6;
                                                                    break;
                                                                }
                                                                let var7 = self.wasm_cpu_opcodes_handleOpcode1x(imports, var0, var1, var2, var3);
                                                                return var7;
                                                                break;
                                                            }
                                                            let var8 = self.wasm_cpu_opcodes_handleOpcode2x(imports, var0, var1, var2, var3);
                                                            return var8;
                                                            break;
                                                        }
                                                        let var9 = self.wasm_cpu_opcodes_handleOpcode3x(imports, var0, var1, var2, var3);
                                                        return var9;
                                                        break;
                                                    }
                                                    let var10 = self.wasm_cpu_opcodes_handleOpcode4x(imports, var0, var1, var2, var3);
                                                    return var10;
                                                    break;
                                                }
                                                let var11 = self.wasm_cpu_opcodes_handleOpcode5x(imports, var0, var1, var2, var3);
                                                return var11;
                                                break;
                                            }
                                            let var12 = self.wasm_cpu_opcodes_handleOpcode6x(imports, var0, var1, var2, var3);
                                            return var12;
                                            break;
                                        }
                                        let var13 = self.wasm_cpu_opcodes_handleOpcode7x(imports, var0, var1, var2, var3);
                                        return var13;
                                        break;
                                    }
                                    let var14 = self.wasm_cpu_opcodes_handleOpcode8x(imports, var0, var1, var2, var3);
                                    return var14;
                                    break;
                                }
                                let var15 = self.wasm_cpu_opcodes_handleOpcode9x(imports, var0, var1, var2, var3);
                                return var15;
                                break;
                            }
                            let var16 = self.wasm_cpu_opcodes_handleOpcodeAx(imports, var0, var1, var2, var3);
                            return var16;
                            break;
                        }
                        let var17 = self.wasm_cpu_opcodes_handleOpcodeBx(imports, var0, var1, var2, var3);
                        return var17;
                        break;
                    }
                    let var18 = self.wasm_cpu_opcodes_handleOpcodeCx(imports, var0, var1, var2, var3);
                    return var18;
                    break;
                }
                let var19 = self.wasm_cpu_opcodes_handleOpcodeDx(imports, var0, var1, var2, var3);
                return var19;
                break;
            }
            let var20 = self.wasm_cpu_opcodes_handleOpcodeEx(imports, var0, var1, var2, var3);
            return var20;
            break;
        }
        let var21 = self.wasm_cpu_opcodes_handleOpcodeFx(imports, var0, var1, var2, var3);
        var21
    }
    fn wasm_interrupts_index_areInterruptsEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global119;
        var0
    }
    fn wasm_interrupts_index_areInterruptsPending<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65295i32);
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65535i32);
        let var2: i32;
        if var0 & var1 & 255i32 != 0 {
            var2 = 1i32;
        } else {
            var2 = 0i32;
        }
        var2
    }
    fn wasm_interrupts_index__handleInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.wasm_interrupts_index_setInterrupts(imports, 0i32);
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65295i32);
        let var2 = self.wasm_helpers_index_resetBitOnByte(imports, var0, var1);
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65295i32, var2);
        let var3 = self.global29;
        self.global29 = var3.wrapping_sub(2i32) & 65535i32;
        let var4 = self.global29;
        let var5 = self.global28;
        self.wasm_memory_store_sixteenBitStoreIntoGBMemorySkipTraps(imports, var4, var5);
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
                        self.global28 = 64i32;
                        break 'label0;
                        break;
                    }
                    self.global28 = 72i32;
                    break 'label0;
                    break;
                }
                self.global28 = 80i32;
                break 'label0;
                break;
            }
            self.global28 = 96i32;
            break;
        }
    }
    fn wasm_interrupts_index_checkInterrupts<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global119;
        if var4 != 0 {
            let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65295i32);
            var2 = var5;
            let var6 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65535i32);
            var3 = var6;
            if (var2 as u32 > 0i32 as u32) as i32 != 0 {
                let var7 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var2);
                var0 = var7;
                let var8: i32;
                if var0 != 0 {
                    let var9 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var3);
                    var8 = var9;
                } else {
                    var8 = var0;
                }
                if var8 & 1i32 != 0 {
                    self.wasm_interrupts_index__handleInterrupt(imports, 0i32);
                    var1 = 1i32;
                } else {
                    let var10 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var2);
                    var0 = var10;
                    let var11: i32;
                    if var0 != 0 {
                        let var12 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var3);
                        var11 = var12;
                    } else {
                        var11 = var0;
                    }
                    if var11 & 1i32 != 0 {
                        self.wasm_interrupts_index__handleInterrupt(imports, 1i32);
                        var1 = 1i32;
                    } else {
                        let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var2);
                        var0 = var13;
                        let var14: i32;
                        if var0 != 0 {
                            let var15 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var3);
                            var14 = var15;
                        } else {
                            var14 = var0;
                        }
                        if var14 & 1i32 != 0 {
                            self.wasm_interrupts_index__handleInterrupt(imports, 2i32);
                            var1 = 1i32;
                        } else {
                            let var16 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var2);
                            var0 = var16;
                            let var17: i32;
                            if var0 != 0 {
                                let var18 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var3);
                                var17 = var18;
                            } else {
                                var17 = var0;
                            }
                            if var17 & 1i32 != 0 {
                                self.wasm_interrupts_index__handleInterrupt(imports, 4i32);
                                var1 = 1i32;
                            }
                        }
                    }
                }
            }
            if var1 != 0 {
                var0 = 20i32;
                let var19 = self.global44;
                if var19 != 0 {
                    self.global44 = 0i32;
                    var0 = 24i32;
                }
                return var0;
            }
        }
        0i32
    }
    fn wasm_graphics_lcd_isLcdEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65344i32);
        let var1 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var0);
        var1
    }
    fn wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 912i32;
        }
        456i32
    }
    fn wasm_memory_memory_loadFromVramBank<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = self.memory.load8(var0.wrapping_add(-30720i32).wrapping_add((var1 & 1i32).wrapping_mul(8192i32)) as usize) as i32;
        var2
    }
    fn wasm_graphics_renderUtils_getTileDataAddress<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let var3: i32;
        if (var0 == 34816i32) as i32 != 0 {
            var2 = var1.wrapping_add(128i32) & 255i32;
            let var4 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var1);
            if var4 != 0 {
                var2 = var1.wrapping_sub(128i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
            }
            var3 = var0.wrapping_add(var2.wrapping_mul(16i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32) & 65535i32) & 65535i32;
        } else {
            var3 = var0.wrapping_add(var1.wrapping_mul(16i32)) & 65535i32;
        }
        var3
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
    fn wasm_graphics_palette_getRgbColorFromPalette<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        var3 = (var0.wrapping_mul(8i32) & 255i32).wrapping_add(var1.wrapping_mul(2i32) & 255i32) & 255i32;
        let var4 = self.wasm_memory_memory_loadPaletteByteFromWasmMemory(imports, var3.wrapping_add(1i32) & 255i32, var2);
        let var5 = self.wasm_memory_memory_loadPaletteByteFromWasmMemory(imports, var3, var2);
        let var6 = self.wasm_helpers_index_concatenateBytes(imports, var4, var5);
        var6
    }
    fn wasm_graphics_palette_getColorComponentFromRgb<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        (((var1 & 31i32.wrapping_shl((var0.wrapping_mul(5i32) & 255i32) as u32) & 65535i32) as u32).wrapping_shr((var0.wrapping_mul(5i32) & 255i32) as u32) as i32).wrapping_mul(8i32) & 255i32
    }
    fn wasm_memory_memory_getRgbPixelStart<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var1.wrapping_mul(160i32).wrapping_add(var0).wrapping_mul(3i32)
    }
    fn wasm_memory_memory_setPixelOnFrame<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) {
        let var4 = self.wasm_memory_memory_getRgbPixelStart(imports, var0, var1);
        self.memory.store8(var4.wrapping_add(93184i32).wrapping_add(var2) as usize, var3 as u8);
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
    fn wasm_graphics_backgroundWindow_drawColorPixelFromTile<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let var8 = self.wasm_memory_memory_loadFromVramBank(imports, var4, 1i32);
        var4 = var8;
        var3 = (var3 as u32).wrapping_rem(8i32 as u32) as i32;
        let var9 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var4);
        if var9 != 0 {
            var3 = 7i32.wrapping_sub(var3) & 65535i32;
        }
        let var10 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var4);
        if var10 != 0 {
            var6 = 1i32;
        }
        let var11 = self.wasm_memory_memory_loadFromVramBank(imports, var5.wrapping_add(var3.wrapping_mul(2i32) & 65535i32) & 65535i32, var6);
        var7 = var11;
        let var12 = self.wasm_memory_memory_loadFromVramBank(imports, var5.wrapping_add(var3.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32, var6);
        var5 = var12;
        var3 = ((var2 & 255i32) as u32).wrapping_rem(8i32 as u32) as i32;
        let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var4);
        if (var13 == 0) as i32 != 0 {
            var3 = 7i32.wrapping_sub(var3) & 255i32;
        }
        var2 = 0i32;
        let var14 = self.wasm_helpers_index_checkBitOnByte(imports, var3, var5);
        if var14 != 0 {
            var2 = 2i32;
        }
        let var15 = self.wasm_helpers_index_checkBitOnByte(imports, var3, var7);
        if var15 != 0 {
            var2 = var2.wrapping_add(1i32) & 255i32;
        }
        let var16 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var4 & 7i32, var2, 0i32);
        var3 = var16;
        let var17 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var3);
        var5 = var17;
        let var18 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var3);
        var6 = var18;
        let var19 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var3);
        var3 = var19;
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 0i32, var5);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 1i32, var6);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 2i32, var3);
        let var20 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var4);
        self.wasm_graphics_priority_addPriorityforPixel(imports, var0, var1, var2, var20);
    }
    fn wasm_graphics_palette_getMonochromeColorFromPalette<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        if (var2 == 0) as i32 != 0 {
            let var3 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var1);
            var0 = (var3 as u32).wrapping_shr((var0.wrapping_mul(2i32) & 255i32) as u32) as i32 & 3i32;
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
    fn wasm_graphics_backgroundWindow_drawMonochromePixelFromTile<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) {
        let mut var5: i32 = 0;
        var3 = (var3 as u32).wrapping_rem(8i32 as u32) as i32;
        let var6 = self.wasm_memory_memory_loadFromVramBank(imports, var4.wrapping_add(var3.wrapping_mul(2i32) & 65535i32) & 65535i32, 0i32);
        var5 = var6;
        let var7 = self.wasm_memory_memory_loadFromVramBank(imports, var4.wrapping_add(var3.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32, 0i32);
        var4 = var7;
        var3 = 0i32;
        var2 = 7i32.wrapping_sub(((var2 & 255i32) as u32).wrapping_rem(8i32 as u32) as i32) & 255i32;
        let var8 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var4);
        if var8 != 0 {
            var3 = 2i32;
        }
        let var9 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var5);
        if var9 != 0 {
            var3 = var3.wrapping_add(1i32) & 255i32;
        }
        let var10 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var3, 65351i32, 0i32);
        var2 = var10;
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 0i32, var2);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 1i32, var2);
        self.wasm_memory_memory_setPixelOnFrame(imports, var0, var1, 2i32, var2);
        self.wasm_graphics_priority_addPriorityforPixel(imports, var0, var1, var3, 0i32);
    }
    fn wasm_graphics_backgroundWindow_drawBackgroundWindowScanline<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32) {
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        'label0: loop {
            if ((var4) < 160i32) as i32 != 0 {
                var6 = var4.wrapping_add(var5);
                if (var6 >= 256i32) as i32 != 0 {
                    var6 = var6.wrapping_sub(256i32);
                }
                var8 = var2.wrapping_add(((var3 as u32).wrapping_shr(3i32 as u32) as i32).wrapping_mul(32i32) & 65535i32).wrapping_add(var6.wrapping_shr(3i32 as u32) & 65535i32) & 65535i32;
                let var9 = self.wasm_memory_memory_loadFromVramBank(imports, var8, 0i32);
                let var10 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var1, var9);
                var7 = var10;
                let var11 = self.global16;
                if var11 != 0 {
                    self.wasm_graphics_backgroundWindow_drawColorPixelFromTile(imports, var4, var0, var6, var3, var8, var7);
                } else {
                    self.wasm_graphics_backgroundWindow_drawMonochromePixelFromTile(imports, var4, var0, var6, var3, var7);
                }
                var4 = var4.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_backgroundWindow_renderBackground<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65347i32);
        var4 = var5;
        let var6 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65346i32);
        var3 = var0.wrapping_add(var6) & 65535i32;
        if (var3 as u32 >= 256i32 as u32) as i32 != 0 {
            var3 = var3.wrapping_sub(256i32) & 65535i32;
        }
        self.wasm_graphics_backgroundWindow_drawBackgroundWindowScanline(imports, var0, var1, var2, var3, 0i32, var4);
    }
    fn wasm_graphics_backgroundWindow_renderWindow<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65355i32);
        var3 = var5;
        let var6 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65354i32);
        var4 = var6;
        if ((var0 as u32) < (var4 & 255i32) as u32) as i32 != 0 {
            return;
        }
        var3 = var3.wrapping_sub(7i32) & 65535i32;
        self.wasm_graphics_backgroundWindow_drawBackgroundWindowScanline(imports, var0, var1, var2, var0.wrapping_sub(var4) & 65535i32, var3, var3.wrapping_mul(-1i32));
    }
    fn wasm_graphics_priority_getPriorityforPixel<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = self.wasm_graphics_priority_getPixelStart(imports, var0, var1);
        let var3 = self.memory.load8(var2.wrapping_add(69632i32) as usize) as i32;
        var3
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
        var8 = 39i32;
        'label0: loop {
            if (var8 >= 0i32) as i32 != 0 {
                var3 = var8.wrapping_mul(4i32) & 65535i32;
                let var15 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3.wrapping_add(65024i32) & 65535i32);
                var2 = var15;
                let var16 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3.wrapping_add(65025i32) & 65535i32);
                var9 = var16;
                let var17 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3.wrapping_add(65026i32) & 65535i32);
                var4 = var17;
                var2 = var2.wrapping_sub(16i32) & 255i32;
                var9 = var9.wrapping_sub(8i32) & 255i32;
                let var18 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var3.wrapping_add(65027i32) & 65535i32);
                var7 = var18;
                let var19 = self.wasm_helpers_index_checkBitOnByte(imports, 7i32, var7);
                var10 = var19;
                let var20 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var7);
                var6 = var20;
                let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var7);
                var12 = var21;
                var5 = 8i32;
                if var1 != 0 {
                    var5 = 16i32;
                    if ((var4 as u32).wrapping_rem(2i32 as u32) as i32 == 1i32) as i32 != 0 {
                        var4 = var4.wrapping_sub(1i32) & 255i32;
                    }
                }
                var3 = (var0 as u32 >= var2 as u32) as i32;
                let var22: i32;
                if var3 != 0 {
                    var22 = ((var0 as u32) < (var2.wrapping_add(var5) & 255i32) as u32) as i32;
                } else {
                    var22 = var3;
                }
                if var22 & 1i32 != 0 {
                    var3 = var0.wrapping_sub(var2) & 255i32;
                    if var6 != 0 {
                        var3 = var3.wrapping_sub(var5).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_sub(1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
                    }
                    let var23 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, 32768i32, var4);
                    var2 = var23.wrapping_add(var3.wrapping_mul(2i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32));
                    var3 = 0i32;
                    let var24 = self.global16;
                    let var25: i32;
                    if var24 != 0 {
                        let var26 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var7);
                        var25 = var26;
                    } else {
                        let var27 = self.global16;
                        var25 = var27;
                    }
                    if var25 & 1i32 != 0 {
                        var3 = 1i32;
                    }
                    var2 = var2 & 65535i32;
                    let var28 = self.wasm_memory_memory_loadFromVramBank(imports, var2, var3);
                    var13 = var28;
                    let var29 = self.wasm_memory_memory_loadFromVramBank(imports, var2.wrapping_add(1i32) & 65535i32, var3);
                    var14 = var29;
                    var3 = 7i32;
                    'label1: loop {
                        if (var3 >= 0i32) as i32 != 0 {
                            var2 = var3;
                            if var12 != 0 {
                                var2 = var2.wrapping_sub(7i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32).wrapping_mul(-1i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            }
                            var4 = 0i32;
                            let var30 = self.wasm_helpers_index_checkBitOnByte(imports, var2 & 255i32, var14);
                            if var30 != 0 {
                                var4 = 2i32;
                            }
                            let var31 = self.wasm_helpers_index_checkBitOnByte(imports, var2 & 255i32, var13);
                            if var31 != 0 {
                                var4 = var4.wrapping_add(1i32) & 255i32;
                            }
                            if var4 != 0 {
                                var5 = var9.wrapping_add(7i32.wrapping_sub(var3 & 255i32) & 255i32) & 255i32;
                                let var32 = self.wasm_graphics_priority_getPriorityforPixel(imports, var5, var0);
                                var2 = var32;
                                var6 = 0i32;
                                let var33: i32;
                                if var10 != 0 {
                                    var33 = ((var2 & 3i32) as u32 > 0i32 as u32) as i32;
                                } else {
                                    var33 = var10;
                                }
                                if var33 & 1i32 != 0 {
                                    var6 = 1i32;
                                }
                                var11 = 0i32;
                                let var34 = self.global16;
                                let var35: i32;
                                if var34 != 0 {
                                    let var36 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var2);
                                    var35 = var36;
                                } else {
                                    let var37 = self.global16;
                                    var35 = var37;
                                }
                                if var35 & 1i32 != 0 {
                                    var11 = 1i32;
                                }
                                var2 = 0i32;
                                let var38 = self.global16;
                                let var39: i32;
                                if var38 != 0 {
                                    let var40 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65344i32);
                                    let var41 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var40);
                                    var39 = (var41 == 0) as i32;
                                } else {
                                    let var42 = self.global16;
                                    var39 = var42;
                                }
                                if var39 & 1i32 != 0 {
                                    var2 = 1i32;
                                }
                                let var43: i32;
                                if var2 != 0 {
                                    var43 = var2;
                                } else {
                                    var2 = (var6 == 0) as i32;
                                    let var44: i32;
                                    if var2 != 0 {
                                        var44 = (var11 == 0) as i32;
                                    } else {
                                        var44 = var2;
                                    }
                                    var43 = var44 & 1i32;
                                }
                                if var43 & 1i32 != 0 {
                                    let var45 = self.global16;
                                    if var45 != 0 {
                                        let var46 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var7 & 7i32, var4, 1i32);
                                        var2 = var46;
                                        let var47 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var2);
                                        var4 = var47;
                                        let var48 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var2);
                                        var6 = var48;
                                        let var49 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var2);
                                        var2 = var49;
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 0i32, var4);
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 1i32, var6);
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 2i32, var2);
                                    } else {
                                        var2 = 65352i32;
                                        let var50 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var7);
                                        if var50 != 0 {
                                            var2 = 65353i32;
                                        }
                                        let var51 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var4, var2, 0i32);
                                        var2 = var51;
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 0i32, var2);
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 1i32, var2);
                                        self.wasm_memory_memory_setPixelOnFrame(imports, var5, var0, 2i32, var2);
                                    }
                                }
                            }
                            var3 = var3.wrapping_sub(1i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
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
        let mut var3: i32 = 0;
        var3 = 34816i32;
        let var4 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65344i32);
        var1 = var4;
        let var5 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var1);
        if var5 != 0 {
            var3 = 32768i32;
        }
        let var6 = self.global16;
        let var7: i32;
        if var6 != 0 {
            let var8 = self.global16;
            var7 = var8;
        } else {
            let var9 = self.wasm_helpers_index_checkBitOnByte(imports, 0i32, var1);
            var7 = var9;
        }
        if var7 & 1i32 != 0 {
            var2 = 38912i32;
            let var10 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var1);
            if var10 != 0 {
                var2 = 39936i32;
            }
            self.wasm_graphics_backgroundWindow_renderBackground(imports, var0, var3, var2);
        }
        let var11 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var1);
        if var11 != 0 {
            var2 = 38912i32;
            let var12 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var1);
            if var12 != 0 {
                var2 = 39936i32;
            }
            self.wasm_graphics_backgroundWindow_renderWindow(imports, var0, var3, var2);
        }
        let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 1i32, var1);
        if var13 != 0 {
            let var14 = self.wasm_helpers_index_checkBitOnByte(imports, 2i32, var1);
            self.wasm_graphics_sprites_renderSprites(imports, var0, var14);
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
    fn wasm_memory_memory_storeFrameToBeRendered<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        var4 = 162816i32;
        var5 = 93184i32;
        'label0: loop {
            if ((var1) < 144i32) as i32 != 0 {
                var2 = 0i32;
                'label1: loop {
                    if ((var2) < 160i32) as i32 != 0 {
                        let var6 = self.wasm_memory_memory_getRgbPixelStart(imports, var2, var1);
                        var3 = var6;
                        var0 = 0i32;
                        'label2: loop {
                            if ((var0) < 3i32) as i32 != 0 {
                                let var7 = self.memory.load8(93184i32.wrapping_add(var3).wrapping_add(var0) as usize) as i32;
                                self.memory.store8(162816i32.wrapping_add(var3).wrapping_add(var0) as usize, var7 as u8);
                                var0 = var0.wrapping_add(1i32);
                                continue 'label2;
                            }
                            break;
                        }
                        var2 = var2.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                var1 = var1.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_priority_clearPriorityMap<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        'label0: loop {
            if ((var0) < 144i32) as i32 != 0 {
                var1 = 0i32;
                'label1: loop {
                    if ((var1) < 160i32) as i32 != 0 {
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
    fn wasm_graphics_graphics_Graphics_MIN_CYCLES_SPRITES_LCD_MODE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 752i32;
        }
        376i32
    }
    fn wasm_graphics_graphics_Graphics_MIN_CYCLES_TRANSFER_DATA_LCD_MODE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global43;
        if var0 != 0 {
            return 498i32;
        }
        249i32
    }
    fn wasm_interrupts_index_requestLcdInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_interrupts_index__requestInterrupt(imports, 1i32);
    }
    fn wasm_memory_dma_updateHblankHdma<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global107;
        if (var3 == 0) as i32 != 0 {
            return;
        }
        let var4 = self.wasm_memory_dma_getHdmaSource(imports);
        var1 = var4;
        let var5 = self.wasm_memory_dma_getHdmaDestination(imports);
        var2 = var5;
        let var6 = self.global108;
        var0 = 16i32;
        let var7 = self.global109;
        if (var6.wrapping_add(var0) > var7) as i32 != 0 {
            let var8 = self.global109;
            let var9 = self.global108;
            var0 = var8.wrapping_sub(var9);
        }
        let var10 = self.global108;
        let var11 = self.global108;
        self.wasm_memory_dma_hdmaTransfer(imports, var1.wrapping_add(var10 & 65535i32) & 65535i32, var2.wrapping_add(var11 & 65535i32) & 65535i32, var0);
        let var12 = self.global108;
        self.global108 = var12.wrapping_add(var0);
        let var13 = self.global108;
        let var14 = self.global109;
        if (var13 >= var14) as i32 != 0 {
            self.global107 = 0i32;
            self.global108 = 0i32;
            self.global109 = 0i32;
            self.global110 = 0i32;
            self.global111 = 0i32;
            let var15 = self.global106;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var15, 255i32);
        } else {
            let var16 = self.global106;
            let var17 = self.global109;
            let var18 = self.global108;
            let var19 = self.wasm_helpers_index_setBitOnByte(imports, 7i32, (var17.wrapping_sub(var18) / 16i32).wrapping_sub(1i32) & 255i32);
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, var16, var19);
        }
    }
    fn wasm_interrupts_index_requestVBlankInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_interrupts_index__requestInterrupt(imports, 0i32);
    }
    fn wasm_graphics_lcd_setLcdStatus<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65345i32);
        var1 = var5;
        if (var0 == 0) as i32 != 0 {
            self.global120 = 0i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65348i32, 0i32);
            let var6 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var1);
            let var7 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var6);
            var1 = var7;
            self.global46 = 0i32;
            self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65345i32, var1);
            return;
        }
        var3 = var1 & 3i32;
        let var8 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65348i32);
        var4 = var8;
        let var9: i32;
        if (var4 as u32 >= 144i32 as u32) as i32 != 0 {
            let var10 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var1);
            let var11 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var10);
            var1 = var11;
            let var12 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var1);
            var2 = var12;
            var9 = 1i32;
        } else {
            let var13 = self.global120;
            let var14 = self.wasm_graphics_graphics_Graphics_MIN_CYCLES_SPRITES_LCD_MODE(imports);
            let var15: i32;
            if (var13 >= var14) as i32 != 0 {
                let var16 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var1);
                let var17 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var16);
                var1 = var17;
                let var18 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var1);
                var2 = var18;
                var15 = 2i32;
            } else {
                let var19 = self.global120;
                let var20 = self.wasm_graphics_graphics_Graphics_MIN_CYCLES_TRANSFER_DATA_LCD_MODE(imports);
                let var21: i32;
                if (var19 >= var20) as i32 != 0 {
                    let var22 = self.wasm_helpers_index_setBitOnByte(imports, 0i32, var1);
                    let var23 = self.wasm_helpers_index_setBitOnByte(imports, 1i32, var22);
                    var1 = var23;
                    var21 = 3i32;
                } else {
                    let var24 = self.wasm_helpers_index_resetBitOnByte(imports, 0i32, var1);
                    let var25 = self.wasm_helpers_index_resetBitOnByte(imports, 1i32, var24);
                    var1 = var25;
                    let var26 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var1);
                    var2 = var26;
                    var21 = 0i32;
                }
                var15 = var21;
            }
            var9 = var15;
        }
        var0 = var9;
        if (var3 != var0) as i32 != 0 {
            if var2 != 0 {
                self.wasm_interrupts_index_requestLcdInterrupt(imports);
            }
            if (var0 == 0) as i32 != 0 {
                self.wasm_memory_dma_updateHblankHdma(imports);
            }
            if (var0 == 1i32) as i32 != 0 {
                self.wasm_interrupts_index_requestVBlankInterrupt(imports);
            }
            let var27 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65349i32);
            var3 = var27;
            var2 = (var0 == 0) as i32;
            let var28: i32;
            if var2 != 0 {
                var28 = var2;
            } else {
                var28 = (var0 == 1i32) as i32;
            }
            var2 = var28 & 1i32;
            let var29: i32;
            if var2 != 0 {
                var29 = (var4 == var3) as i32;
            } else {
                var29 = var2;
            }
            if var29 & 1i32 != 0 {
                let var30 = self.wasm_helpers_index_setBitOnByte(imports, 2i32, var1);
                var1 = var30;
                let var31 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var1);
                if var31 != 0 {
                    self.wasm_interrupts_index_requestLcdInterrupt(imports);
                }
            } else {
                let var32 = self.wasm_helpers_index_resetBitOnByte(imports, 2i32, var1);
                var1 = var32;
            }
        }
        self.global46 = var0;
        self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65345i32, var1);
    }
    fn wasm_graphics_graphics_updateGraphics<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.wasm_graphics_lcd_isLcdEnabled(imports);
        var1 = var2;
        if var1 != 0 {
            let var3 = self.global120;
            self.global120 = var3.wrapping_add(var0);
            let var4 = self.global120;
            let var5 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
            if (var4 >= var5) as i32 != 0 {
                let var6 = self.global120;
                let var7 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
                self.global120 = var6.wrapping_sub(var7);
                let var8 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65348i32);
                var0 = var8;
                if (var0 == 144i32) as i32 != 0 {
                    let var9 = self.global40;
                    if var9 != 0 {
                        self.wasm_graphics_graphics__renderEntireFrame(imports);
                    } else {
                        self.wasm_graphics_graphics__drawScanline(imports, var0);
                    }
                    self.wasm_memory_memory_storeFrameToBeRendered(imports);
                    self.wasm_graphics_priority_clearPriorityMap(imports);
                } else {
                    if ((var0 as u32) < 144i32 as u32) as i32 != 0 {
                        let var10 = self.global40;
                        if (var10 == 0) as i32 != 0 {
                            self.wasm_graphics_graphics__drawScanline(imports, var0);
                        }
                    }
                }
                let var11: i32;
                if (var0 as u32 > 153i32 as u32) as i32 != 0 {
                    var11 = 0i32;
                } else {
                    var11 = var0.wrapping_add(1i32) & 255i32;
                }
                var0 = var11;
                self.wasm_memory_store_eightBitStoreIntoGBMemorySkipTraps(imports, 65348i32, var0);
            }
        }
        self.wasm_graphics_lcd_setLcdStatus(imports, var1);
    }
    fn wasm_cpu_opcodes_emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        var3 = 4i32;
        let var5 = self.global44;
        var4 = (var5 == 0) as i32;
        let var6: i32;
        if var4 != 0 {
            let var7 = self.global45;
            var6 = (var7 == 0) as i32;
        } else {
            var6 = var4;
        }
        if var6 & 1i32 != 0 {
            let var8 = self.global28;
            let var9 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var8);
            let var10 = self.global28;
            let var11 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var10.wrapping_add(1i32) & 65535i32);
            let var12 = self.global28;
            let var13 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var12.wrapping_add(2i32) & 65535i32);
            let var14 = self.wasm_cpu_opcodes_executeOpcode(imports, var9, var11, var13);
            var3 = var14;
        } else {
            let var15 = self.global44;
            let var16: i32;
            if var15 != 0 {
                let var17 = self.wasm_interrupts_index_areInterruptsEnabled(imports);
                var16 = (var17 == 0) as i32;
            } else {
                let var18 = self.global44;
                var16 = var18;
            }
            var4 = var16 & 1i32;
            let var19: i32;
            if var4 != 0 {
                let var20 = self.wasm_interrupts_index_areInterruptsPending(imports);
                var19 = var20;
            } else {
                var19 = var4;
            }
            if var19 & 1i32 != 0 {
                self.global44 = 0i32;
                self.global45 = 0i32;
                let var21 = self.global28;
                let var22 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var21);
                let var23 = self.global28;
                let var24 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var23);
                let var25 = self.global28;
                let var26 = self.wasm_memory_load_eightBitLoadFromGBMemory(imports, var25.wrapping_add(1i32) & 65535i32);
                let var27 = self.wasm_cpu_opcodes_executeOpcode(imports, var22, var24, var26);
                var3 = var27;
                let var28 = self.global28;
                self.global28 = var28.wrapping_sub(1i32) & 65535i32;
            }
        }
        let var29 = self.global21;
        self.global21 = var29 & 240i32;
        if (var3 <= 0i32) as i32 != 0 {
            return var3;
        }
        let var30 = self.global105;
        if (var30 > 0i32) as i32 != 0 {
            let var31 = self.global105;
            var3 = var3.wrapping_add(var31);
            self.global105 = 0i32;
        }
        let var32 = self.wasm_interrupts_index_checkInterrupts(imports);
        var3 = var3.wrapping_add(var32);
        let var33 = self.global45;
        if (var33 == 0) as i32 != 0 {
            if (var1 == 0) as i32 != 0 {
                self.wasm_graphics_graphics_updateGraphics(imports, var3);
            }
            if (var0 == 0) as i32 != 0 {
                self.wasm_sound_sound_updateSound(imports, var3);
            }
        }
        if (var2 == 0) as i32 != 0 {
            self.wasm_timers_index_updateTimers(imports, var3);
        }
        var3
    }
    fn wasm_graphics_graphics_Graphics_batchProcessCycles<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.wasm_graphics_graphics_Graphics_MAX_CYCLES_PER_SCANLINE(imports);
        var0
    }
    fn wasm_graphics_graphics_batchProcessGraphics<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global121;
        let var1 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
        if ((var0) < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global121;
            let var3 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
                self.wasm_graphics_graphics_updateGraphics(imports, var4);
                let var5 = self.global121;
                let var6 = self.wasm_graphics_graphics_Graphics_batchProcessCycles(imports);
                self.global121 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_cpu_opcodes_update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global37;
        var1 = var5;
        let var6 = self.global38;
        var2 = var6;
        let var7 = self.global39;
        var3 = var7;
        'label0: loop {
            var0 = (var4 == 0) as i32;
            let var8: i32;
            if var0 != 0 {
                let var9 = self.global42;
                let var10 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
                var8 = ((var9) < var10) as i32;
            } else {
                var8 = var0;
            }
            if var8 & 1i32 != 0 {
                let var11 = self.wasm_cpu_opcodes_emulationStep(imports, var1, var2, var3);
                var0 = var11;
                if (var0 >= 0i32) as i32 != 0 {
                    let var12 = self.global42;
                    self.global42 = var12.wrapping_add(var0);
                    if var1 != 0 {
                        let var13 = self.global55;
                        self.global55 = var13.wrapping_add(var0);
                    }
                    if var2 != 0 {
                        let var14 = self.global121;
                        self.global121 = var14.wrapping_add(var0);
                        self.wasm_graphics_graphics_batchProcessGraphics(imports);
                    }
                    if var3 != 0 {
                        let var15 = self.global102;
                        self.global102 = var15.wrapping_add(var0);
                        self.wasm_timers_index_batchProcessTimers(imports);
                    }
                } else {
                    var4 = 1i32;
                }
                continue 'label0;
            }
            break;
        }
        let var16 = self.global42;
        let var17 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
        if (var16 >= var17) as i32 != 0 {
            let var18 = self.global42;
            let var19 = self.wasm_cpu_cpu_Cpu_MAX_CYCLES_PER_FRAME(imports);
            self.global42 = var18.wrapping_sub(var19);
            return 1i32;
        }
        let var20 = self.global28;
        self.global28 = var20.wrapping_sub(1i32) & 65535i32;
        -1i32
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
                                        let var1 = self.global51;
                                        return var1;
                                        break;
                                    }
                                    let var2 = self.global52;
                                    return var2;
                                    break;
                                }
                                let var3 = self.global53;
                                return var3;
                                break;
                            }
                            let var4 = self.global54;
                            return var4;
                            break;
                        }
                        let var5 = self.global47;
                        return var5;
                        break;
                    }
                    let var6 = self.global48;
                    return var6;
                    break;
                }
                let var7 = self.global49;
                return var7;
                break;
            }
            let var8 = self.global50;
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
                                        self.global51 = var1;
                                        break 'label0;
                                        break;
                                    }
                                    self.global52 = var1;
                                    break 'label0;
                                    break;
                                }
                                self.global53 = var1;
                                break 'label0;
                                break;
                            }
                            self.global54 = var1;
                            break 'label0;
                            break;
                        }
                        self.global47 = var1;
                        break 'label0;
                        break;
                    }
                    self.global48 = var1;
                    break 'label0;
                    break;
                }
                self.global49 = var1;
                break 'label0;
                break;
            }
            self.global50 = var1;
            break;
        }
    }
    fn wasm_interrupts_index_requestJoypadInterrupt<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_interrupts_index__requestInterrupt(imports, 4i32);
    }
    fn wasm_joypad_index__pressJoypadButton<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        self.global45 = 0i32;
        let var4 = self.wasm_joypad_index__getJoypadButtonStateFromButtonId(imports, var0);
        if (var4 == 0) as i32 != 0 {
            var1 = 1i32;
        }
        self.wasm_joypad_index__setJoypadButtonStateFromButtonId(imports, var0, 1i32);
        if var1 != 0 {
            var1 = 0i32;
            if (var0 as u32 <= 3i32 as u32) as i32 != 0 {
                var1 = 1i32;
            }
            var0 = 0i32;
            let var5 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65280i32);
            var3 = var5;
            let var6 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var3);
            var2 = var6;
            let var7: i32;
            if var2 != 0 {
                var7 = var1;
            } else {
                var7 = var2;
            }
            if var7 & 1i32 != 0 {
                var0 = 1i32;
            }
            let var8 = self.wasm_helpers_index_checkBitOnByte(imports, 5i32, var3);
            var2 = var8;
            let var9: i32;
            if var2 != 0 {
                var9 = (var1 == 0) as i32;
            } else {
                var9 = var2;
            }
            if var9 & 1i32 != 0 {
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
        let var0 = self.global96;
        var0
    }
    fn wasm_sound_sound_resetAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global96 = 0i32;
    }
    fn wasm_debug_debug_cpu_getRegisterA<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global20;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterB<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global22;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterC<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global23;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterD<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global24;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global25;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterH<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global26;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterL<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global27;
        var0
    }
    fn wasm_debug_debug_cpu_getRegisterF<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global21;
        var0
    }
    fn wasm_debug_debug_cpu_getProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global28;
        var0
    }
    fn wasm_debug_debug_cpu_getStackPointer<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global29;
        var0
    }
    fn wasm_debug_debug_cpu_getOpcodeAtProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global28;
        let var1 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, var0);
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
        let var12 = self.wasm_memory_load_eightBitLoadFromGBMemorySkipTraps(imports, 65344i32);
        var3 = var12;
        let var13 = self.wasm_helpers_index_checkBitOnByte(imports, 4i32, var3);
        if var13 != 0 {
            var8 = 32768i32;
        }
        var9 = 38912i32;
        let var14 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var3);
        if var14 != 0 {
            var9 = 39936i32;
        }
        'label0: loop {
            if ((var6) < 256i32) as i32 != 0 {
                var3 = 0i32;
                'label1: loop {
                    if ((var3) < 256i32) as i32 != 0 {
                        var1 = var6 & 65535i32;
                        var4 = var9.wrapping_add(((var1 as u32).wrapping_shr(3i32 as u32) as i32).wrapping_mul(32i32) & 65535i32).wrapping_add(var3.wrapping_shr(3i32 as u32) & 65535i32) & 65535i32;
                        let var15 = self.wasm_memory_memory_loadFromVramBank(imports, var4, 0i32);
                        let var16 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var8, var15);
                        var10 = var16;
                        var2 = (var1 as u32).wrapping_rem(8i32 as u32) as i32;
                        var7 = 7i32.wrapping_sub(((var3 & 255i32) as u32).wrapping_rem(8i32 as u32) as i32) & 255i32;
                        var5 = 0i32;
                        let var17 = self.global16;
                        let var18: i32;
                        if var17 != 0 {
                            var18 = (var0 > 0i32) as i32;
                        } else {
                            let var19 = self.global16;
                            var18 = var19;
                        }
                        if var18 & 1i32 != 0 {
                            let var20 = self.wasm_memory_memory_loadFromVramBank(imports, var4, 1i32);
                            var5 = var20;
                        }
                        let var21 = self.wasm_helpers_index_checkBitOnByte(imports, 6i32, var5);
                        if var21 != 0 {
                            var2 = 7i32.wrapping_sub(var2) & 65535i32;
                        }
                        var4 = 0i32;
                        let var22 = self.wasm_helpers_index_checkBitOnByte(imports, 3i32, var5);
                        if var22 != 0 {
                            var4 = 1i32;
                        }
                        let var23 = self.wasm_memory_memory_loadFromVramBank(imports, var10.wrapping_add(var2.wrapping_mul(2i32) & 65535i32) & 65535i32, var4);
                        var11 = var23;
                        var1 = 0i32;
                        let var24 = self.wasm_memory_memory_loadFromVramBank(imports, var10.wrapping_add(var2.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32, var4);
                        let var25 = self.wasm_helpers_index_checkBitOnByte(imports, var7, var24);
                        if var25 != 0 {
                            var1 = 2i32;
                        }
                        let var26 = self.wasm_helpers_index_checkBitOnByte(imports, var7, var11);
                        if var26 != 0 {
                            var1 = var1.wrapping_add(1i32) & 255i32;
                        }
                        var7 = var6.wrapping_mul(256i32).wrapping_add(var3).wrapping_mul(3i32);
                        let var27 = self.global16;
                        let var28: i32;
                        if var27 != 0 {
                            var28 = (var0 > 0i32) as i32;
                        } else {
                            let var29 = self.global16;
                            var28 = var29;
                        }
                        if var28 & 1i32 != 0 {
                            let var30 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var5 & 7i32, var1, 0i32);
                            var1 = var30;
                            let var31 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var1);
                            var5 = var31;
                            let var32 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var1);
                            var4 = var32;
                            let var33 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var1);
                            var2 = var33;
                            var1 = var7.wrapping_add(232448i32);
                            self.memory.store8(var1 as usize, var5 as u8);
                            self.memory.store8(var1.wrapping_add(1i32) as usize, var4 as u8);
                            self.memory.store8(var1.wrapping_add(2i32) as usize, var2 as u8);
                        } else {
                            let var34 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var1, 65351i32, 0i32);
                            var1 = var34;
                            var2 = 0i32;
                            'label2: loop {
                                if ((var2) < 3i32) as i32 != 0 {
                                    self.memory.store8(var7.wrapping_add(232448i32).wrapping_add(var2) as usize, var1 as u8);
                                    var2 = var2.wrapping_add(1i32);
                                    continue 'label2;
                                }
                                break;
                            }
                        }
                        var3 = var3.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                var6 = var6.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_graphics_tiles_getTilePixelStart<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32) -> i32 {
        var3.wrapping_mul(8i32).wrapping_add(var1).wrapping_mul(var4.wrapping_mul(8i32)).wrapping_add(var2.wrapping_mul(8i32)).wrapping_add(var0).wrapping_mul(3i32)
    }
    fn wasm_graphics_tiles_drawLineOfTile<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32, mut var8: i32, mut var9: i32) {
        let mut var10: i32 = 0;
        let mut var11: i32 = 0;
        let mut var12: i32 = 0;
        let mut var13: i32 = 0;
        let var14 = self.wasm_graphics_renderUtils_getTileDataAddress(imports, var2, var0);
        var0 = var14;
        let var15 = self.wasm_memory_memory_loadFromVramBank(imports, var0.wrapping_add(var1.wrapping_mul(2i32) & 65535i32) & 65535i32, var3);
        var12 = var15;
        let var16 = self.wasm_memory_memory_loadFromVramBank(imports, var0.wrapping_add(var1.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32, var3);
        var13 = var16;
        'label0: loop {
            if ((var10) < 8i32) as i32 != 0 {
                var0 = 0i32;
                var2 = 7i32.wrapping_sub(((var10 & 255i32) as u32).wrapping_rem(8i32 as u32) as i32) & 255i32;
                let var17 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var13);
                if var17 != 0 {
                    var0 = 2i32;
                }
                let var18 = self.wasm_helpers_index_checkBitOnByte(imports, var2, var12);
                if var18 != 0 {
                    var0 = var0.wrapping_add(1i32) & 255i32;
                }
                let var19: i32;
                if (var9 as u32 <= 0i32 as u32) as i32 != 0 {
                    if (var8 as u32 <= 0i32 as u32) as i32 != 0 {
                        var8 = 65351i32;
                    }
                    let var20 = self.wasm_graphics_palette_getMonochromeColorFromPalette(imports, var0, var8, 1i32);
                    var2 = var20;
                    var11 = var2;
                    var0 = var2;
                    var19 = var0;
                } else {
                    let var21 = self.wasm_graphics_palette_getRgbColorFromPalette(imports, var9, var0, 0i32);
                    var2 = var21;
                    let var22 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 0i32, var2);
                    var11 = var22;
                    let var23 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 1i32, var2);
                    var0 = var23;
                    let var24 = self.wasm_graphics_palette_getColorComponentFromRgb(imports, 2i32, var2);
                    var19 = var24;
                }
                var3 = var19;
                let var25 = self.wasm_graphics_tiles_getTilePixelStart(imports, var10, var1, var4, var5, var6);
                var2 = var25;
                self.memory.store8(var7.wrapping_add(var2) as usize, var11 as u8);
                self.memory.store8(var7.wrapping_add(var2).wrapping_add(1i32) as usize, var0 as u8);
                self.memory.store8(var7.wrapping_add(var2).wrapping_add(2i32) as usize, var3 as u8);
                var10 = var10.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_debug_debug_graphics_drawTileDataToWasmMemory<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        'label0: loop {
            if ((var2 as u32) < 23i32 as u32) as i32 != 0 {
                var0 = 0i32;
                'label1: loop {
                    if ((var0 as u32) < 31i32 as u32) as i32 != 0 {
                        var4 = 0i32;
                        if (var0 as u32 > 15i32 as u32) as i32 != 0 {
                            var4 = 1i32;
                        }
                        var1 = var2;
                        if (var2 as u32 > 15i32 as u32) as i32 != 0 {
                            var1 = var1.wrapping_sub(15i32) & 255i32;
                        }
                        var1 = var1.wrapping_shl(4i32 as u32) & 255i32;
                        let var6: i32;
                        if (var0 as u32 > 15i32 as u32) as i32 != 0 {
                            var6 = var1.wrapping_add(var0.wrapping_sub(15i32) & 255i32) & 255i32;
                        } else {
                            var6 = var1.wrapping_add(var0) & 255i32;
                        }
                        var1 = var6;
                        var5 = 32768i32;
                        if (var2 as u32 > 15i32 as u32) as i32 != 0 {
                            var5 = 34816i32;
                        }
                        var3 = 0i32;
                        'label2: loop {
                            if ((var3 as u32) < 8i32 as u32) as i32 != 0 {
                                self.wasm_graphics_tiles_drawLineOfTile(imports, var1, var3, var5, var4, var0, var2, 31i32, 429056i32, 0i32, 0i32);
                                var3 = var3.wrapping_add(1i32) & 65535i32;
                                continue 'label2;
                            }
                            break;
                        }
                        var0 = var0.wrapping_add(1i32) & 255i32;
                        continue 'label1;
                    }
                    break;
                }
                var2 = var2.wrapping_add(1i32) & 255i32;
                continue 'label0;
            }
            break;
        }
    }
    fn wasm_memory_memory_getSaveStateMemoryOffset<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        var0.wrapping_add(var1.wrapping_mul(50i32) & 65535i32) & 65535i32
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
        let var1 = self.global20;
        self.memory.store8(var0 as usize, var1 as u8);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 0i32);
        let var3 = self.global22;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 0i32);
        let var5 = self.global23;
        self.memory.store8(var4 as usize, var5 as u8);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 3i32, 0i32);
        let var7 = self.global24;
        self.memory.store8(var6 as usize, var7 as u8);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 0i32);
        let var9 = self.global25;
        self.memory.store8(var8 as usize, var9 as u8);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 0i32);
        let var11 = self.global26;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 0i32);
        let var13 = self.global27;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 0i32);
        let var15 = self.global21;
        self.memory.store8(var14 as usize, var15 as u8);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 0i32);
        let var17 = self.global29;
        self.memory.store16(var16 as usize, var17 as u16);
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 0i32);
        let var19 = self.global28;
        self.memory.store16(var18 as usize, var19 as u16);
        let var20 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 12i32, 0i32);
        let var21 = self.global42;
        self.memory.store32(var20 as usize, var21 as u32);
        let var22 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 17i32, 0i32);
        let var23 = self.global44;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var22, var23);
        let var24 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 18i32, 0i32);
        let var25 = self.global45;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var24, var25);
    }
    fn wasm_graphics_graphics_Graphics_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 1i32);
        let var1 = self.global120;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 1i32);
        let var3 = self.global46;
        self.memory.store8(var2 as usize, var3 as u8);
    }
    fn wasm_interrupts_index_Interrupts_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 2i32);
        let var1 = self.global119;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 2i32);
        let var3 = self.global122;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var2, var3);
    }
    fn wasm_joypad_index_Joypad_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    fn wasm_memory_memory_Memory_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 4i32);
        let var1 = self.global14;
        self.memory.store16(var0 as usize, var1 as u16);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 4i32);
        let var3 = self.global18;
        self.memory.store16(var2 as usize, var3 as u16);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 4i32);
        let var5 = self.global98;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var4, var5);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 4i32);
        let var7 = self.global99;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var6, var7);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 4i32);
        let var9 = self.global30;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var8, var9);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 4i32);
        let var11 = self.global31;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var10, var11);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 4i32);
        let var13 = self.global32;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var12, var13);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 4i32);
        let var15 = self.global33;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var14, var15);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 4i32);
        let var17 = self.global15;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var16, var17);
    }
    fn wasm_timers_index_Timers_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 5i32);
        let var1 = self.global104;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 5i32);
        let var3 = self.global101;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 5i32);
        let var5 = self.global103;
        self.memory.store32(var4 as usize, var5 as u32);
    }
    fn wasm_sound_sound_Sound_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 6i32);
        let var1 = self.global56;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 6i32);
        let var3 = self.global94;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 6i32);
        let var5 = self.global57;
        self.memory.store8(var4 as usize, var5 as u8);
    }
    fn wasm_sound_channel1_Channel1_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 7i32);
        let var1 = self.global59;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 7i32);
        let var3 = self.global76;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 7i32);
        let var5 = self.global69;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 7i32);
        let var7 = self.global58;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 7i32);
        let var9 = self.global70;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 7i32);
        let var11 = self.global123;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 7i32);
        let var13 = self.global84;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 25i32, 7i32);
        let var15 = self.global67;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var14, var15);
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 26i32, 7i32);
        let var17 = self.global66;
        self.memory.store32(var16 as usize, var17 as u32);
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 31i32, 7i32);
        let var19 = self.global68;
        self.memory.store16(var18 as usize, var19 as u16);
    }
    fn wasm_sound_channel2_Channel2_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 8i32);
        let var1 = self.global61;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 8i32);
        let var3 = self.global78;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 8i32);
        let var5 = self.global71;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 8i32);
        let var7 = self.global60;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 8i32);
        let var9 = self.global72;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 8i32);
        let var11 = self.global124;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 8i32);
        let var13 = self.global86;
        self.memory.store8(var12 as usize, var13 as u8);
    }
    fn wasm_sound_channel3_Channel3_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 9i32);
        let var1 = self.global63;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 9i32);
        let var3 = self.global80;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 9i32);
        let var5 = self.global62;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 9i32);
        let var7 = self.global88;
        self.memory.store16(var6 as usize, var7 as u16);
    }
    fn wasm_sound_channel4_Channel4_saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 10i32);
        let var1 = self.global65;
        self.wasm_memory_store_storeBooleanDirectlyToWasmMemory(imports, var0, var1);
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 10i32);
        let var3 = self.global82;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 10i32);
        let var5 = self.global73;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 10i32);
        let var7 = self.global64;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 10i32);
        let var9 = self.global74;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 10i32);
        let var11 = self.global91;
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
        if (var1 as u32 > 0i32 as u32) as i32 != 0 {
            return 1i32;
        }
        0i32
    }
    fn wasm_cpu_cpu_Cpu_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 0i32);
        let var1 = self.memory.load8(var0 as usize) as i32;
        self.global20 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 0i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global22 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 0i32);
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global23 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 3i32, 0i32);
        let var7 = self.memory.load8(var6 as usize) as i32;
        self.global24 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 0i32);
        let var9 = self.memory.load8(var8 as usize) as i32;
        self.global25 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 0i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global26 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 0i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global27 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 0i32);
        let var15 = self.memory.load8(var14 as usize) as i32;
        self.global21 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 0i32);
        let var17 = self.memory.load16(var16 as usize) as i32;
        self.global29 = var17;
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 0i32);
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global28 = var19;
        let var20 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 12i32, 0i32);
        let var21 = self.memory.load32(var20 as usize) as i32;
        self.global42 = var21;
        let var22 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 17i32, 0i32);
        let var23 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var22);
        self.global44 = var23;
        let var24 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 18i32, 0i32);
        let var25 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var24);
        self.global45 = var25;
    }
    fn wasm_graphics_graphics_Graphics_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 1i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global120 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 1i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global46 = var3;
    }
    fn wasm_interrupts_index_Interrupts_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 2i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global119 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 2i32);
        let var3 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var2);
        self.global122 = var3;
    }
    fn wasm_memory_memory_Memory_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 4i32);
        let var1 = self.memory.load16(var0 as usize) as i32;
        self.global14 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 2i32, 4i32);
        let var3 = self.memory.load16(var2 as usize) as i32;
        self.global18 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 4i32);
        let var5 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var4);
        self.global98 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 4i32);
        let var7 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var6);
        self.global99 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 6i32, 4i32);
        let var9 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var8);
        self.global30 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 7i32, 4i32);
        let var11 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var10);
        self.global31 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 4i32);
        let var13 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var12);
        self.global32 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 4i32);
        let var15 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var14);
        self.global33 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 10i32, 4i32);
        let var17 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var16);
        self.global15 = var17;
    }
    fn wasm_timers_index_Timers_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 5i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global104 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 5i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global101 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 8i32, 5i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global103 = var5;
    }
    fn wasm_sound_sound_Sound_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 6i32);
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global56 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 4i32, 6i32);
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global94 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 6i32);
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global57 = var5;
        self.wasm_sound_sound_resetAudioQueue(imports);
    }
    fn wasm_sound_channel1_Channel1_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 7i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global59 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 7i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global76 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 7i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global69 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 7i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global58 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 7i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global70 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 7i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global123 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 7i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global84 = var13;
        let var14 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 25i32, 7i32);
        let var15 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var14);
        self.global67 = var15;
        let var16 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 26i32, 7i32);
        let var17 = self.memory.load32(var16 as usize) as i32;
        self.global66 = var17;
        let var18 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 31i32, 7i32);
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global68 = var19;
    }
    fn wasm_sound_channel2_Channel2_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 8i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global61 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 8i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global78 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 8i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global71 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 8i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global60 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 8i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global72 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 8i32);
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global124 = var11;
        let var12 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 20i32, 8i32);
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global86 = var13;
    }
    fn wasm_sound_channel3_Channel3_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 9i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global63 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 9i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global80 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 9i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global62 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 9i32);
        let var7 = self.memory.load16(var6 as usize) as i32;
        self.global88 = var7;
    }
    fn wasm_sound_channel4_Channel4_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 0i32, 10i32);
        let var1 = self.wasm_memory_load_loadBooleanDirectlyFromWasmMemory(imports, var0);
        self.global65 = var1;
        let var2 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 1i32, 10i32);
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global82 = var3;
        let var4 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 5i32, 10i32);
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global73 = var5;
        let var6 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 9i32, 10i32);
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global64 = var7;
        let var8 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 14i32, 10i32);
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global74 = var9;
        let var10 = self.wasm_memory_memory_getSaveStateMemoryOffset(imports, 19i32, 10i32);
        let var11 = self.memory.load16(var10 as usize) as i32;
        self.global91 = var11;
    }
    fn wasm_index_loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.wasm_cpu_cpu_Cpu_loadState(imports);
        self.wasm_graphics_graphics_Graphics_loadState(imports);
        self.wasm_interrupts_index_Interrupts_loadState(imports);
        self.wasm_joypad_index_Joypad_saveState(imports);
        self.wasm_memory_memory_Memory_loadState(imports);
        self.wasm_timers_index_Timers_loadState(imports);
        self.wasm_sound_sound_Sound_loadState(imports);
        self.wasm_sound_channel1_Channel1_loadState(imports);
        self.wasm_sound_channel2_Channel2_loadState(imports);
        self.wasm_sound_channel3_Channel3_loadState(imports);
        self.wasm_sound_channel4_Channel4_loadState(imports);
    }
    fn start<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.memory.size();
        if ((var0) < 139i32) as i32 != 0 {
            let var1 = self.memory.size();
            let var2 = self.memory.grow(139i32.wrapping_sub(var1) as usize);
        }
    }
}

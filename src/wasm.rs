#![allow(unreachable_code, dead_code, unused_assignments, unused_mut, unused_variables, non_snake_case, non_upper_case_globals, unused_parens)]

pub const PAGE_SIZE: usize = 64 << 10;

pub trait Imports {
    type Memory: Memory;
    fn log(&mut self, context: &mut Context<Self::Memory>, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32);
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

    fn store_slice(&mut self, addr: usize, val: &[u8]);

    fn grow(&mut self, pages: usize) -> i32;
    fn size(&mut self) -> i32;
}

pub struct Instance<I: Imports<Memory = M>, M: Memory> {
    pub imports: I,
    pub context: Context<M>,
}

pub struct Context<M: Memory> {
    pub memory: M,
    global0: i32,
    global1: i32,
    global2: i32,
    global3: i32,
    global4: i32,
    global5: i32,
    global6: i32,
    global7: i32,
    global8: i32,
    global9: i32,
    global10: i32,
    global11: i32,
    global12: i32,
    global13: i32,
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
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        let current_pages = memory.size() as usize;
        if current_pages < 136 {
            memory.grow(136 - current_pages);
            assert_eq!(memory.size(), 136, "Not enough memory pages allocated");
        }
        memory.store_slice(8861696, b"    i n i t i a l i z i n g   ( i n c l u d e B o o t R o m = $ 0 )");
        memory.store_slice(8861764, b"   O p c o d e   a t   c r a s h :   $ 0");
        let mut instance = Self {
            imports,
            context: Context {
                memory,
                global0: 0,
                global1: 0,
                global2: 0,
                global3: 0,
                global4: 0,
                global5: 0,
                global6: 0,
                global7: 0,
                global8: 0,
                global9: 0,
                global10: 1,
                global11: 0,
                global12: 0,
                global13: 0,
                global14: 0,
                global15: 0,
                global16: 0,
                global17: 1,
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
                global59: 1024,
                global60: 0,
                global61: 0,
                global62: 0,
                global63: 0,
                global64: 0,
                global65: 0,
                global66: 0,
                global67: 0,
                global68: 0,
            },
        };
        instance
    }
    pub fn initialize(&mut self, var0: i32) {
        self.context.func52(&mut self.imports, var0)
    }
    pub fn update(&mut self) -> i32 {
        self.context.func144(&mut self.imports)
    }
    pub fn emulationStep(&mut self) -> i32 {
        self.context.func143(&mut self.imports)
    }
    pub fn areInterruptsEnabled(&mut self) -> i32 {
        self.context.func95(&mut self.imports)
    }
    pub fn setJoypadState(&mut self, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.context.func150(&mut self.imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex(&mut self) -> i32 {
        self.context.func151(&mut self.imports)
    }
    pub fn resetAudioQueue(&mut self) {
        self.context.func141(&mut self.imports)
    }
    pub fn saveState(&mut self) {
        self.context.func165(&mut self.imports)
    }
    pub fn loadState(&mut self) {
        self.context.func177(&mut self.imports)
    }
    pub fn getRegisterA(&mut self) -> i32 {
        self.context.func178(&mut self.imports)
    }
    pub fn getRegisterB(&mut self) -> i32 {
        self.context.func179(&mut self.imports)
    }
    pub fn getRegisterC(&mut self) -> i32 {
        self.context.func180(&mut self.imports)
    }
    pub fn getRegisterD(&mut self) -> i32 {
        self.context.func181(&mut self.imports)
    }
    pub fn getRegisterE(&mut self) -> i32 {
        self.context.func182(&mut self.imports)
    }
    pub fn getRegisterH(&mut self) -> i32 {
        self.context.func183(&mut self.imports)
    }
    pub fn getRegisterL(&mut self) -> i32 {
        self.context.func184(&mut self.imports)
    }
    pub fn getRegisterF(&mut self) -> i32 {
        self.context.func185(&mut self.imports)
    }
    pub fn getProgramCounter(&mut self) -> i32 {
        self.context.func186(&mut self.imports)
    }
    pub fn getStackPointer(&mut self) -> i32 {
        self.context.func187(&mut self.imports)
    }
    pub fn getPreviousOpcode(&mut self) -> i32 {
        self.context.func188(&mut self.imports)
    }
    pub fn getOpcodeAtProgramCounter(&mut self) -> i32 {
        self.context.func189(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32) {
        self.func52(imports, var0)
    }
    pub fn update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func144(imports)
    }
    pub fn emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func143(imports)
    }
    pub fn areInterruptsEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func95(imports)
    }
    pub fn setJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.func150(imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func151(imports)
    }
    pub fn resetAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func141(imports)
    }
    pub fn saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func165(imports)
    }
    pub fn loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func177(imports)
    }
    pub fn getRegisterA<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func178(imports)
    }
    pub fn getRegisterB<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func179(imports)
    }
    pub fn getRegisterC<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func180(imports)
    }
    pub fn getRegisterD<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func181(imports)
    }
    pub fn getRegisterE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func182(imports)
    }
    pub fn getRegisterH<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func183(imports)
    }
    pub fn getRegisterL<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func184(imports)
    }
    pub fn getRegisterF<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func185(imports)
    }
    pub fn getProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func186(imports)
    }
    pub fn getStackPointer<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func187(imports)
    }
    pub fn getPreviousOpcode<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func188(imports)
    }
    pub fn getOpcodeAtProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func189(imports)
    }
    // wasm/helpers/index/log
    fn func1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32) {
        let var8 = var0;
        let var9 = var1;
        let var10 = var2;
        let var11 = var3;
        let var12 = var4;
        let var13 = var5;
        let var14 = var6;
        let var15 = var7;
        imports.log(self, var8, var9, var10, var11, var12, var13, var14, var15);
    }
    // wasm/helpers/index/checkBitOnByte
    fn func2<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        (var2 & 1i32.wrapping_shl(var3 as u32) != 0i32) as i32
    }
    // wasm/memory/banking/handleBanking
    fn func3<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = self.global10;
        if var3 != 0 {
            return;
        }
        let var4 = var0;
        if (var4 as u32 <= 8191i32 as u32) as i32 != 0 {
            let var5 = self.global11;
            let var6: i32;
            if var5 != 0 {
                let var7 = var1;
                let var8 = self.func2(imports, 4i32, var7 & 255i32); // wasm/helpers/index/checkBitOnByte
                var6 = (var8 == 0) as i32;
            } else {
                let var9 = self.global11;
                var6 = var9;
            }
            if (var6 & 1i32 == 0) as i32 != 0 {
                let var10 = var1;
                var2 = var10 & 15i32;
                let var11 = var2;
                if var11 != 0 {
                    let var12 = var2;
                    if (var12 == 10i32) as i32 != 0 {
                        self.global12 = 1i32;
                    }
                } else {
                    self.global12 = 0i32;
                }
            }
        } else {
            let var13 = var0;
            if (var13 as u32 <= 16383i32 as u32) as i32 != 0 {
                let var14 = self.global13;
                var2 = (var14 == 0) as i32;
                let var15 = var2;
                let var16: i32;
                if var15 != 0 {
                    let var17 = var2;
                    var16 = var17;
                } else {
                    let var18 = var0;
                    var16 = (var18 as u32 <= 12287i32 as u32) as i32;
                }
                if var16 & 1i32 != 0 {
                    let var19 = self.global11;
                    if var19 != 0 {
                        let var20 = var1;
                        self.global14 = var20 & 15i32;
                    }
                    let var21 = var1;
                    var2 = var21;
                    let var22 = self.global15;
                    if var22 != 0 {
                        let var23 = var2;
                        var2 = var23 & 31i32;
                        let var24 = self.global14;
                        self.global14 = var24 & 224i32;
                    } else {
                        let var25 = self.global16;
                        if var25 != 0 {
                            let var26 = var2;
                            var2 = var26 & 127i32;
                            let var27 = self.global14;
                            self.global14 = var27 & 128i32;
                        } else {
                            let var28 = self.global13;
                            if var28 != 0 {
                                let var29 = self.global14;
                                self.global14 = var29 & 0i32;
                            }
                        }
                    }
                    let var30 = self.global14;
                    let var31 = var2;
                    self.global14 = (var30 | var31) & 65535i32;
                }
            } else {
                let var32 = self.global11;
                var2 = (var32 == 0) as i32;
                let var33 = var2;
                let var34: i32;
                if var33 != 0 {
                    let var35 = var0;
                    var34 = (var35 as u32 <= 24575i32 as u32) as i32;
                } else {
                    let var36 = var2;
                    var34 = var36;
                }
                if var34 & 1i32 != 0 {
                    let var37 = self.global15;
                    let var38: i32;
                    if var37 != 0 {
                        let var39 = self.global17;
                        var38 = var39;
                    } else {
                        let var40 = self.global15;
                        var38 = var40;
                    }
                    if var38 & 1i32 != 0 {
                        let var41 = self.global14;
                        self.global14 = var41 & 31i32;
                        let var42 = self.global14;
                        let var43 = var1;
                        self.global14 = (var42 | var43 & 224i32) & 65535i32;
                        return;
                    }
                    let var44 = self.global16;
                    if var44 != 0 {
                        let var45 = var1;
                        var2 = (var45 as u32 >= 8i32 as u32) as i32;
                        let var46 = var2;
                        let var47: i32;
                        if var46 != 0 {
                            let var48 = var1;
                            var47 = (var48 as u32 <= 12i32 as u32) as i32;
                        } else {
                            let var49 = var2;
                            var47 = var49;
                        }
                    }
                    let var50 = var1;
                    var2 = var50;
                    let var51 = self.global13;
                    let var52: i32;
                    if var51 != 0 {
                        let var53 = var2;
                        var52 = var53 & 15i32;
                    } else {
                        let var54 = var2;
                        var52 = var54 & 3i32;
                    }
                    var2 = var52;
                    let var55 = var2;
                    self.global18 = var55;
                } else {
                    let var56 = self.global11;
                    var2 = (var56 == 0) as i32;
                    let var57 = var2;
                    let var58: i32;
                    if var57 != 0 {
                        let var59 = var0;
                        var58 = (var59 as u32 <= 32767i32 as u32) as i32;
                    } else {
                        let var60 = var2;
                        var58 = var60;
                    }
                    if var58 & 1i32 != 0 {
                        let var61 = self.global15;
                        if var61 != 0 {
                            let var62 = var1;
                            let var63 = self.func2(imports, 0i32, var62 & 255i32); // wasm/helpers/index/checkBitOnByte
                            if var63 != 0 {
                                self.global17 = 1i32;
                            } else {
                                self.global17 = 0i32;
                            }
                        }
                    }
                }
            }
        }
    }
    // wasm/memory/banking/getRomBankAddress
    fn func4<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global14;
        var1 = var3;
        let var4 = self.global13;
        var2 = (var4 == 0) as i32;
        let var5 = var2;
        let var6: i32;
        if var5 != 0 {
            let var7 = var1;
            var6 = (var7 == 0) as i32;
        } else {
            let var8 = var2;
            var6 = var8;
        }
        if var6 & 1i32 != 0 {
            var1 = 1i32;
        }
        let var9 = var1;
        let var10 = var0;
        var9.wrapping_mul(16384i32).wrapping_add(var10.wrapping_sub(16384i32))
    }
    // wasm/memory/banking/getRamBankAddress
    fn func5<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = self.global18;
        let var2 = var0;
        var1.wrapping_mul(8192i32).wrapping_add(var2.wrapping_sub(40960i32))
    }
    // wasm/memory/memoryMap/getWasmBoyOffsetFromGameBoyOffset
    fn func6<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if ((var1 as u32) < 16384i32 as u32) as i32 != 0 {
            let var3 = var0;
            var2 = var3.wrapping_add(473088i32);
        } else {
            let var4 = var0;
            let var5: i32;
            if ((var4 as u32) < 32768i32 as u32) as i32 != 0 {
                let var6 = var0;
                let var7 = self.func4(imports, var6); // wasm/memory/banking/getRomBankAddress
                var5 = var7.wrapping_add(473088i32);
            } else {
                let var8 = var0;
                let var9: i32;
                if ((var8 as u32) < 40960i32 as u32) as i32 != 0 {
                    let var10 = var0;
                    var9 = var10.wrapping_add(-31744i32);
                } else {
                    let var11 = var0;
                    let var12: i32;
                    if ((var11 as u32) < 49152i32 as u32) as i32 != 0 {
                        let var13 = var0;
                        let var14 = self.func5(imports, var13); // wasm/memory/banking/getRamBankAddress
                        var12 = var14.wrapping_add(33792i32);
                    } else {
                        let var15 = var0;
                        var12 = var15.wrapping_add(-31744i32);
                    }
                    var9 = var12;
                }
                var5 = var9;
            }
            var2 = var5;
        }
        var2
    }
    // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
    fn func7<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = self.func6(imports, var2); // wasm/memory/memoryMap/getWasmBoyOffsetFromGameBoyOffset
        let var4 = var1;
        self.memory.store8(var3 as usize, var4 as u8);
    }
    // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    fn func8<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = var1;
        self.func7(imports, var2, var3); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
    }
    // wasm/helpers/index/splitHighByte
    fn func9<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        ((var1 & 65280i32) as u32).wrapping_shr(8i32 as u32) as i32
    }
    // wasm/helpers/index/splitLowByte
    fn func10<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        var1 & 255i32
    }
    // wasm/memory/store/sixteenBitStoreIntoGBMemorySkipTraps
    fn func11<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        let var4 = self.func9(imports, var3); // wasm/helpers/index/splitHighByte
        var2 = var4;
        let var5 = var0;
        let var6 = var1;
        let var7 = self.func10(imports, var6); // wasm/helpers/index/splitLowByte
        self.func7(imports, var5, var7); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        let var8 = var0;
        let var9 = var2;
        self.func7(imports, var8.wrapping_add(1i32) & 65535i32, var9); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
    }
    // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
    fn func12<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func6(imports, var1); // wasm/memory/memoryMap/getWasmBoyOffsetFromGameBoyOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        var3
    }
    // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
    fn func13<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func12(imports, var1); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
        var2
    }
    // wasm/helpers/index/resetBitOnByte
    fn func14<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        var2 & (1i32.wrapping_shl(var3 as u32) & 255i32 ^ -1i32) & 255i32
    }
    // wasm/helpers/index/setBitOnByte
    fn func15<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        (var2 | 1i32.wrapping_shl(var3 as u32) & 255i32) & 255i32
    }
    // wasm/joypad/index/getJoypadState
    fn func16<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.func13(imports, 65280i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var0 = (var1 ^ 255i32) & 255i32;
        let var2 = var0;
        let var3 = self.func2(imports, 4i32, var2); // wasm/helpers/index/checkBitOnByte
        if var3 != 0 {
            let var4 = var0;
            let var5 = self.func2(imports, 5i32, var4); // wasm/helpers/index/checkBitOnByte
            if (var5 == 0) as i32 != 0 {
                let var6 = var0;
                var0 = (var6 | 240i32) & 255i32;
                let var7 = self.global24;
                let var8: i32;
                if var7 != 0 {
                    let var9 = var0;
                    let var10 = self.func14(imports, 2i32, var9); // wasm/helpers/index/resetBitOnByte
                    var8 = var10;
                } else {
                    let var11 = var0;
                    let var12 = self.func15(imports, 2i32, var11); // wasm/helpers/index/setBitOnByte
                    var8 = var12;
                }
                var0 = var8;
                let var13 = self.global25;
                let var14: i32;
                if var13 != 0 {
                    let var15 = var0;
                    let var16 = self.func14(imports, 0i32, var15); // wasm/helpers/index/resetBitOnByte
                    var14 = var16;
                } else {
                    let var17 = var0;
                    let var18 = self.func15(imports, 0i32, var17); // wasm/helpers/index/setBitOnByte
                    var14 = var18;
                }
                var0 = var14;
                let var19 = self.global26;
                let var20: i32;
                if var19 != 0 {
                    let var21 = var0;
                    let var22 = self.func14(imports, 3i32, var21); // wasm/helpers/index/resetBitOnByte
                    var20 = var22;
                } else {
                    let var23 = var0;
                    let var24 = self.func15(imports, 3i32, var23); // wasm/helpers/index/setBitOnByte
                    var20 = var24;
                }
                var0 = var20;
                let var25 = self.global27;
                let var26: i32;
                if var25 != 0 {
                    let var27 = var0;
                    let var28 = self.func14(imports, 1i32, var27); // wasm/helpers/index/resetBitOnByte
                    var26 = var28;
                } else {
                    let var29 = var0;
                    let var30 = self.func15(imports, 1i32, var29); // wasm/helpers/index/setBitOnByte
                    var26 = var30;
                }
                var0 = var26;
            }
        } else {
            let var31 = var0;
            var0 = (var31 | 240i32) & 255i32;
            let var32 = self.global20;
            let var33: i32;
            if var32 != 0 {
                let var34 = var0;
                let var35 = self.func14(imports, 0i32, var34); // wasm/helpers/index/resetBitOnByte
                var33 = var35;
            } else {
                let var36 = var0;
                let var37 = self.func15(imports, 0i32, var36); // wasm/helpers/index/setBitOnByte
                var33 = var37;
            }
            var0 = var33;
            let var38 = self.global21;
            let var39: i32;
            if var38 != 0 {
                let var40 = var0;
                let var41 = self.func14(imports, 1i32, var40); // wasm/helpers/index/resetBitOnByte
                var39 = var41;
            } else {
                let var42 = var0;
                let var43 = self.func15(imports, 1i32, var42); // wasm/helpers/index/setBitOnByte
                var39 = var43;
            }
            var0 = var39;
            let var44 = self.global22;
            let var45: i32;
            if var44 != 0 {
                let var46 = var0;
                let var47 = self.func14(imports, 2i32, var46); // wasm/helpers/index/resetBitOnByte
                var45 = var47;
            } else {
                let var48 = var0;
                let var49 = self.func15(imports, 2i32, var48); // wasm/helpers/index/setBitOnByte
                var45 = var49;
            }
            var0 = var45;
            let var50 = self.global23;
            let var51: i32;
            if var50 != 0 {
                let var52 = var0;
                let var53 = self.func14(imports, 3i32, var52); // wasm/helpers/index/resetBitOnByte
                var51 = var53;
            } else {
                let var54 = var0;
                let var55 = self.func15(imports, 3i32, var54); // wasm/helpers/index/setBitOnByte
                var51 = var55;
            }
            var0 = var51;
        }
        let var56 = var0;
        var56
    }
    // wasm/memory/readTraps/checkReadTraps
    fn func17<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        var1 = 32768i32;
        let var3 = var1;
        if ((var2 as u32) < var3 as u32) as i32 != 0 {
            return -1i32;
        }
        let var4 = var0;
        var1 = (var4 as u32 >= 32768i32 as u32) as i32;
        let var5 = var1;
        let var6: i32;
        if var5 != 0 {
            let var7 = var0;
            var6 = ((var7 as u32) < 40960i32 as u32) as i32;
        } else {
            let var8 = var1;
            var6 = var8;
        }
        if var6 & 1i32 != 0 {
            let var9 = self.global19;
            if (var9 as u32 > 2i32 as u32) as i32 != 0 {
                return 255i32;
            }
        }
        let var10 = var0;
        var1 = (var10 as u32 >= 65024i32 as u32) as i32;
        let var11 = var1;
        let var12: i32;
        if var11 != 0 {
            let var13 = var0;
            var12 = (var13 as u32 <= 65183i32 as u32) as i32;
        } else {
            let var14 = var1;
            var12 = var14;
        }
        if var12 & 1i32 != 0 {
            let var15 = self.global19;
            if (var15 != 2i32) as i32 != 0 {
                return 255i32;
            }
        }
        let var16 = var0;
        if (var16 == 65280i32) as i32 != 0 {
            let var17 = self.func16(imports); // wasm/joypad/index/getJoypadState
            return var17;
        }
        -1i32
    }
    // wasm/memory/load/eightBitLoadFromGBMemory
    fn func18<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        let var3 = self.func17(imports, var2); // wasm/memory/readTraps/checkReadTraps
        var1 = var3;
        let var4 = var1;
        let var5: i32;
        if ((var4) < 0i32) as i32 != 0 {
            let var6 = var0;
            let var7 = self.func12(imports, var6); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
            var5 = var7;
        } else {
            let var8 = var1;
            var5 = var8 & 255i32;
        }
        var5
    }
    // wasm/sound/registers/getRegister1OfChannel
    fn func19<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 == 1i32) as i32 != 0 {
            let var3 = self.func18(imports, 65297i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = var3;
        } else {
            let var4 = var0;
            let var5: i32;
            if (var4 == 2i32) as i32 != 0 {
                let var6 = self.func18(imports, 65302i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var5 = var6;
            } else {
                let var7 = var0;
                let var8: i32;
                if (var7 == 3i32) as i32 != 0 {
                    let var9 = self.func18(imports, 65307i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var9;
                } else {
                    let var10 = self.func18(imports, 65312i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var10;
                }
                var5 = var8;
            }
            var2 = var5;
        }
        var2
    }
    // wasm/sound/length/setChannelLengthCounter
    fn func20<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func19(imports, var3); // wasm/sound/registers/getRegister1OfChannel
        var1 = var4 & 63i32;
        var2 = 64i32;
        let var5 = var0;
        let var6: i32;
        if (var5 == 3i32) as i32 != 0 {
            let var7 = var1;
            var6 = (255i32.wrapping_sub(var7) & 255i32).wrapping_add(1i32) & 255i32;
        } else {
            let var8 = var1;
            var6 = 64i32.wrapping_sub(var8) & 255i32;
        }
        var1 = var6;
        let var9 = var0;
        if (var9 == 1i32) as i32 != 0 {
            let var10 = var1;
            self.global28 = var10;
        } else {
            let var11 = var0;
            if (var11 == 2i32) as i32 != 0 {
                let var12 = var1;
                self.global29 = var12;
            } else {
                let var13 = var0;
                if (var13 == 3i32) as i32 != 0 {
                    let var14 = var1;
                    self.global30 = var14;
                } else {
                    let var15 = var0;
                    if (var15 == 4i32) as i32 != 0 {
                        let var16 = var1;
                        self.global31 = var16;
                    }
                }
            }
        }
    }
    // wasm/sound/registers/getRegister4OfChannel
    fn func21<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 == 1i32) as i32 != 0 {
            let var3 = self.func18(imports, 65300i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = var3;
        } else {
            let var4 = var0;
            let var5: i32;
            if (var4 == 2i32) as i32 != 0 {
                let var6 = self.func18(imports, 65305i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var5 = var6;
            } else {
                let var7 = var0;
                let var8: i32;
                if (var7 == 3i32) as i32 != 0 {
                    let var9 = self.func18(imports, 65310i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var9;
                } else {
                    let var10 = self.func18(imports, 65315i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var10;
                }
                var5 = var8;
            }
            var2 = var5;
        }
        var2
    }
    // wasm/sound/registers/getRegister3OfChannel
    fn func22<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 == 1i32) as i32 != 0 {
            let var3 = self.func18(imports, 65299i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = var3;
        } else {
            let var4 = var0;
            let var5: i32;
            if (var4 == 2i32) as i32 != 0 {
                let var6 = self.func18(imports, 65304i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var5 = var6;
            } else {
                let var7 = var0;
                let var8: i32;
                if (var7 == 3i32) as i32 != 0 {
                    let var9 = self.func18(imports, 65309i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var9;
                } else {
                    let var10 = self.func18(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var10;
                }
                var5 = var8;
            }
            var2 = var5;
        }
        var2
    }
    // wasm/sound/frequency/getChannelFrequency
    fn func23<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func21(imports, var1); // wasm/sound/registers/getRegister4OfChannel
        let var3 = var0;
        let var4 = self.func22(imports, var3); // wasm/sound/registers/getRegister3OfChannel
        ((var2 & 7i32).wrapping_shl(8i32 as u32) | var4) & 65535i32
    }
    // wasm/sound/registers/getRegister2OfChannel
    fn func24<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 == 1i32) as i32 != 0 {
            let var3 = self.func18(imports, 65298i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = var3;
        } else {
            let var4 = var0;
            let var5: i32;
            if (var4 == 2i32) as i32 != 0 {
                let var6 = self.func18(imports, 65303i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var5 = var6;
            } else {
                let var7 = var0;
                let var8: i32;
                if (var7 == 3i32) as i32 != 0 {
                    let var9 = self.func18(imports, 65308i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var9;
                } else {
                    let var10 = self.func18(imports, 65313i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    var8 = var10;
                }
                var5 = var8;
            }
            var2 = var5;
        }
        var2
    }
    // wasm/sound/envelope/getChannelEnvelopePeriod
    fn func25<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func24(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        var2 & 7i32
    }
    // wasm/sound/registers/getChannelStartingVolume
    fn func26<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func24(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        (var2 as u32).wrapping_shr(4i32 as u32) as i32 & 15i32
    }
    // wasm/sound/channel1/getSweepPeriod
    fn func27<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemory
        ((var0 & 112i32) as u32).wrapping_shr(4i32 as u32) as i32
    }
    // wasm/sound/channel1/getSweepShift
    fn func28<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var0 & 7i32
    }
    // wasm/sound/channel1/getNewFrequencyFromSweep
    fn func29<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global36;
        let var2 = self.func28(imports); // wasm/sound/channel1/getSweepShift
        var0 = (var1 as u32).wrapping_shr(var2 as u32) as i32;
        let var3 = self.func18(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var4 = self.func2(imports, 3i32, var3); // wasm/helpers/index/checkBitOnByte
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global36;
            let var7 = var0;
            var5 = var6.wrapping_sub(var7) & 65535i32;
        } else {
            let var8 = self.global36;
            let var9 = var0;
            var5 = var8.wrapping_add(var9) & 65535i32;
        }
        var0 = var5;
        let var10 = var0;
        var10
    }
    // wasm/sound/registers/setRegister3OfChannel
    fn func30<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        if (var2 == 1i32) as i32 != 0 {
            let var3 = var1;
            self.func8(imports, 65299i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        } else {
            let var4 = var0;
            if (var4 == 2i32) as i32 != 0 {
                let var5 = var1;
                self.func8(imports, 65304i32, var5); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            } else {
                let var6 = var0;
                if (var6 == 3i32) as i32 != 0 {
                    let var7 = var1;
                    self.func8(imports, 65309i32, var7); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                } else {
                    let var8 = var1;
                    self.func8(imports, 65314i32, var8); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                }
            }
        }
    }
    // wasm/sound/registers/setRegister4OfChannel
    fn func31<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        if (var2 == 1i32) as i32 != 0 {
            let var3 = var1;
            self.func8(imports, 65300i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        } else {
            let var4 = var0;
            if (var4 == 2i32) as i32 != 0 {
                let var5 = var1;
                self.func8(imports, 65305i32, var5); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            } else {
                let var6 = var0;
                if (var6 == 3i32) as i32 != 0 {
                    let var7 = var1;
                    self.func8(imports, 65310i32, var7); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                } else {
                    let var8 = var1;
                    self.func8(imports, 65315i32, var8); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                }
            }
        }
    }
    // wasm/sound/frequency/setChannelFrequency
    fn func32<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func21(imports, var3); // wasm/sound/registers/getRegister4OfChannel
        let var5 = var1;
        var2 = var4 & 248i32 | (var5 as u32).wrapping_shr(8i32 as u32) as i32 & 255i32;
        let var6 = var0;
        let var7 = var1;
        self.func30(imports, var6, var7 & 255i32); // wasm/sound/registers/setRegister3OfChannel
        let var8 = var0;
        let var9 = var2;
        self.func31(imports, var8, var9); // wasm/sound/registers/setRegister4OfChannel
    }
    // wasm/sound/channel1/calculateSweepAndCheckOverflow
    fn func33<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func29(imports); // wasm/sound/channel1/getNewFrequencyFromSweep
        var0 = var2;
        let var3 = var0;
        var1 = (var3 as u32 <= 2047i32 as u32) as i32;
        let var4 = var1;
        let var5: i32;
        if var4 != 0 {
            let var6 = self.func28(imports); // wasm/sound/channel1/getSweepShift
            var5 = (var6 as u32 > 0i32 as u32) as i32;
        } else {
            let var7 = var1;
            var5 = var7;
        }
        if var5 & 1i32 != 0 {
            let var8 = var0;
            self.global36 = var8;
            let var9 = var0;
            self.func32(imports, 1i32, var9); // wasm/sound/frequency/setChannelFrequency
            let var10 = self.func29(imports); // wasm/sound/channel1/getNewFrequencyFromSweep
            var0 = var10;
        }
        let var11 = var0;
        if (var11 as u32 > 2047i32 as u32) as i32 != 0 {
            self.global32 = 0i32;
        }
    }
    // wasm/sound/registers/isChannelDacEnabled
    fn func34<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 != 3i32) as i32 != 0 {
            let var3 = var0;
            let var4 = self.func24(imports, var3); // wasm/sound/registers/getRegister2OfChannel
            let var5: i32;
            if ((var4 & 248i32) as u32 > 0i32 as u32) as i32 != 0 {
                var5 = 1i32;
            } else {
                var5 = 0i32;
            }
            var2 = var5;
        } else {
            let var6 = self.func18(imports, 65306i32); // wasm/memory/load/eightBitLoadFromGBMemory
            let var7 = self.func2(imports, 7i32, var6); // wasm/helpers/index/checkBitOnByte
            var2 = var7;
        }
        var2
    }
    // wasm/sound/channel1/Channel1.trigger
    fn func35<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        self.global32 = 1i32;
        let var1 = self.global28;
        if (var1 == 0) as i32 != 0 {
            self.global28 = 64i32;
        }
        let var2 = self.func23(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
        self.global33 = 2048i32.wrapping_sub(var2).wrapping_mul(4i32);
        let var3 = self.func25(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global34 = var3;
        let var4 = self.func26(imports, 1i32); // wasm/sound/registers/getChannelStartingVolume
        self.global35 = var4;
        let var5 = self.func23(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
        self.global36 = var5;
        let var6 = self.func27(imports); // wasm/sound/channel1/getSweepPeriod
        self.global37 = var6;
        let var7 = self.func27(imports); // wasm/sound/channel1/getSweepPeriod
        var0 = (var7 as u32 > 0i32 as u32) as i32;
        let var8 = var0;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.func28(imports); // wasm/sound/channel1/getSweepShift
            var9 = (var10 as u32 > 0i32 as u32) as i32;
        } else {
            let var11 = var0;
            var9 = var11;
        }
        if var9 & 1i32 != 0 {
            self.global38 = 1i32;
        } else {
            self.global38 = 0i32;
        }
        let var12 = self.func28(imports); // wasm/sound/channel1/getSweepShift
        if (var12 as u32 > 0i32 as u32) as i32 != 0 {
            self.func33(imports); // wasm/sound/channel1/calculateSweepAndCheckOverflow
        }
        let var13 = self.func34(imports, 1i32); // wasm/sound/registers/isChannelDacEnabled
        if (var13 == 0) as i32 != 0 {
            self.global32 = 0i32;
        }
    }
    // wasm/sound/channel2/Channel2.trigger
    fn func36<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global39 = 1i32;
        let var0 = self.global29;
        if (var0 == 0) as i32 != 0 {
            self.global29 = 64i32;
        }
        let var1 = self.func23(imports, 2i32); // wasm/sound/frequency/getChannelFrequency
        self.global40 = 2048i32.wrapping_sub(var1).wrapping_mul(4i32);
        let var2 = self.func25(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global41 = var2;
        let var3 = self.func26(imports, 2i32); // wasm/sound/registers/getChannelStartingVolume
        self.global42 = var3;
        let var4 = self.func34(imports, 2i32); // wasm/sound/registers/isChannelDacEnabled
        if (var4 == 0) as i32 != 0 {
            self.global39 = 0i32;
        }
    }
    // wasm/sound/channel3/Channel3.trigger
    fn func37<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global43 = 1i32;
        let var0 = self.global30;
        if (var0 == 0) as i32 != 0 {
            self.global30 = 256i32;
        }
        let var1 = self.func23(imports, 3i32); // wasm/sound/frequency/getChannelFrequency
        self.global44 = 2048i32.wrapping_sub(var1).wrapping_mul(2i32);
        self.global45 = 0i32;
        let var2 = self.func34(imports, 3i32); // wasm/sound/registers/isChannelDacEnabled
        if (var2 == 0) as i32 != 0 {
            self.global43 = 0i32;
        }
    }
    // wasm/sound/channel4/Channel4.getNoiseChannelDivisorFromDivisorCode
    fn func38<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func18(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var1 = var2 & 7i32;
        let var3 = var1;
        if var3 != 0 {
            let var4 = var1;
            if (var4 == 1i32) as i32 != 0 {
                var0 = 16i32;
            } else {
                let var5 = var1;
                if (var5 == 2i32) as i32 != 0 {
                    var0 = 32i32;
                } else {
                    let var6 = var1;
                    if (var6 == 3i32) as i32 != 0 {
                        var0 = 48i32;
                    } else {
                        let var7 = var1;
                        if (var7 == 4i32) as i32 != 0 {
                            var0 = 64i32;
                        } else {
                            let var8 = var1;
                            if (var8 == 5i32) as i32 != 0 {
                                var0 = 80i32;
                            } else {
                                let var9 = var1;
                                if (var9 == 6i32) as i32 != 0 {
                                    var0 = 96i32;
                                } else {
                                    let var10 = var1;
                                    if (var10 == 7i32) as i32 != 0 {
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
        let var11 = var0;
        var11
    }
    // wasm/sound/channel4/Channel4.getNoiseChannelClockShift
    fn func39<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemory
        (var0 as u32).wrapping_shr(4i32 as u32) as i32
    }
    // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
    fn func40<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func38(imports); // wasm/sound/channel4/Channel4.getNoiseChannelDivisorFromDivisorCode
        let var1 = self.func39(imports); // wasm/sound/channel4/Channel4.getNoiseChannelClockShift
        var0.wrapping_shl(var1 as u32) & 65535i32
    }
    // wasm/sound/channel4/Channel4.trigger
    fn func41<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global46 = 1i32;
        let var0 = self.global31;
        if (var0 == 0) as i32 != 0 {
            self.global31 = 64i32;
        }
        let var1 = self.func40(imports); // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
        self.global47 = var1;
        let var2 = self.func25(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global48 = var2;
        let var3 = self.func26(imports, 4i32); // wasm/sound/registers/getChannelStartingVolume
        self.global49 = var3;
        self.global50 = 32767i32;
        let var4 = self.func34(imports, 4i32); // wasm/sound/registers/isChannelDacEnabled
        if (var4 == 0) as i32 != 0 {
            self.global46 = 0i32;
        }
    }
    // wasm/sound/registers/handledWriteToSoundRegister
    fn func42<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.func18(imports, 65318i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var3 = var4;
        let var5 = var0;
        var2 = (var5 != 65318i32) as i32;
        let var6 = var2;
        let var7: i32;
        if var6 != 0 {
            let var8 = var3;
            let var9 = self.func2(imports, 7i32, var8); // wasm/helpers/index/checkBitOnByte
            var7 = (var9 == 0) as i32;
        } else {
            let var10 = var2;
            var7 = var10;
        }
        if var7 & 1i32 != 0 {
            return 1i32;
        }
        let var11 = var0;
        if (var11 == 65297i32) as i32 != 0 {
            let var12 = var0;
            let var13 = var1;
            self.func8(imports, var12, var13 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            self.func20(imports, 1i32); // wasm/sound/length/setChannelLengthCounter
            return 1i32;
        } else {
            let var14 = var0;
            if (var14 == 65302i32) as i32 != 0 {
                let var15 = var0;
                let var16 = var1;
                self.func8(imports, var15, var16 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                self.func20(imports, 2i32); // wasm/sound/length/setChannelLengthCounter
                return 1i32;
            } else {
                let var17 = var0;
                if (var17 == 65307i32) as i32 != 0 {
                    let var18 = var0;
                    let var19 = var1;
                    self.func8(imports, var18, var19 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func20(imports, 3i32); // wasm/sound/length/setChannelLengthCounter
                    return 1i32;
                } else {
                    let var20 = var0;
                    if (var20 == 65312i32) as i32 != 0 {
                        let var21 = var0;
                        let var22 = var1;
                        self.func8(imports, var21, var22 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        self.func20(imports, 4i32); // wasm/sound/length/setChannelLengthCounter
                        return 1i32;
                    }
                }
            }
        }
        let var23 = var0;
        var2 = (var23 == 65300i32) as i32;
        let var24 = var2;
        let var25: i32;
        if var24 != 0 {
            let var26 = var1;
            let var27 = self.func2(imports, 7i32, var26 & 255i32); // wasm/helpers/index/checkBitOnByte
            var25 = var27;
        } else {
            let var28 = var2;
            var25 = var28;
        }
        if var25 & 1i32 != 0 {
            let var29 = var0;
            let var30 = var1;
            self.func8(imports, var29, var30 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            self.func35(imports); // wasm/sound/channel1/Channel1.trigger
            return 1i32;
        } else {
            let var31 = var0;
            var2 = (var31 == 65305i32) as i32;
            let var32 = var2;
            let var33: i32;
            if var32 != 0 {
                let var34 = var1;
                let var35 = self.func2(imports, 7i32, var34 & 255i32); // wasm/helpers/index/checkBitOnByte
                var33 = var35;
            } else {
                let var36 = var2;
                var33 = var36;
            }
            if var33 & 1i32 != 0 {
                let var37 = var0;
                let var38 = var1;
                self.func8(imports, var37, var38 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                self.func36(imports); // wasm/sound/channel2/Channel2.trigger
                return 1i32;
            } else {
                let var39 = var0;
                var2 = (var39 == 65310i32) as i32;
                let var40 = var2;
                let var41: i32;
                if var40 != 0 {
                    let var42 = var1;
                    let var43 = self.func2(imports, 7i32, var42 & 255i32); // wasm/helpers/index/checkBitOnByte
                    var41 = var43;
                } else {
                    let var44 = var2;
                    var41 = var44;
                }
                if var41 & 1i32 != 0 {
                    let var45 = var0;
                    let var46 = var1;
                    self.func8(imports, var45, var46 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func37(imports); // wasm/sound/channel3/Channel3.trigger
                    return 1i32;
                } else {
                    let var47 = var0;
                    var2 = (var47 == 65315i32) as i32;
                    let var48 = var2;
                    let var49: i32;
                    if var48 != 0 {
                        let var50 = var1;
                        let var51 = self.func2(imports, 7i32, var50 & 255i32); // wasm/helpers/index/checkBitOnByte
                        var49 = var51;
                    } else {
                        let var52 = var2;
                        var49 = var52;
                    }
                    if var49 & 1i32 != 0 {
                        let var53 = var0;
                        let var54 = var1;
                        self.func8(imports, var53, var54 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        self.func41(imports); // wasm/sound/channel4/Channel4.trigger
                        return 1i32;
                    }
                }
            }
        }
        let var55 = var0;
        var2 = (var55 == 65318i32) as i32;
        let var56 = var2;
        let var57: i32;
        if var56 != 0 {
            let var58 = var1;
            let var59 = self.func2(imports, 7i32, var58 & 255i32); // wasm/helpers/index/checkBitOnByte
            var57 = (var59 == 0) as i32;
        } else {
            let var60 = var2;
            var57 = var60;
        }
        if var57 & 1i32 != 0 {
            var2 = 65296i32;
            'label0: loop {
                let var61 = var2;
                if ((var61 as u32) < 65318i32 as u32) as i32 != 0 {
                    let var62 = var2;
                    self.func8(imports, var62, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    let var63 = var2;
                    var2 = var63.wrapping_add(1i32) & 65535i32;
                    continue 'label0;
                }
                break;
            }
            let var64 = var0;
            let var65 = var1;
            self.func8(imports, var64, var65 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return 1i32;
        }
        0i32
    }
    // wasm/memory/writeTraps/_dmaTransfer
    fn func43<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = var0;
        var0 = var2.wrapping_shl(8i32 as u32) & 65535i32;
        'label0: loop {
            let var3 = var1;
            if ((var3 as u32) < 160i32 as u32) as i32 != 0 {
                let var4 = var1;
                let var5 = var0;
                let var6 = var1;
                let var7 = self.func13(imports, var5.wrapping_add(var6) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                self.func8(imports, var4.wrapping_add(65024i32) & 65535i32, var7); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                let var8 = var1;
                var1 = var8.wrapping_add(1i32) & 65535i32;
                continue 'label0;
            }
            break;
        }
    }
    // wasm/memory/writeTraps/checkWriteTraps
    fn func44<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = var0;
        var3 = 32768i32;
        let var6 = var3;
        if ((var5 as u32) < var6 as u32) as i32 != 0 {
            let var7 = var0;
            let var8 = var1;
            self.func3(imports, var7, var8); // wasm/memory/banking/handleBanking
            return 0i32;
        }
        let var9 = var0;
        var3 = (var9 as u32 >= 32768i32 as u32) as i32;
        let var10 = var3;
        let var11: i32;
        if var10 != 0 {
            let var12 = var0;
            var11 = ((var12 as u32) < 40960i32 as u32) as i32;
        } else {
            let var13 = var3;
            var11 = var13;
        }
        if var11 & 1i32 != 0 {
            let var14 = self.global19;
            if (var14 as u32 > 2i32 as u32) as i32 != 0 {
                return 0i32;
            }
        }
        var4 = 65024i32;
        let var15 = var0;
        var3 = (var15 as u32 >= 57344i32 as u32) as i32;
        let var16 = var3;
        let var17: i32;
        if var16 != 0 {
            let var18 = var0;
            var17 = ((var18 as u32) < 65024i32 as u32) as i32;
        } else {
            let var19 = var3;
            var17 = var19;
        }
        if var17 & 1i32 != 0 {
            let var20 = var2;
            if var20 != 0 {
                let var21 = var0;
                let var22 = var1;
                self.func8(imports, var21, var22 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            } else {
                let var23 = var0;
                let var24 = var1;
                self.func11(imports, var23, var24); // wasm/memory/store/sixteenBitStoreIntoGBMemorySkipTraps
            }
        }
        let var25 = var0;
        var3 = (var25 as u32 >= 65024i32 as u32) as i32;
        let var26 = var3;
        let var27: i32;
        if var26 != 0 {
            let var28 = var0;
            var27 = (var28 as u32 <= 65183i32 as u32) as i32;
        } else {
            let var29 = var3;
            var27 = var29;
        }
        if var27 & 1i32 != 0 {
            let var30 = self.global19;
            if (var30 != 2i32) as i32 != 0 {
                return 0i32;
            }
        }
        let var31 = var0;
        var3 = (var31 as u32 >= 65184i32 as u32) as i32;
        let var32 = var3;
        let var33: i32;
        if var32 != 0 {
            let var34 = var0;
            var33 = (var34 as u32 <= 65279i32 as u32) as i32;
        } else {
            let var35 = var3;
            var33 = var35;
        }
        if var33 & 1i32 != 0 {
            return 0i32;
        }
        let var36 = var0;
        if (var36 == 65284i32) as i32 != 0 {
            let var37 = var0;
            self.func8(imports, var37, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return 0i32;
        }
        let var38 = var0;
        var3 = (var38 as u32 >= 65296i32 as u32) as i32;
        let var39 = var3;
        let var40: i32;
        if var39 != 0 {
            let var41 = var0;
            var40 = (var41 as u32 <= 65318i32 as u32) as i32;
        } else {
            let var42 = var3;
            var40 = var42;
        }
        if var40 & 1i32 != 0 {
            let var43 = var0;
            let var44 = var1;
            let var45 = self.func42(imports, var43, var44); // wasm/sound/registers/handledWriteToSoundRegister
            if var45 != 0 {
                return 0i32;
            }
        }
        let var46 = var0;
        if (var46 == 65348i32) as i32 != 0 {
            let var47 = var0;
            self.func8(imports, var47, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return 0i32;
        }
        let var48 = var0;
        if (var48 == 65350i32) as i32 != 0 {
            let var49 = var1;
            self.func43(imports, var49 & 255i32); // wasm/memory/writeTraps/_dmaTransfer
        }
        1i32
    }
    // wasm/memory/store/eightBitStoreIntoGBMemory
    fn func45<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = var1;
        let var4 = self.func44(imports, var2, var3, 1i32); // wasm/memory/writeTraps/checkWriteTraps
        if var4 != 0 {
            let var5 = var0;
            let var6 = var1;
            self.func7(imports, var5, var6); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
    }
    // wasm/memory/memory/initializeCartridge
    fn func46<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func18(imports, 327i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var0 = var2;
        self.global10 = 0i32;
        self.global15 = 0i32;
        self.global11 = 0i32;
        self.global16 = 0i32;
        self.global13 = 0i32;
        let var3 = var0;
        if var3 != 0 {
            let var4 = var0;
            var1 = (var4 as u32 >= 1i32 as u32) as i32;
            let var5 = var1;
            let var6: i32;
            if var5 != 0 {
                let var7 = var0;
                var6 = (var7 as u32 <= 3i32 as u32) as i32;
            } else {
                let var8 = var1;
                var6 = var8;
            }
            if var6 & 1i32 != 0 {
                self.global15 = 1i32;
            } else {
                let var9 = var0;
                var1 = (var9 as u32 >= 5i32 as u32) as i32;
                let var10 = var1;
                let var11: i32;
                if var10 != 0 {
                    let var12 = var0;
                    var11 = (var12 as u32 <= 6i32 as u32) as i32;
                } else {
                    let var13 = var1;
                    var11 = var13;
                }
                if var11 & 1i32 != 0 {
                    self.global11 = 1i32;
                } else {
                    let var14 = var0;
                    var1 = (var14 as u32 >= 15i32 as u32) as i32;
                    let var15 = var1;
                    let var16: i32;
                    if var15 != 0 {
                        let var17 = var0;
                        var16 = (var17 as u32 <= 19i32 as u32) as i32;
                    } else {
                        let var18 = var1;
                        var16 = var18;
                    }
                    if var16 & 1i32 != 0 {
                        self.global16 = 1i32;
                    } else {
                        let var19 = var0;
                        var1 = (var19 as u32 >= 25i32 as u32) as i32;
                        let var20 = var1;
                        let var21: i32;
                        if var20 != 0 {
                            let var22 = var0;
                            var21 = (var22 as u32 <= 30i32 as u32) as i32;
                        } else {
                            let var23 = var1;
                            var21 = var23;
                        }
                        if var21 & 1i32 != 0 {
                            self.global13 = 1i32;
                        }
                    }
                }
            }
        } else {
            self.global10 = 1i32;
        }
    }
    // wasm/sound/channel1/Channel1.initialize
    fn func47<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func45(imports, 65296i32, 128i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65297i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65298i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65299i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65300i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
    }
    // wasm/sound/channel2/Channel2.initialize
    fn func48<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func45(imports, 65301i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65302i32, 63i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65303i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65304i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65305i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
    }
    // wasm/sound/channel3/Channel3.initialize
    fn func49<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func45(imports, 65306i32, 127i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65307i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65308i32, 159i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65309i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65310i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
    }
    // wasm/sound/channel4/Channel4.initialize
    fn func50<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func45(imports, 65311i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65312i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65313i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65314i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65315i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
    }
    // wasm/sound/sound/initializeSound
    fn func51<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func47(imports); // wasm/sound/channel1/Channel1.initialize
        self.func48(imports); // wasm/sound/channel2/Channel2.initialize
        self.func49(imports); // wasm/sound/channel3/Channel3.initialize
        self.func50(imports); // wasm/sound/channel4/Channel4.initialize
        self.func45(imports, 65316i32, 119i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65317i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        self.func45(imports, 65318i32, 241i32); // wasm/memory/store/eightBitStoreIntoGBMemory
    }
    // wasm/cpu/index/initialize
    fn func52<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.func1(imports, 8861696i32, 1i32, var1, 0i32, 0i32, 0i32, 0i32, 0i32); // wasm/helpers/index/log
        let var2 = var0;
        if (var2 as u32 <= 0i32 as u32) as i32 != 0 {
            self.global0 = 256i32;
            self.global1 = 1i32;
            self.global2 = 176i32;
            self.global3 = 0i32;
            self.global4 = 19i32;
            self.global5 = 0i32;
            self.global6 = 216i32;
            self.global7 = 1i32;
            self.global8 = 77i32;
            self.global9 = 65534i32;
            self.func45(imports, 65296i32, 128i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65297i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65298i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65300i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65302i32, 63i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65303i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65305i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65306i32, 127i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65307i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65308i32, 159i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65310i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65312i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65315i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65316i32, 119i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65317i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65318i32, 241i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65344i32, 145i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65351i32, 252i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65352i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func45(imports, 65353i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        }
        self.func46(imports); // wasm/memory/memory/initializeCartridge
        self.func51(imports); // wasm/sound/sound/initializeSound
    }
    // wasm/helpers/index/concatenateBytes
    fn func53<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = var1;
        (var2 & 255i32).wrapping_shl(8i32 as u32) | var3 & 255i32
    }
    // wasm/cpu/opcodes/isOpcode
    fn func54<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = var1;
        if (var2 == var3) as i32 != 0 {
            return 1i32;
        }
        0i32
    }
    // wasm/cpu/flags/setFlagBit
    fn func55<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let var3 = var0;
        var2 = 1i32.wrapping_shl(var3 as u32) & 255i32;
        let var4 = var1;
        if (var4 as u32 > 0i32 as u32) as i32 != 0 {
            let var5 = self.global2;
            let var6 = var2;
            self.global2 = (var5 | var6) & 255i32;
        } else {
            let var7 = self.global2;
            let var8 = var2;
            self.global2 = var7 & (var8 ^ 255i32);
        }
        let var9 = self.global2;
        var9
    }
    // wasm/cpu/flags/setHalfCarryFlag
    fn func56<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func55(imports, 5i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
    fn func57<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        if (var3 >= 0i32) as i32 != 0 {
            let var4 = var0;
            let var5 = var1;
            if (var4 & 15i32).wrapping_add(var5 & 15i32) & 16i32 != 0 {
                self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        } else {
            let var6 = var1;
            var2 = var6;
            let var7 = var2;
            let var8 = var2;
            let var9 = var2;
            let var10 = var0;
            if (((if (var9 > 0i32) as i32 != 0 { var7 } else { 0i32.wrapping_sub(var8) }) & 15i32) as u32 > (var10 & 15i32) as u32) as i32 != 0 {
                self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        }
    }
    // wasm/cpu/flags/setZeroFlag
    fn func58<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func55(imports, 7i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/setSubtractFlag
    fn func59<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func55(imports, 6i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/setCarryFlag
    fn func60<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func55(imports, 4i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/helpers/index/rotateByteLeft
    fn func61<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        (var1.wrapping_shl(1i32 as u32) & 255i32 | (var2 as u32).wrapping_shr(7i32 as u32) as i32) & 255i32
    }
    // wasm/memory/store/sixteenBitStoreIntoGBMemory
    fn func62<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        let var4 = self.func9(imports, var3); // wasm/helpers/index/splitHighByte
        var2 = var4;
        let var5 = var0;
        let var6 = var1;
        let var7 = self.func10(imports, var6); // wasm/helpers/index/splitLowByte
        var1 = var7;
        let var8 = var1;
        let var9 = self.func44(imports, var5, var8, 0i32); // wasm/memory/writeTraps/checkWriteTraps
        if var9 != 0 {
            let var10 = var0;
            let var11 = var1;
            self.func7(imports, var10, var11); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
        let var12 = var0;
        var0 = var12.wrapping_add(1i32) & 65535i32;
        let var13 = var0;
        let var14 = var2;
        let var15 = self.func44(imports, var13, var14, 0i32); // wasm/memory/writeTraps/checkWriteTraps
        if var15 != 0 {
            let var16 = var0;
            let var17 = var2;
            self.func7(imports, var16, var17); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
    }
    // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
    fn func63<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let var3 = var2;
        if var3 != 0 {
            let var4 = var0;
            let var5 = var1;
            let var6 = var0;
            let var7 = var1;
            var2 = var4 ^ var5 ^ var6.wrapping_add(var7);
            let var8 = var2;
            if var8 & 16i32 != 0 {
                self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
            let var9 = var2;
            if var9 & 256i32 != 0 {
                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        } else {
            let var10 = var0;
            let var11 = var1;
            var2 = var10.wrapping_add(var11 & 65535i32) & 65535i32;
            let var12 = var2;
            let var13 = var0;
            if ((var12 as u32) < var13 as u32) as i32 != 0 {
                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
            let var14 = var0;
            let var15 = var1;
            let var16 = var2;
            if (var14 ^ var15 & 65535i32 ^ var16) & 4096i32 != 0 {
                self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        }
    }
    // wasm/helpers/index/rotateByteRight
    fn func64<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        ((var1 as u32).wrapping_shr(1i32 as u32) as i32 | var2.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    // wasm/cpu/flags/getCarryFlag
    fn func65<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(4i32 as u32) as i32 & 1i32
    }
    // wasm/helpers/index/rotateByteLeftThroughCarry
    fn func66<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        (var1.wrapping_shl(1i32 as u32) & 255i32 | var2) & 255i32
    }
    // wasm/cpu/index/relativeJump
    fn func67<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global0;
        let var2 = var0;
        self.global0 = var1.wrapping_add(var2.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32)) & 65535i32;
        let var3 = self.global0;
        self.global0 = var3.wrapping_add(1i32) & 65535i32;
    }
    // wasm/helpers/index/rotateByteRightThroughCarry
    fn func68<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        ((var1 as u32).wrapping_shr(1i32 as u32) as i32 | var2.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    // wasm/cpu/flags/getZeroFlag
    fn func69<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(7i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/flags/getHalfCarryFlag
    fn func70<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(5i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/flags/getSubtractFlag
    fn func71<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(6i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/flags/checkAndSetEightBitCarryFlag
    fn func72<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        if (var3 >= 0i32) as i32 != 0 {
            let var4 = var0;
            let var5 = var0;
            let var6 = var1;
            if (var4 as u32 > (var5.wrapping_add(var6 & 255i32) & 255i32) as u32) as i32 != 0 {
                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        } else {
            let var7 = var1;
            var2 = var7;
            let var8 = var2;
            let var9 = var2;
            let var10 = var2;
            let var11 = var0;
            if ((if (var10 > 0i32) as i32 != 0 { var8 } else { 0i32.wrapping_sub(var9) }) > var11) as i32 != 0 {
                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        }
    }
    // wasm/cpu/instructions/addARegister
    fn func73<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.func57(imports, var1, var2); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var3 = self.global1;
        let var4 = var0;
        self.func72(imports, var3, var4); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var5 = self.global1;
        let var6 = var0;
        self.global1 = var5.wrapping_add(var6) & 255i32;
        let var7 = self.global1;
        if var7 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/addAThroughCarryRegister
    fn func74<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        let var4 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        var1 = var2.wrapping_add(var3).wrapping_add(var4) & 255i32;
        let var5 = self.global1;
        let var6 = var0;
        let var7 = var1;
        if (var5 ^ var6 ^ var7) & 16i32 != 0 {
            self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        } else {
            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        }
        let var8 = self.global1;
        let var9 = var0;
        let var10 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        if ((var8.wrapping_add(var9).wrapping_add(var10) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var11 = var1;
        self.global1 = var11;
        let var12 = self.global1;
        if var12 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/subARegister
    fn func75<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        var1 = var3.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        let var4 = var1;
        self.func57(imports, var2, var4); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var5 = self.global1;
        let var6 = var1;
        self.func72(imports, var5, var6); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var7 = self.global1;
        let var8 = var0;
        self.global1 = var7.wrapping_sub(var8) & 255i32;
        let var9 = self.global1;
        if var9 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/subAThroughCarryRegister
    fn func76<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        let var4 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        var1 = var2.wrapping_sub(var3).wrapping_sub(var4) & 255i32;
        let var5 = self.global1;
        let var6 = var0;
        let var7 = var1;
        if (var5 ^ var6 ^ var7) & 16i32 != 0 {
            self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        } else {
            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        }
        let var8 = self.global1;
        let var9 = var0;
        let var10 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
        if ((var8.wrapping_sub(var9).wrapping_sub(var10) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var11 = var1;
        self.global1 = var11;
        let var12 = self.global1;
        if var12 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/andARegister
    fn func77<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = var1 & var2 & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/instructions/xorARegister
    fn func78<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = (var1 ^ var2) & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/instructions/orARegister
    fn func79<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = (var1 | var2) & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/instructions/cpARegister
    fn func80<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        var1 = var3.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        let var4 = var1;
        self.func57(imports, var2, var4); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var5 = self.global1;
        let var6 = var1;
        self.func72(imports, var5, var6); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var7 = self.global1;
        let var8 = var1;
        if var7.wrapping_add(var8) & 65535i32 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/memory/load/sixteenBitLoadFromGBMemory
    fn func81<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func17(imports, var3); // wasm/memory/readTraps/checkReadTraps
        var1 = var4;
        let var5 = var1;
        let var6: i32;
        if ((var5) < 0i32) as i32 != 0 {
            let var7 = var0;
            let var8 = self.func12(imports, var7); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
            var6 = var8;
        } else {
            let var9 = var1;
            var6 = var9 & 255i32;
        }
        var1 = var6;
        let var10 = var0;
        var0 = var10.wrapping_add(1i32) & 65535i32;
        let var11 = var0;
        let var12 = self.func17(imports, var11); // wasm/memory/readTraps/checkReadTraps
        var2 = var12;
        let var13 = var2;
        let var14: i32;
        if ((var13) < 0i32) as i32 != 0 {
            let var15 = var0;
            let var16 = self.func12(imports, var15); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
            var14 = var16;
        } else {
            let var17 = var2;
            var14 = var17 & 255i32;
        }
        var0 = var14;
        let var18 = var0;
        let var19 = var1;
        let var20 = self.func53(imports, var18, var19); // wasm/helpers/index/concatenateBytes
        var20
    }
    // wasm/cpu/instructions/rotateRegisterLeft
    fn func82<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        if (var1 & 128i32 == 128i32) as i32 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var2 = var0;
        let var3 = self.func61(imports, var2); // wasm/helpers/index/rotateByteLeft
        var0 = var3;
        let var4 = var0;
        if var4 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var0;
        var5
    }
    // wasm/cpu/instructions/rotateRegisterRight
    fn func83<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        if ((var1 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var2 = var0;
        let var3 = self.func64(imports, var2); // wasm/helpers/index/rotateByteRight
        var0 = var3;
        let var4 = var0;
        if var4 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var0;
        var5
    }
    // wasm/cpu/instructions/rotateRegisterLeftThroughCarry
    fn func84<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        let var4 = self.func66(imports, var3); // wasm/helpers/index/rotateByteLeftThroughCarry
        var0 = var4;
        let var5 = var1;
        if var5 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        if var6 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var7 = var0;
        var7
    }
    // wasm/cpu/instructions/rotateRegisterRightThroughCarry
    fn func85<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        let var4 = self.func68(imports, var3); // wasm/helpers/index/rotateByteRightThroughCarry
        var0 = var4;
        let var5 = var1;
        if var5 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        if var6 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var7 = var0;
        var7
    }
    // wasm/cpu/instructions/shiftLeftRegister
    fn func86<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        var0 = var3.wrapping_shl(1i32 as u32) & 255i32;
        let var4 = var1;
        if var4 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var5 = var0;
        if var5 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var6 = var0;
        var6
    }
    // wasm/cpu/instructions/shiftRightArithmeticRegister
    fn func87<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = var0;
        if (var3 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var4 = var0;
        if (var4 & 1i32 == 1i32) as i32 != 0 {
            var2 = 1i32;
        }
        let var5 = var0;
        var0 = (var5 as u32).wrapping_shr(1i32 as u32) as i32;
        let var6 = var1;
        if var6 != 0 {
            let var7 = var0;
            var0 = (var7 | 128i32) & 255i32;
        }
        let var8 = var0;
        if var8 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var9 = var2;
        if var9 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var10 = var0;
        var10
    }
    // wasm/cpu/instructions/swapNibblesOnRegister
    fn func88<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        var0 = (var1 & 15i32).wrapping_shl(4i32 as u32) | ((var2 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32;
        let var3 = var0;
        if var3 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        let var4 = var0;
        var4
    }
    // wasm/cpu/instructions/shiftRightLogicalRegister
    fn func89<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        var0 = (var3 as u32).wrapping_shr(1i32 as u32) as i32;
        let var4 = var0;
        if var4 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var1;
        if var5 != 0 {
            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        var6
    }
    // wasm/cpu/instructions/testBitOnRegister
    fn func90<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        if var2 & 1i32.wrapping_shl(var3 as u32) & 255i32 != 0 {
            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        let var4 = var1;
        var4
    }
    // wasm/cpu/instructions/setBitOnRegister
    fn func91<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let var3 = var1;
        let var4: i32;
        if (var3 as u32 > 0i32 as u32) as i32 != 0 {
            let var5 = var2;
            let var6 = var0;
            var4 = (var5 | 1i32.wrapping_shl(var6 as u32) & 255i32) & 255i32;
        } else {
            let var7 = var2;
            let var8 = var0;
            var4 = var7 & (1i32.wrapping_shl(var8 as u32) & 255i32 ^ -1i32) & 255i32;
        }
        var2 = var4;
        let var9 = var2;
        var9
    }
    // wasm/cpu/cbOpcodes/handleCbOpcode
    fn func92<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        var6 = -1i32;
        let var7 = var0;
        var4 = (var7 as u32).wrapping_rem(8i32 as u32) as i32;
        let var8 = var4;
        if var8 != 0 {
            let var9 = var4;
            if (var9 == 1i32) as i32 != 0 {
                let var10 = self.global4;
                var1 = var10;
            } else {
                let var11 = var4;
                if (var11 == 2i32) as i32 != 0 {
                    let var12 = self.global5;
                    var1 = var12;
                } else {
                    let var13 = var4;
                    if (var13 == 3i32) as i32 != 0 {
                        let var14 = self.global6;
                        var1 = var14;
                    } else {
                        let var15 = var4;
                        if (var15 == 4i32) as i32 != 0 {
                            let var16 = self.global7;
                            var1 = var16;
                        } else {
                            let var17 = var4;
                            if (var17 == 5i32) as i32 != 0 {
                                let var18 = self.global8;
                                var1 = var18;
                            } else {
                                let var19 = var4;
                                if (var19 == 6i32) as i32 != 0 {
                                    let var20 = self.global7;
                                    let var21 = self.global8;
                                    let var22 = self.func53(imports, var20, var21); // wasm/helpers/index/concatenateBytes
                                    let var23 = self.func18(imports, var22); // wasm/memory/load/eightBitLoadFromGBMemory
                                    var1 = var23;
                                } else {
                                    let var24 = var4;
                                    if (var24 == 7i32) as i32 != 0 {
                                        let var25 = self.global1;
                                        var1 = var25;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let var26 = self.global3;
            var1 = var26;
        }
        let var27 = var0;
        var5 = ((var27 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32;
        let var28 = var5;
        if (var28 == 0) as i32 != 0 {
            let var29 = var0;
            if (var29 as u32 <= 7i32 as u32) as i32 != 0 {
                let var30 = var1;
                let var31 = self.func82(imports, var30); // wasm/cpu/instructions/rotateRegisterLeft
                var2 = var31;
                var3 = 1i32;
            } else {
                let var32 = var0;
                if (var32 as u32 <= 15i32 as u32) as i32 != 0 {
                    let var33 = var1;
                    let var34 = self.func83(imports, var33); // wasm/cpu/instructions/rotateRegisterRight
                    var2 = var34;
                    var3 = 1i32;
                }
            }
        }
        let var35 = var5;
        if (var35 == 1i32) as i32 != 0 {
            let var36 = var0;
            if (var36 as u32 <= 23i32 as u32) as i32 != 0 {
                let var37 = var1;
                let var38 = self.func84(imports, var37); // wasm/cpu/instructions/rotateRegisterLeftThroughCarry
                var2 = var38;
                var3 = 1i32;
            } else {
                let var39 = var0;
                if (var39 as u32 <= 31i32 as u32) as i32 != 0 {
                    let var40 = var1;
                    let var41 = self.func85(imports, var40); // wasm/cpu/instructions/rotateRegisterRightThroughCarry
                    var2 = var41;
                    var3 = 1i32;
                }
            }
        }
        let var42 = var5;
        if (var42 == 2i32) as i32 != 0 {
            let var43 = var0;
            if (var43 as u32 <= 39i32 as u32) as i32 != 0 {
                let var44 = var1;
                let var45 = self.func86(imports, var44); // wasm/cpu/instructions/shiftLeftRegister
                var2 = var45;
                var3 = 1i32;
            } else {
                let var46 = var0;
                if (var46 as u32 <= 47i32 as u32) as i32 != 0 {
                    let var47 = var1;
                    let var48 = self.func87(imports, var47); // wasm/cpu/instructions/shiftRightArithmeticRegister
                    var2 = var48;
                    var3 = 1i32;
                }
            }
        }
        let var49 = var5;
        if (var49 == 3i32) as i32 != 0 {
            let var50 = var0;
            if (var50 as u32 <= 55i32 as u32) as i32 != 0 {
                let var51 = var1;
                let var52 = self.func88(imports, var51); // wasm/cpu/instructions/swapNibblesOnRegister
                var2 = var52;
                var3 = 1i32;
            } else {
                let var53 = var0;
                if (var53 as u32 <= 63i32 as u32) as i32 != 0 {
                    let var54 = var1;
                    let var55 = self.func89(imports, var54); // wasm/cpu/instructions/shiftRightLogicalRegister
                    var2 = var55;
                    var3 = 1i32;
                }
            }
        }
        let var56 = var5;
        if (var56 == 4i32) as i32 != 0 {
            let var57 = var0;
            if (var57 as u32 <= 71i32 as u32) as i32 != 0 {
                let var58 = var1;
                let var59 = self.func90(imports, 0i32, var58); // wasm/cpu/instructions/testBitOnRegister
                var2 = var59;
                var3 = 1i32;
            } else {
                let var60 = var0;
                if (var60 as u32 <= 79i32 as u32) as i32 != 0 {
                    let var61 = var1;
                    let var62 = self.func90(imports, 1i32, var61); // wasm/cpu/instructions/testBitOnRegister
                    var2 = var62;
                    var3 = 1i32;
                }
            }
        }
        let var63 = var5;
        if (var63 == 5i32) as i32 != 0 {
            let var64 = var0;
            if (var64 as u32 <= 87i32 as u32) as i32 != 0 {
                let var65 = var1;
                let var66 = self.func90(imports, 2i32, var65); // wasm/cpu/instructions/testBitOnRegister
                var2 = var66;
                var3 = 1i32;
            } else {
                let var67 = var0;
                if (var67 as u32 <= 95i32 as u32) as i32 != 0 {
                    let var68 = var1;
                    let var69 = self.func90(imports, 3i32, var68); // wasm/cpu/instructions/testBitOnRegister
                    var2 = var69;
                    var3 = 1i32;
                }
            }
        }
        let var70 = var5;
        if (var70 == 6i32) as i32 != 0 {
            let var71 = var0;
            if (var71 as u32 <= 103i32 as u32) as i32 != 0 {
                let var72 = var1;
                let var73 = self.func90(imports, 4i32, var72); // wasm/cpu/instructions/testBitOnRegister
                var2 = var73;
                var3 = 1i32;
            } else {
                let var74 = var0;
                if (var74 as u32 <= 111i32 as u32) as i32 != 0 {
                    let var75 = var1;
                    let var76 = self.func90(imports, 5i32, var75); // wasm/cpu/instructions/testBitOnRegister
                    var2 = var76;
                    var3 = 1i32;
                }
            }
        }
        let var77 = var5;
        if (var77 == 7i32) as i32 != 0 {
            let var78 = var0;
            if (var78 as u32 <= 119i32 as u32) as i32 != 0 {
                let var79 = var1;
                let var80 = self.func90(imports, 6i32, var79); // wasm/cpu/instructions/testBitOnRegister
                var2 = var80;
                var3 = 1i32;
            } else {
                let var81 = var0;
                if (var81 as u32 <= 127i32 as u32) as i32 != 0 {
                    let var82 = var1;
                    let var83 = self.func90(imports, 7i32, var82); // wasm/cpu/instructions/testBitOnRegister
                    var2 = var83;
                    var3 = 1i32;
                }
            }
        }
        let var84 = var5;
        if (var84 == 8i32) as i32 != 0 {
            let var85 = var0;
            if (var85 as u32 <= 135i32 as u32) as i32 != 0 {
                let var86 = var1;
                let var87 = self.func91(imports, 0i32, 0i32, var86); // wasm/cpu/instructions/setBitOnRegister
                var2 = var87;
                var3 = 1i32;
            } else {
                let var88 = var0;
                if (var88 as u32 <= 143i32 as u32) as i32 != 0 {
                    let var89 = var1;
                    let var90 = self.func91(imports, 1i32, 0i32, var89); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var90;
                    var3 = 1i32;
                }
            }
        }
        let var91 = var5;
        if (var91 == 9i32) as i32 != 0 {
            let var92 = var0;
            if (var92 as u32 <= 151i32 as u32) as i32 != 0 {
                let var93 = var1;
                let var94 = self.func91(imports, 2i32, 0i32, var93); // wasm/cpu/instructions/setBitOnRegister
                var2 = var94;
                var3 = 1i32;
            } else {
                let var95 = var0;
                if (var95 as u32 <= 159i32 as u32) as i32 != 0 {
                    let var96 = var1;
                    let var97 = self.func91(imports, 3i32, 0i32, var96); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var97;
                    var3 = 1i32;
                }
            }
        }
        let var98 = var5;
        if (var98 == 10i32) as i32 != 0 {
            let var99 = var0;
            if (var99 as u32 <= 167i32 as u32) as i32 != 0 {
                let var100 = var1;
                let var101 = self.func91(imports, 4i32, 0i32, var100); // wasm/cpu/instructions/setBitOnRegister
                var2 = var101;
                var3 = 1i32;
            } else {
                let var102 = var0;
                if (var102 as u32 <= 175i32 as u32) as i32 != 0 {
                    let var103 = var1;
                    let var104 = self.func91(imports, 5i32, 0i32, var103); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var104;
                    var3 = 1i32;
                }
            }
        }
        let var105 = var5;
        if (var105 == 11i32) as i32 != 0 {
            let var106 = var0;
            if (var106 as u32 <= 183i32 as u32) as i32 != 0 {
                let var107 = var1;
                let var108 = self.func91(imports, 6i32, 0i32, var107); // wasm/cpu/instructions/setBitOnRegister
                var2 = var108;
                var3 = 1i32;
            } else {
                let var109 = var0;
                if (var109 as u32 <= 191i32 as u32) as i32 != 0 {
                    let var110 = var1;
                    let var111 = self.func91(imports, 7i32, 0i32, var110); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var111;
                    var3 = 1i32;
                }
            }
        }
        let var112 = var5;
        if (var112 == 12i32) as i32 != 0 {
            let var113 = var0;
            if (var113 as u32 <= 199i32 as u32) as i32 != 0 {
                let var114 = var1;
                let var115 = self.func91(imports, 0i32, 1i32, var114); // wasm/cpu/instructions/setBitOnRegister
                var2 = var115;
                var3 = 1i32;
            } else {
                let var116 = var0;
                if (var116 as u32 <= 207i32 as u32) as i32 != 0 {
                    let var117 = var1;
                    let var118 = self.func91(imports, 1i32, 1i32, var117); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var118;
                    var3 = 1i32;
                }
            }
        }
        let var119 = var5;
        if (var119 == 13i32) as i32 != 0 {
            let var120 = var0;
            if (var120 as u32 <= 215i32 as u32) as i32 != 0 {
                let var121 = var1;
                let var122 = self.func91(imports, 2i32, 1i32, var121); // wasm/cpu/instructions/setBitOnRegister
                var2 = var122;
                var3 = 1i32;
            } else {
                let var123 = var0;
                if (var123 as u32 <= 223i32 as u32) as i32 != 0 {
                    let var124 = var1;
                    let var125 = self.func91(imports, 3i32, 1i32, var124); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var125;
                    var3 = 1i32;
                }
            }
        }
        let var126 = var5;
        if (var126 == 14i32) as i32 != 0 {
            let var127 = var0;
            if (var127 as u32 <= 231i32 as u32) as i32 != 0 {
                let var128 = var1;
                let var129 = self.func91(imports, 4i32, 1i32, var128); // wasm/cpu/instructions/setBitOnRegister
                var2 = var129;
                var3 = 1i32;
            } else {
                let var130 = var0;
                if (var130 as u32 <= 239i32 as u32) as i32 != 0 {
                    let var131 = var1;
                    let var132 = self.func91(imports, 5i32, 1i32, var131); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var132;
                    var3 = 1i32;
                }
            }
        }
        let var133 = var5;
        if (var133 == 15i32) as i32 != 0 {
            let var134 = var0;
            if (var134 as u32 <= 247i32 as u32) as i32 != 0 {
                let var135 = var1;
                let var136 = self.func91(imports, 6i32, 1i32, var135); // wasm/cpu/instructions/setBitOnRegister
                var2 = var136;
                var3 = 1i32;
            } else {
                let var137 = var0;
                if (var137 as u32 <= 255i32 as u32) as i32 != 0 {
                    let var138 = var1;
                    let var139 = self.func91(imports, 7i32, 1i32, var138); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var139;
                    var3 = 1i32;
                }
            }
        }
        let var140 = var4;
        if var140 != 0 {
            let var141 = var4;
            if (var141 == 1i32) as i32 != 0 {
                let var142 = var2;
                self.global4 = var142;
            } else {
                let var143 = var4;
                if (var143 == 2i32) as i32 != 0 {
                    let var144 = var2;
                    self.global5 = var144;
                } else {
                    let var145 = var4;
                    if (var145 == 3i32) as i32 != 0 {
                        let var146 = var2;
                        self.global6 = var146;
                    } else {
                        let var147 = var4;
                        if (var147 == 4i32) as i32 != 0 {
                            let var148 = var2;
                            self.global7 = var148;
                        } else {
                            let var149 = var4;
                            if (var149 == 5i32) as i32 != 0 {
                                let var150 = var2;
                                self.global8 = var150;
                            } else {
                                let var151 = var4;
                                if (var151 == 6i32) as i32 != 0 {
                                    let var152 = self.global7;
                                    let var153 = self.global8;
                                    let var154 = self.func53(imports, var152, var153); // wasm/helpers/index/concatenateBytes
                                    let var155 = var2;
                                    self.func45(imports, var154, var155); // wasm/memory/store/eightBitStoreIntoGBMemory
                                } else {
                                    let var156 = var4;
                                    if (var156 == 7i32) as i32 != 0 {
                                        let var157 = var2;
                                        self.global1 = var157;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let var158 = var2;
            self.global3 = var158;
        }
        let var159 = self.global0;
        self.global0 = var159.wrapping_add(1i32) & 65535i32;
        let var160 = var3;
        if var160 != 0 {
            let var161 = var4;
            let var162: i32;
            if (var161 == 6i32) as i32 != 0 {
                var162 = 16i32;
            } else {
                var162 = 8i32;
            }
            var6 = var162;
        }
        let var163 = var6;
        var163
    }
    // wasm/interrupts/index/setInterrupts
    fn func93<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.global55 = var1;
    }
    // wasm/cpu/opcodes/executeOpcode
    fn func94<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        var3 = -1i32;
        let var7 = self.global0;
        self.global0 = var7.wrapping_add(1i32) & 65535i32;
        let var8 = var2;
        let var9 = var1;
        let var10 = self.func53(imports, var8, var9); // wasm/helpers/index/concatenateBytes
        var5 = var10;
        let var11 = var0;
        var6 = ((var11 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32;
        let var12 = var6;
        if (var12 == 0) as i32 != 0 {
            let var13 = var0;
            let var14 = self.func54(imports, var13, 0i32); // wasm/cpu/opcodes/isOpcode
            if var14 != 0 {
                var3 = 4i32;
            } else {
                let var15 = var0;
                let var16 = self.func54(imports, var15, 1i32); // wasm/cpu/opcodes/isOpcode
                if var16 != 0 {
                    let var17 = var5;
                    let var18 = self.func9(imports, var17); // wasm/helpers/index/splitHighByte
                    self.global3 = var18;
                    let var19 = var5;
                    let var20 = self.func10(imports, var19); // wasm/helpers/index/splitLowByte
                    self.global4 = var20;
                    let var21 = self.global0;
                    self.global0 = var21.wrapping_add(2i32) & 65535i32;
                    var3 = 12i32;
                } else {
                    let var22 = var0;
                    let var23 = self.func54(imports, var22, 2i32); // wasm/cpu/opcodes/isOpcode
                    if var23 != 0 {
                        let var24 = self.global3;
                        let var25 = self.global4;
                        let var26 = self.func53(imports, var24, var25); // wasm/helpers/index/concatenateBytes
                        let var27 = self.global1;
                        self.func45(imports, var26, var27); // wasm/memory/store/eightBitStoreIntoGBMemory
                        var3 = 8i32;
                    } else {
                        let var28 = var0;
                        let var29 = self.func54(imports, var28, 3i32); // wasm/cpu/opcodes/isOpcode
                        if var29 != 0 {
                            let var30 = self.global3;
                            let var31 = self.global4;
                            let var32 = self.func53(imports, var30, var31); // wasm/helpers/index/concatenateBytes
                            var3 = var32.wrapping_add(1i32) & 65535i32;
                            let var33 = var3;
                            let var34 = self.func9(imports, var33); // wasm/helpers/index/splitHighByte
                            self.global3 = var34;
                            let var35 = var3;
                            let var36 = self.func10(imports, var35); // wasm/helpers/index/splitLowByte
                            self.global4 = var36;
                            var3 = 8i32;
                        } else {
                            let var37 = var0;
                            let var38 = self.func54(imports, var37, 4i32); // wasm/cpu/opcodes/isOpcode
                            if var38 != 0 {
                                let var39 = self.global3;
                                self.func57(imports, var39, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                let var40 = self.global3;
                                self.global3 = var40.wrapping_add(1i32) & 255i32;
                                let var41 = self.global3;
                                if var41 != 0 {
                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                } else {
                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                }
                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                var3 = 4i32;
                            } else {
                                let var42 = var0;
                                let var43 = self.func54(imports, var42, 5i32); // wasm/cpu/opcodes/isOpcode
                                if var43 != 0 {
                                    let var44 = self.global3;
                                    self.func57(imports, var44, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                    let var45 = self.global3;
                                    self.global3 = var45.wrapping_sub(1i32) & 255i32;
                                    let var46 = self.global3;
                                    if var46 != 0 {
                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                    } else {
                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                    }
                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                    var3 = 4i32;
                                } else {
                                    let var47 = var0;
                                    let var48 = self.func54(imports, var47, 6i32); // wasm/cpu/opcodes/isOpcode
                                    if var48 != 0 {
                                        let var49 = var1;
                                        self.global3 = var49;
                                        let var50 = self.global0;
                                        self.global0 = var50.wrapping_add(1i32) & 65535i32;
                                        var3 = 8i32;
                                    } else {
                                        let var51 = var0;
                                        let var52 = self.func54(imports, var51, 7i32); // wasm/cpu/opcodes/isOpcode
                                        if var52 != 0 {
                                            let var53 = self.global1;
                                            if (var53 & 128i32 == 128i32) as i32 != 0 {
                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            let var54 = self.global1;
                                            let var55 = self.func61(imports, var54); // wasm/helpers/index/rotateByteLeft
                                            self.global1 = var55;
                                            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            var3 = 4i32;
                                        } else {
                                            let var56 = var0;
                                            let var57 = self.func54(imports, var56, 8i32); // wasm/cpu/opcodes/isOpcode
                                            if var57 != 0 {
                                                let var58 = var5;
                                                let var59 = self.global9;
                                                self.func62(imports, var58, var59); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                let var60 = self.global0;
                                                self.global0 = var60.wrapping_add(2i32) & 65535i32;
                                                var3 = 20i32;
                                            } else {
                                                let var61 = var0;
                                                let var62 = self.func54(imports, var61, 9i32); // wasm/cpu/opcodes/isOpcode
                                                if var62 != 0 {
                                                    let var63 = self.global7;
                                                    let var64 = self.global8;
                                                    let var65 = self.func53(imports, var63, var64); // wasm/helpers/index/concatenateBytes
                                                    var3 = var65;
                                                    let var66 = var3;
                                                    let var67 = self.global3;
                                                    let var68 = self.global4;
                                                    let var69 = self.func53(imports, var67, var68); // wasm/helpers/index/concatenateBytes
                                                    var4 = var69;
                                                    let var70 = var4;
                                                    self.func63(imports, var66, var70, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                                    let var71 = var3;
                                                    let var72 = var4;
                                                    var2 = var71.wrapping_add(var72) & 65535i32;
                                                    let var73 = var2;
                                                    let var74 = self.func9(imports, var73); // wasm/helpers/index/splitHighByte
                                                    self.global7 = var74;
                                                    let var75 = var2;
                                                    let var76 = self.func10(imports, var75); // wasm/helpers/index/splitLowByte
                                                    self.global8 = var76;
                                                    self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                    var3 = 8i32;
                                                } else {
                                                    let var77 = var0;
                                                    let var78 = self.func54(imports, var77, 10i32); // wasm/cpu/opcodes/isOpcode
                                                    if var78 != 0 {
                                                        let var79 = self.global3;
                                                        let var80 = self.global4;
                                                        let var81 = self.func53(imports, var79, var80); // wasm/helpers/index/concatenateBytes
                                                        let var82 = self.func18(imports, var81); // wasm/memory/load/eightBitLoadFromGBMemory
                                                        self.global1 = var82;
                                                        var3 = 8i32;
                                                    } else {
                                                        let var83 = var0;
                                                        let var84 = self.func54(imports, var83, 11i32); // wasm/cpu/opcodes/isOpcode
                                                        if var84 != 0 {
                                                            let var85 = self.global3;
                                                            let var86 = self.global4;
                                                            let var87 = self.func53(imports, var85, var86); // wasm/helpers/index/concatenateBytes
                                                            var2 = var87.wrapping_sub(1i32) & 65535i32;
                                                            let var88 = var2;
                                                            let var89 = self.func9(imports, var88); // wasm/helpers/index/splitHighByte
                                                            self.global3 = var89;
                                                            let var90 = var2;
                                                            let var91 = self.func10(imports, var90); // wasm/helpers/index/splitLowByte
                                                            self.global4 = var91;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var92 = var0;
                                                            let var93 = self.func54(imports, var92, 12i32); // wasm/cpu/opcodes/isOpcode
                                                            if var93 != 0 {
                                                                let var94 = self.global4;
                                                                self.func57(imports, var94, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                let var95 = self.global4;
                                                                self.global4 = var95.wrapping_add(1i32) & 255i32;
                                                                let var96 = self.global4;
                                                                if var96 != 0 {
                                                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                } else {
                                                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                }
                                                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                var3 = 4i32;
                                                            } else {
                                                                let var97 = var0;
                                                                let var98 = self.func54(imports, var97, 13i32); // wasm/cpu/opcodes/isOpcode
                                                                if var98 != 0 {
                                                                    let var99 = self.global4;
                                                                    self.func57(imports, var99, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                    let var100 = self.global4;
                                                                    self.global4 = var100.wrapping_sub(1i32) & 255i32;
                                                                    let var101 = self.global4;
                                                                    if var101 != 0 {
                                                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                    } else {
                                                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                    }
                                                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var102 = var0;
                                                                    let var103 = self.func54(imports, var102, 14i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var103 != 0 {
                                                                        let var104 = var1;
                                                                        self.global4 = var104;
                                                                        let var105 = self.global0;
                                                                        self.global0 = var105.wrapping_add(1i32) & 65535i32;
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var106 = var0;
                                                                        let var107 = self.func54(imports, var106, 15i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var107 != 0 {
                                                                            let var108 = self.global1;
                                                                            if ((var108 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
                                                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                                                            } else {
                                                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                                                            }
                                                                            let var109 = self.global1;
                                                                            let var110 = self.func64(imports, var109); // wasm/helpers/index/rotateByteRight
                                                                            self.global1 = var110;
                                                                            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var111 = var6;
        if (var111 == 1i32) as i32 != 0 {
            let var112 = var0;
            let var113 = self.func54(imports, var112, 16i32); // wasm/cpu/opcodes/isOpcode
            if var113 != 0 {
                let var114 = self.global0;
                self.global0 = var114.wrapping_add(1i32) & 65535i32;
                var3 = 4i32;
            } else {
                let var115 = var0;
                let var116 = self.func54(imports, var115, 17i32); // wasm/cpu/opcodes/isOpcode
                if var116 != 0 {
                    let var117 = var5;
                    let var118 = self.func9(imports, var117); // wasm/helpers/index/splitHighByte
                    self.global5 = var118;
                    let var119 = var5;
                    let var120 = self.func10(imports, var119); // wasm/helpers/index/splitLowByte
                    self.global6 = var120;
                    let var121 = self.global0;
                    self.global0 = var121.wrapping_add(2i32) & 65535i32;
                    var3 = 12i32;
                } else {
                    let var122 = var0;
                    let var123 = self.func54(imports, var122, 18i32); // wasm/cpu/opcodes/isOpcode
                    if var123 != 0 {
                        let var124 = self.global5;
                        let var125 = self.global6;
                        let var126 = self.func53(imports, var124, var125); // wasm/helpers/index/concatenateBytes
                        let var127 = self.global1;
                        self.func45(imports, var126, var127); // wasm/memory/store/eightBitStoreIntoGBMemory
                        var3 = 8i32;
                    } else {
                        let var128 = var0;
                        let var129 = self.func54(imports, var128, 19i32); // wasm/cpu/opcodes/isOpcode
                        if var129 != 0 {
                            let var130 = self.global5;
                            let var131 = self.global6;
                            let var132 = self.func53(imports, var130, var131); // wasm/helpers/index/concatenateBytes
                            var2 = var132.wrapping_add(1i32) & 65535i32;
                            let var133 = var2;
                            let var134 = self.func9(imports, var133); // wasm/helpers/index/splitHighByte
                            self.global5 = var134;
                            let var135 = var2;
                            let var136 = self.func10(imports, var135); // wasm/helpers/index/splitLowByte
                            self.global6 = var136;
                            var3 = 8i32;
                        } else {
                            let var137 = var0;
                            let var138 = self.func54(imports, var137, 20i32); // wasm/cpu/opcodes/isOpcode
                            if var138 != 0 {
                                let var139 = self.global5;
                                self.func57(imports, var139, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                let var140 = self.global5;
                                self.global5 = var140.wrapping_add(1i32) & 255i32;
                                let var141 = self.global5;
                                if var141 != 0 {
                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                } else {
                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                }
                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                var3 = 4i32;
                            } else {
                                let var142 = var0;
                                let var143 = self.func54(imports, var142, 21i32); // wasm/cpu/opcodes/isOpcode
                                if var143 != 0 {
                                    let var144 = self.global5;
                                    self.func57(imports, var144, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                    let var145 = self.global5;
                                    self.global5 = var145.wrapping_sub(1i32) & 255i32;
                                    let var146 = self.global5;
                                    if var146 != 0 {
                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                    } else {
                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                    }
                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                    var3 = 4i32;
                                } else {
                                    let var147 = var0;
                                    let var148 = self.func54(imports, var147, 22i32); // wasm/cpu/opcodes/isOpcode
                                    if var148 != 0 {
                                        let var149 = var1;
                                        self.global5 = var149;
                                        let var150 = self.global0;
                                        self.global0 = var150.wrapping_add(1i32) & 65535i32;
                                        var3 = 8i32;
                                    } else {
                                        let var151 = var0;
                                        let var152 = self.func54(imports, var151, 23i32); // wasm/cpu/opcodes/isOpcode
                                        if var152 != 0 {
                                            var2 = 0i32;
                                            let var153 = self.global1;
                                            if (var153 & 128i32 == 128i32) as i32 != 0 {
                                                var2 = 1i32;
                                            }
                                            let var154 = self.global1;
                                            let var155 = self.func66(imports, var154); // wasm/helpers/index/rotateByteLeftThroughCarry
                                            self.global1 = var155;
                                            let var156 = var2;
                                            if var156 != 0 {
                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            var3 = 4i32;
                                        } else {
                                            let var157 = var0;
                                            let var158 = self.func54(imports, var157, 24i32); // wasm/cpu/opcodes/isOpcode
                                            if var158 != 0 {
                                                let var159 = var1;
                                                self.func67(imports, var159); // wasm/cpu/index/relativeJump
                                                var3 = 12i32;
                                            } else {
                                                let var160 = var0;
                                                let var161 = self.func54(imports, var160, 25i32); // wasm/cpu/opcodes/isOpcode
                                                if var161 != 0 {
                                                    let var162 = self.global7;
                                                    let var163 = self.global8;
                                                    let var164 = self.func53(imports, var162, var163); // wasm/helpers/index/concatenateBytes
                                                    var2 = var164;
                                                    let var165 = var2;
                                                    let var166 = self.global5;
                                                    let var167 = self.global6;
                                                    let var168 = self.func53(imports, var166, var167); // wasm/helpers/index/concatenateBytes
                                                    var4 = var168;
                                                    let var169 = var4;
                                                    self.func63(imports, var165, var169, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                                    let var170 = var2;
                                                    let var171 = var4;
                                                    var3 = var170.wrapping_add(var171) & 65535i32;
                                                    let var172 = var3;
                                                    let var173 = self.func9(imports, var172); // wasm/helpers/index/splitHighByte
                                                    self.global7 = var173;
                                                    let var174 = var3;
                                                    let var175 = self.func10(imports, var174); // wasm/helpers/index/splitLowByte
                                                    self.global8 = var175;
                                                    self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                    var3 = 8i32;
                                                } else {
                                                    let var176 = var0;
                                                    let var177 = self.func54(imports, var176, 26i32); // wasm/cpu/opcodes/isOpcode
                                                    if var177 != 0 {
                                                        let var178 = self.global5;
                                                        let var179 = self.global6;
                                                        let var180 = self.func53(imports, var178, var179); // wasm/helpers/index/concatenateBytes
                                                        let var181 = self.func18(imports, var180); // wasm/memory/load/eightBitLoadFromGBMemory
                                                        self.global1 = var181;
                                                        var3 = 8i32;
                                                    } else {
                                                        let var182 = var0;
                                                        let var183 = self.func54(imports, var182, 27i32); // wasm/cpu/opcodes/isOpcode
                                                        if var183 != 0 {
                                                            let var184 = self.global5;
                                                            let var185 = self.global6;
                                                            let var186 = self.func53(imports, var184, var185); // wasm/helpers/index/concatenateBytes
                                                            var3 = var186.wrapping_sub(1i32) & 65535i32;
                                                            let var187 = var3;
                                                            let var188 = self.func9(imports, var187); // wasm/helpers/index/splitHighByte
                                                            self.global5 = var188;
                                                            let var189 = var3;
                                                            let var190 = self.func10(imports, var189); // wasm/helpers/index/splitLowByte
                                                            self.global6 = var190;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var191 = var0;
                                                            let var192 = self.func54(imports, var191, 28i32); // wasm/cpu/opcodes/isOpcode
                                                            if var192 != 0 {
                                                                let var193 = self.global6;
                                                                self.func57(imports, var193, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                let var194 = self.global6;
                                                                self.global6 = var194.wrapping_add(1i32) & 255i32;
                                                                let var195 = self.global6;
                                                                if var195 != 0 {
                                                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                } else {
                                                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                }
                                                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                var3 = 4i32;
                                                            } else {
                                                                let var196 = var0;
                                                                let var197 = self.func54(imports, var196, 29i32); // wasm/cpu/opcodes/isOpcode
                                                                if var197 != 0 {
                                                                    let var198 = self.global6;
                                                                    self.func57(imports, var198, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                    let var199 = self.global6;
                                                                    self.global6 = var199.wrapping_sub(1i32) & 255i32;
                                                                    let var200 = self.global6;
                                                                    if var200 != 0 {
                                                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                    } else {
                                                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                    }
                                                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var201 = var0;
                                                                    let var202 = self.func54(imports, var201, 30i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var202 != 0 {
                                                                        let var203 = var1;
                                                                        self.global6 = var203;
                                                                        let var204 = self.global0;
                                                                        self.global0 = var204.wrapping_add(1i32) & 65535i32;
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var205 = var0;
                                                                        let var206 = self.func54(imports, var205, 31i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var206 != 0 {
                                                                            var3 = 0i32;
                                                                            let var207 = self.global1;
                                                                            if (var207 & 1i32 == 1i32) as i32 != 0 {
                                                                                var3 = 1i32;
                                                                            }
                                                                            let var208 = self.global1;
                                                                            let var209 = self.func68(imports, var208); // wasm/helpers/index/rotateByteRightThroughCarry
                                                                            self.global1 = var209;
                                                                            let var210 = var3;
                                                                            if var210 != 0 {
                                                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                                                            } else {
                                                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                                                            }
                                                                            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var211 = var6;
        if (var211 == 2i32) as i32 != 0 {
            let var212 = var0;
            let var213 = self.func54(imports, var212, 32i32); // wasm/cpu/opcodes/isOpcode
            if var213 != 0 {
                let var214 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                let var215: i32;
                if var214 != 0 {
                    let var216 = self.global0;
                    self.global0 = var216.wrapping_add(1i32) & 65535i32;
                    var215 = 8i32;
                } else {
                    let var217 = var1;
                    self.func67(imports, var217); // wasm/cpu/index/relativeJump
                    var215 = 12i32;
                }
                var3 = var215;
            } else {
                let var218 = var0;
                let var219 = self.func54(imports, var218, 33i32); // wasm/cpu/opcodes/isOpcode
                if var219 != 0 {
                    let var220 = var5;
                    let var221 = self.func9(imports, var220); // wasm/helpers/index/splitHighByte
                    self.global7 = var221;
                    let var222 = var5;
                    let var223 = self.func10(imports, var222); // wasm/helpers/index/splitLowByte
                    self.global8 = var223;
                    var3 = 12i32;
                    let var224 = self.global0;
                    self.global0 = var224.wrapping_add(2i32) & 65535i32;
                } else {
                    let var225 = var0;
                    let var226 = self.func54(imports, var225, 34i32); // wasm/cpu/opcodes/isOpcode
                    if var226 != 0 {
                        let var227 = self.global7;
                        let var228 = self.global8;
                        let var229 = self.func53(imports, var227, var228); // wasm/helpers/index/concatenateBytes
                        var3 = var229;
                        let var230 = var3;
                        let var231 = self.global1;
                        self.func45(imports, var230, var231); // wasm/memory/store/eightBitStoreIntoGBMemory
                        let var232 = var3;
                        var3 = var232.wrapping_add(1i32) & 65535i32;
                        let var233 = var3;
                        let var234 = self.func9(imports, var233); // wasm/helpers/index/splitHighByte
                        self.global7 = var234;
                        let var235 = var3;
                        let var236 = self.func10(imports, var235); // wasm/helpers/index/splitLowByte
                        self.global8 = var236;
                        var3 = 8i32;
                    } else {
                        let var237 = var0;
                        let var238 = self.func54(imports, var237, 35i32); // wasm/cpu/opcodes/isOpcode
                        if var238 != 0 {
                            let var239 = self.global7;
                            let var240 = self.global8;
                            let var241 = self.func53(imports, var239, var240); // wasm/helpers/index/concatenateBytes
                            var3 = var241.wrapping_add(1i32) & 65535i32;
                            let var242 = var3;
                            let var243 = self.func9(imports, var242); // wasm/helpers/index/splitHighByte
                            self.global7 = var243;
                            let var244 = var3;
                            let var245 = self.func10(imports, var244); // wasm/helpers/index/splitLowByte
                            self.global8 = var245;
                            var3 = 8i32;
                        } else {
                            let var246 = var0;
                            let var247 = self.func54(imports, var246, 36i32); // wasm/cpu/opcodes/isOpcode
                            if var247 != 0 {
                                let var248 = self.global7;
                                self.func57(imports, var248, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                let var249 = self.global7;
                                self.global7 = var249.wrapping_add(1i32) & 255i32;
                                let var250 = self.global7;
                                if var250 != 0 {
                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                } else {
                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                }
                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                var3 = 4i32;
                            } else {
                                let var251 = var0;
                                let var252 = self.func54(imports, var251, 37i32); // wasm/cpu/opcodes/isOpcode
                                if var252 != 0 {
                                    let var253 = self.global7;
                                    self.func57(imports, var253, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                    let var254 = self.global7;
                                    self.global7 = var254.wrapping_sub(1i32) & 255i32;
                                    let var255 = self.global7;
                                    if var255 != 0 {
                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                    } else {
                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                    }
                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                    var3 = 4i32;
                                } else {
                                    let var256 = var0;
                                    let var257 = self.func54(imports, var256, 38i32); // wasm/cpu/opcodes/isOpcode
                                    if var257 != 0 {
                                        let var258 = var1;
                                        self.global7 = var258;
                                        let var259 = self.global0;
                                        self.global0 = var259.wrapping_add(1i32) & 65535i32;
                                        var3 = 8i32;
                                    } else {
                                        let var260 = var0;
                                        let var261 = self.func54(imports, var260, 39i32); // wasm/cpu/opcodes/isOpcode
                                        if var261 != 0 {
                                            var4 = 0i32;
                                            let var262 = self.func70(imports); // wasm/cpu/flags/getHalfCarryFlag
                                            if (var262 as u32 > 0i32 as u32) as i32 != 0 {
                                                var4 = 6i32;
                                            }
                                            let var263 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                            if (var263 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var264 = var4;
                                                var4 = (var264 | 96i32) & 255i32;
                                            }
                                            let var265 = self.func71(imports); // wasm/cpu/flags/getSubtractFlag
                                            let var266: i32;
                                            if (var265 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var267 = self.global1;
                                                let var268 = var4;
                                                var266 = var267.wrapping_sub(var268) & 255i32;
                                            } else {
                                                let var269 = self.global1;
                                                if ((var269 & 15i32) as u32 > 9i32 as u32) as i32 != 0 {
                                                    let var270 = var4;
                                                    var4 = (var270 | 6i32) & 255i32;
                                                }
                                                let var271 = self.global1;
                                                if (var271 as u32 > 153i32 as u32) as i32 != 0 {
                                                    let var272 = var4;
                                                    var4 = (var272 | 96i32) & 255i32;
                                                }
                                                let var273 = self.global1;
                                                let var274 = var4;
                                                var266 = var273.wrapping_add(var274) & 255i32;
                                            }
                                            var3 = var266;
                                            let var275 = var3;
                                            if var275 != 0 {
                                                self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            } else {
                                                self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                            }
                                            let var276 = var4;
                                            if var276 & 96i32 != 0 {
                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            let var277 = var3;
                                            self.global1 = var277;
                                            var3 = 4i32;
                                        } else {
                                            let var278 = var0;
                                            let var279 = self.func54(imports, var278, 40i32); // wasm/cpu/opcodes/isOpcode
                                            if var279 != 0 {
                                                let var280 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                                                let var281: i32;
                                                if (var280 as u32 > 0i32 as u32) as i32 != 0 {
                                                    let var282 = var1;
                                                    self.func67(imports, var282); // wasm/cpu/index/relativeJump
                                                    var281 = 12i32;
                                                } else {
                                                    let var283 = self.global0;
                                                    self.global0 = var283.wrapping_add(1i32) & 65535i32;
                                                    var281 = 8i32;
                                                }
                                                var3 = var281;
                                            } else {
                                                let var284 = var0;
                                                let var285 = self.func54(imports, var284, 41i32); // wasm/cpu/opcodes/isOpcode
                                                if var285 != 0 {
                                                    let var286 = self.global7;
                                                    let var287 = self.global8;
                                                    let var288 = self.func53(imports, var286, var287); // wasm/helpers/index/concatenateBytes
                                                    var4 = var288;
                                                    let var289 = var4;
                                                    let var290 = var4;
                                                    self.func63(imports, var289, var290, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                                    let var291 = var4;
                                                    var4 = var291.wrapping_mul(2i32) & 65535i32;
                                                    let var292 = var4;
                                                    let var293 = self.func9(imports, var292); // wasm/helpers/index/splitHighByte
                                                    self.global7 = var293;
                                                    let var294 = var4;
                                                    let var295 = self.func10(imports, var294); // wasm/helpers/index/splitLowByte
                                                    self.global8 = var295;
                                                    self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                    var3 = 8i32;
                                                } else {
                                                    let var296 = var0;
                                                    let var297 = self.func54(imports, var296, 42i32); // wasm/cpu/opcodes/isOpcode
                                                    if var297 != 0 {
                                                        let var298 = self.global7;
                                                        let var299 = self.global8;
                                                        let var300 = self.func53(imports, var298, var299); // wasm/helpers/index/concatenateBytes
                                                        var4 = var300;
                                                        let var301 = var4;
                                                        let var302 = self.func18(imports, var301); // wasm/memory/load/eightBitLoadFromGBMemory
                                                        self.global1 = var302;
                                                        let var303 = var4;
                                                        var4 = var303.wrapping_add(1i32) & 65535i32;
                                                        let var304 = var4;
                                                        let var305 = self.func9(imports, var304); // wasm/helpers/index/splitHighByte
                                                        self.global7 = var305;
                                                        let var306 = var4;
                                                        let var307 = self.func10(imports, var306); // wasm/helpers/index/splitLowByte
                                                        self.global8 = var307;
                                                        var3 = 8i32;
                                                    } else {
                                                        let var308 = var0;
                                                        let var309 = self.func54(imports, var308, 43i32); // wasm/cpu/opcodes/isOpcode
                                                        if var309 != 0 {
                                                            let var310 = self.global7;
                                                            let var311 = self.global8;
                                                            let var312 = self.func53(imports, var310, var311); // wasm/helpers/index/concatenateBytes
                                                            var4 = var312.wrapping_sub(1i32) & 65535i32;
                                                            let var313 = var4;
                                                            let var314 = self.func9(imports, var313); // wasm/helpers/index/splitHighByte
                                                            self.global7 = var314;
                                                            let var315 = var4;
                                                            let var316 = self.func10(imports, var315); // wasm/helpers/index/splitLowByte
                                                            self.global8 = var316;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var317 = var0;
                                                            let var318 = self.func54(imports, var317, 44i32); // wasm/cpu/opcodes/isOpcode
                                                            if var318 != 0 {
                                                                let var319 = self.global8;
                                                                self.func57(imports, var319, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                let var320 = self.global8;
                                                                self.global8 = var320.wrapping_add(1i32) & 255i32;
                                                                let var321 = self.global8;
                                                                if var321 != 0 {
                                                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                } else {
                                                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                }
                                                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                var3 = 4i32;
                                                            } else {
                                                                let var322 = var0;
                                                                let var323 = self.func54(imports, var322, 45i32); // wasm/cpu/opcodes/isOpcode
                                                                if var323 != 0 {
                                                                    let var324 = self.global8;
                                                                    self.func57(imports, var324, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                    let var325 = self.global8;
                                                                    self.global8 = var325.wrapping_sub(1i32) & 255i32;
                                                                    let var326 = self.global8;
                                                                    if var326 != 0 {
                                                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                    } else {
                                                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                    }
                                                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var327 = var0;
                                                                    let var328 = self.func54(imports, var327, 46i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var328 != 0 {
                                                                        let var329 = var1;
                                                                        self.global8 = var329;
                                                                        var3 = 8i32;
                                                                        let var330 = self.global0;
                                                                        self.global0 = var330.wrapping_add(1i32) & 65535i32;
                                                                    } else {
                                                                        let var331 = var0;
                                                                        let var332 = self.func54(imports, var331, 47i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var332 != 0 {
                                                                            let var333 = self.global1;
                                                                            self.global1 = (var333 ^ -1i32) & 255i32;
                                                                            self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                                            self.func56(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var334 = var6;
        if (var334 == 3i32) as i32 != 0 {
            let var335 = var0;
            let var336 = self.func54(imports, var335, 48i32); // wasm/cpu/opcodes/isOpcode
            if var336 != 0 {
                let var337 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                let var338: i32;
                if var337 != 0 {
                    let var339 = self.global0;
                    self.global0 = var339.wrapping_add(1i32) & 65535i32;
                    var338 = 8i32;
                } else {
                    let var340 = var1;
                    self.func67(imports, var340); // wasm/cpu/index/relativeJump
                    var338 = 12i32;
                }
                var3 = var338;
            } else {
                let var341 = var0;
                let var342 = self.func54(imports, var341, 49i32); // wasm/cpu/opcodes/isOpcode
                if var342 != 0 {
                    let var343 = var5;
                    self.global9 = var343;
                    let var344 = self.global0;
                    self.global0 = var344.wrapping_add(2i32) & 65535i32;
                    var3 = 12i32;
                } else {
                    let var345 = var0;
                    let var346 = self.func54(imports, var345, 50i32); // wasm/cpu/opcodes/isOpcode
                    if var346 != 0 {
                        let var347 = self.global7;
                        let var348 = self.global8;
                        let var349 = self.func53(imports, var347, var348); // wasm/helpers/index/concatenateBytes
                        var4 = var349;
                        let var350 = var4;
                        let var351 = self.global1;
                        self.func45(imports, var350, var351); // wasm/memory/store/eightBitStoreIntoGBMemory
                        let var352 = var4;
                        var4 = var352.wrapping_sub(1i32) & 65535i32;
                        let var353 = var4;
                        let var354 = self.func9(imports, var353); // wasm/helpers/index/splitHighByte
                        self.global7 = var354;
                        let var355 = var4;
                        let var356 = self.func10(imports, var355); // wasm/helpers/index/splitLowByte
                        self.global8 = var356;
                        var3 = 8i32;
                    } else {
                        let var357 = var0;
                        let var358 = self.func54(imports, var357, 51i32); // wasm/cpu/opcodes/isOpcode
                        if var358 != 0 {
                            let var359 = self.global9;
                            self.global9 = var359.wrapping_add(1i32) & 65535i32;
                            var3 = 8i32;
                        } else {
                            let var360 = var0;
                            let var361 = self.func54(imports, var360, 52i32); // wasm/cpu/opcodes/isOpcode
                            if var361 != 0 {
                                let var362 = self.global7;
                                let var363 = self.global8;
                                let var364 = self.func53(imports, var362, var363); // wasm/helpers/index/concatenateBytes
                                var4 = var364;
                                let var365 = var4;
                                let var366 = self.func18(imports, var365); // wasm/memory/load/eightBitLoadFromGBMemory
                                var3 = var366;
                                let var367 = var3;
                                var2 = 1i32;
                                let var368 = var2;
                                self.func57(imports, var367, var368); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                let var369 = var3;
                                var3 = var369.wrapping_add(1i32) & 255i32;
                                let var370 = var3;
                                if var370 != 0 {
                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                } else {
                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                }
                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                let var371 = var4;
                                let var372 = var3;
                                self.func45(imports, var371, var372); // wasm/memory/store/eightBitStoreIntoGBMemory
                                var3 = 12i32;
                            } else {
                                let var373 = var0;
                                let var374 = self.func54(imports, var373, 53i32); // wasm/cpu/opcodes/isOpcode
                                if var374 != 0 {
                                    let var375 = self.global7;
                                    let var376 = self.global8;
                                    let var377 = self.func53(imports, var375, var376); // wasm/helpers/index/concatenateBytes
                                    var2 = var377;
                                    let var378 = var2;
                                    let var379 = self.func18(imports, var378); // wasm/memory/load/eightBitLoadFromGBMemory
                                    var3 = var379;
                                    let var380 = var3;
                                    self.func57(imports, var380, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                    let var381 = var3;
                                    var3 = var381.wrapping_sub(1i32) & 255i32;
                                    let var382 = var3;
                                    if var382 != 0 {
                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                    } else {
                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                    }
                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                    let var383 = var2;
                                    let var384 = var3;
                                    self.func45(imports, var383, var384); // wasm/memory/store/eightBitStoreIntoGBMemory
                                    var3 = 12i32;
                                } else {
                                    let var385 = var0;
                                    let var386 = self.func54(imports, var385, 54i32); // wasm/cpu/opcodes/isOpcode
                                    if var386 != 0 {
                                        let var387 = self.global7;
                                        let var388 = self.global8;
                                        let var389 = self.func53(imports, var387, var388); // wasm/helpers/index/concatenateBytes
                                        let var390 = var1;
                                        self.func45(imports, var389, var390); // wasm/memory/store/eightBitStoreIntoGBMemory
                                        let var391 = self.global0;
                                        self.global0 = var391.wrapping_add(1i32) & 65535i32;
                                        var3 = 12i32;
                                    } else {
                                        let var392 = var0;
                                        let var393 = self.func54(imports, var392, 55i32); // wasm/cpu/opcodes/isOpcode
                                        if var393 != 0 {
                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            var3 = 4i32;
                                        } else {
                                            let var394 = var0;
                                            let var395 = self.func54(imports, var394, 56i32); // wasm/cpu/opcodes/isOpcode
                                            if var395 != 0 {
                                                let var396 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                                let var397: i32;
                                                if (var396 == 1i32) as i32 != 0 {
                                                    let var398 = var1;
                                                    self.func67(imports, var398); // wasm/cpu/index/relativeJump
                                                    var397 = 12i32;
                                                } else {
                                                    let var399 = self.global0;
                                                    self.global0 = var399.wrapping_add(1i32) & 65535i32;
                                                    var397 = 8i32;
                                                }
                                                var3 = var397;
                                            } else {
                                                let var400 = var0;
                                                let var401 = self.func54(imports, var400, 57i32); // wasm/cpu/opcodes/isOpcode
                                                if var401 != 0 {
                                                    let var402 = self.global7;
                                                    let var403 = self.global8;
                                                    let var404 = self.func53(imports, var402, var403); // wasm/helpers/index/concatenateBytes
                                                    var3 = var404;
                                                    let var405 = var3;
                                                    let var406 = self.global9;
                                                    self.func63(imports, var405, var406, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                                    let var407 = var3;
                                                    let var408 = self.global9;
                                                    var2 = var407.wrapping_add(var408) & 65535i32;
                                                    let var409 = var2;
                                                    let var410 = self.func9(imports, var409); // wasm/helpers/index/splitHighByte
                                                    self.global7 = var410;
                                                    let var411 = var2;
                                                    let var412 = self.func10(imports, var411); // wasm/helpers/index/splitLowByte
                                                    self.global8 = var412;
                                                    self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                    var3 = 8i32;
                                                } else {
                                                    let var413 = var0;
                                                    let var414 = self.func54(imports, var413, 58i32); // wasm/cpu/opcodes/isOpcode
                                                    if var414 != 0 {
                                                        let var415 = self.global7;
                                                        let var416 = self.global8;
                                                        let var417 = self.func53(imports, var415, var416); // wasm/helpers/index/concatenateBytes
                                                        var2 = var417;
                                                        let var418 = var2;
                                                        let var419 = self.func18(imports, var418); // wasm/memory/load/eightBitLoadFromGBMemory
                                                        self.global1 = var419;
                                                        let var420 = var2;
                                                        var2 = var420.wrapping_sub(1i32) & 65535i32;
                                                        let var421 = var2;
                                                        let var422 = self.func9(imports, var421); // wasm/helpers/index/splitHighByte
                                                        self.global7 = var422;
                                                        let var423 = var2;
                                                        let var424 = self.func10(imports, var423); // wasm/helpers/index/splitLowByte
                                                        self.global8 = var424;
                                                        var3 = 8i32;
                                                    } else {
                                                        let var425 = var0;
                                                        let var426 = self.func54(imports, var425, 59i32); // wasm/cpu/opcodes/isOpcode
                                                        if var426 != 0 {
                                                            let var427 = self.global9;
                                                            self.global9 = var427.wrapping_sub(1i32) & 65535i32;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var428 = var0;
                                                            let var429 = self.func54(imports, var428, 60i32); // wasm/cpu/opcodes/isOpcode
                                                            if var429 != 0 {
                                                                let var430 = self.global1;
                                                                self.func57(imports, var430, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                let var431 = self.global1;
                                                                self.global1 = var431.wrapping_add(1i32) & 255i32;
                                                                let var432 = self.global1;
                                                                if var432 != 0 {
                                                                    self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                } else {
                                                                    self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                }
                                                                self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                var3 = 4i32;
                                                            } else {
                                                                let var433 = var0;
                                                                let var434 = self.func54(imports, var433, 61i32); // wasm/cpu/opcodes/isOpcode
                                                                if var434 != 0 {
                                                                    let var435 = self.global1;
                                                                    self.func57(imports, var435, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                                    let var436 = self.global1;
                                                                    self.global1 = var436.wrapping_sub(1i32) & 255i32;
                                                                    let var437 = self.global1;
                                                                    if var437 != 0 {
                                                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                                    } else {
                                                                        self.func58(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                                    }
                                                                    self.func59(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var438 = var0;
                                                                    let var439 = self.func54(imports, var438, 62i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var439 != 0 {
                                                                        let var440 = var1;
                                                                        self.global1 = var440;
                                                                        let var441 = self.global0;
                                                                        self.global0 = var441.wrapping_add(1i32) & 65535i32;
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var442 = var0;
                                                                        let var443 = self.func54(imports, var442, 63i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var443 != 0 {
                                                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                                            self.func56(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                                                            let var444 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                                                            if (var444 as u32 > 0i32 as u32) as i32 != 0 {
                                                                                self.func60(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                                                            } else {
                                                                                self.func60(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                                                            }
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var445 = var6;
        if (var445 == 4i32) as i32 != 0 {
            let var446 = var0;
            let var447 = self.func54(imports, var446, 64i32); // wasm/cpu/opcodes/isOpcode
            if var447 != 0 {
                var3 = 4i32;
            } else {
                let var448 = var0;
                let var449 = self.func54(imports, var448, 65i32); // wasm/cpu/opcodes/isOpcode
                if var449 != 0 {
                    let var450 = self.global4;
                    self.global3 = var450;
                    var3 = 4i32;
                } else {
                    let var451 = var0;
                    let var452 = self.func54(imports, var451, 66i32); // wasm/cpu/opcodes/isOpcode
                    if var452 != 0 {
                        let var453 = self.global5;
                        self.global3 = var453;
                        var3 = 4i32;
                    } else {
                        let var454 = var0;
                        let var455 = self.func54(imports, var454, 67i32); // wasm/cpu/opcodes/isOpcode
                        if var455 != 0 {
                            let var456 = self.global6;
                            self.global3 = var456;
                            var3 = 4i32;
                        } else {
                            let var457 = var0;
                            let var458 = self.func54(imports, var457, 68i32); // wasm/cpu/opcodes/isOpcode
                            if var458 != 0 {
                                let var459 = self.global7;
                                self.global3 = var459;
                                var3 = 4i32;
                            } else {
                                let var460 = var0;
                                let var461 = self.func54(imports, var460, 69i32); // wasm/cpu/opcodes/isOpcode
                                if var461 != 0 {
                                    let var462 = self.global8;
                                    self.global3 = var462;
                                    var3 = 4i32;
                                } else {
                                    let var463 = var0;
                                    let var464 = self.func54(imports, var463, 70i32); // wasm/cpu/opcodes/isOpcode
                                    if var464 != 0 {
                                        let var465 = self.global7;
                                        let var466 = self.global8;
                                        let var467 = self.func53(imports, var465, var466); // wasm/helpers/index/concatenateBytes
                                        let var468 = self.func18(imports, var467); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.global3 = var468;
                                        var3 = 8i32;
                                    } else {
                                        let var469 = var0;
                                        let var470 = self.func54(imports, var469, 71i32); // wasm/cpu/opcodes/isOpcode
                                        if var470 != 0 {
                                            let var471 = self.global1;
                                            self.global3 = var471;
                                            var3 = 4i32;
                                        } else {
                                            let var472 = var0;
                                            let var473 = self.func54(imports, var472, 72i32); // wasm/cpu/opcodes/isOpcode
                                            if var473 != 0 {
                                                let var474 = self.global3;
                                                self.global4 = var474;
                                                var3 = 4i32;
                                            } else {
                                                let var475 = var0;
                                                let var476 = self.func54(imports, var475, 73i32); // wasm/cpu/opcodes/isOpcode
                                                if var476 != 0 {
                                                    var3 = 4i32;
                                                } else {
                                                    let var477 = var0;
                                                    let var478 = self.func54(imports, var477, 74i32); // wasm/cpu/opcodes/isOpcode
                                                    if var478 != 0 {
                                                        let var479 = self.global5;
                                                        self.global4 = var479;
                                                        var3 = 4i32;
                                                    } else {
                                                        let var480 = var0;
                                                        let var481 = self.func54(imports, var480, 75i32); // wasm/cpu/opcodes/isOpcode
                                                        if var481 != 0 {
                                                            let var482 = self.global6;
                                                            self.global4 = var482;
                                                            var3 = 4i32;
                                                        } else {
                                                            let var483 = var0;
                                                            let var484 = self.func54(imports, var483, 76i32); // wasm/cpu/opcodes/isOpcode
                                                            if var484 != 0 {
                                                                let var485 = self.global7;
                                                                self.global4 = var485;
                                                                var3 = 4i32;
                                                            } else {
                                                                let var486 = var0;
                                                                let var487 = self.func54(imports, var486, 77i32); // wasm/cpu/opcodes/isOpcode
                                                                if var487 != 0 {
                                                                    let var488 = self.global8;
                                                                    self.global4 = var488;
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var489 = var0;
                                                                    let var490 = self.func54(imports, var489, 78i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var490 != 0 {
                                                                        let var491 = self.global7;
                                                                        let var492 = self.global8;
                                                                        let var493 = self.func53(imports, var491, var492); // wasm/helpers/index/concatenateBytes
                                                                        let var494 = self.func18(imports, var493); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.global4 = var494;
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var495 = var0;
                                                                        let var496 = self.func54(imports, var495, 79i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var496 != 0 {
                                                                            let var497 = self.global1;
                                                                            self.global4 = var497;
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var498 = var6;
        if (var498 == 5i32) as i32 != 0 {
            let var499 = var0;
            let var500 = self.func54(imports, var499, 80i32); // wasm/cpu/opcodes/isOpcode
            if var500 != 0 {
                let var501 = self.global3;
                self.global5 = var501;
                var3 = 4i32;
            } else {
                let var502 = var0;
                let var503 = self.func54(imports, var502, 81i32); // wasm/cpu/opcodes/isOpcode
                if var503 != 0 {
                    let var504 = self.global4;
                    self.global5 = var504;
                    var3 = 4i32;
                } else {
                    let var505 = var0;
                    let var506 = self.func54(imports, var505, 82i32); // wasm/cpu/opcodes/isOpcode
                    if var506 != 0 {
                        var3 = 4i32;
                    } else {
                        let var507 = var0;
                        let var508 = self.func54(imports, var507, 83i32); // wasm/cpu/opcodes/isOpcode
                        if var508 != 0 {
                            let var509 = self.global6;
                            self.global5 = var509;
                            var3 = 4i32;
                        } else {
                            let var510 = var0;
                            let var511 = self.func54(imports, var510, 84i32); // wasm/cpu/opcodes/isOpcode
                            if var511 != 0 {
                                let var512 = self.global7;
                                self.global5 = var512;
                                var3 = 4i32;
                            } else {
                                let var513 = var0;
                                let var514 = self.func54(imports, var513, 85i32); // wasm/cpu/opcodes/isOpcode
                                if var514 != 0 {
                                    let var515 = self.global8;
                                    self.global5 = var515;
                                    var3 = 4i32;
                                } else {
                                    let var516 = var0;
                                    let var517 = self.func54(imports, var516, 86i32); // wasm/cpu/opcodes/isOpcode
                                    if var517 != 0 {
                                        let var518 = self.global7;
                                        let var519 = self.global8;
                                        let var520 = self.func53(imports, var518, var519); // wasm/helpers/index/concatenateBytes
                                        let var521 = self.func18(imports, var520); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.global5 = var521;
                                        var3 = 8i32;
                                    } else {
                                        let var522 = var0;
                                        let var523 = self.func54(imports, var522, 87i32); // wasm/cpu/opcodes/isOpcode
                                        if var523 != 0 {
                                            let var524 = self.global1;
                                            self.global5 = var524;
                                            var3 = 4i32;
                                        } else {
                                            let var525 = var0;
                                            let var526 = self.func54(imports, var525, 88i32); // wasm/cpu/opcodes/isOpcode
                                            if var526 != 0 {
                                                let var527 = self.global3;
                                                self.global6 = var527;
                                                var3 = 4i32;
                                            } else {
                                                let var528 = var0;
                                                let var529 = self.func54(imports, var528, 89i32); // wasm/cpu/opcodes/isOpcode
                                                if var529 != 0 {
                                                    let var530 = self.global4;
                                                    self.global6 = var530;
                                                    var3 = 4i32;
                                                } else {
                                                    let var531 = var0;
                                                    let var532 = self.func54(imports, var531, 90i32); // wasm/cpu/opcodes/isOpcode
                                                    if var532 != 0 {
                                                        let var533 = self.global5;
                                                        self.global6 = var533;
                                                        var3 = 4i32;
                                                    } else {
                                                        let var534 = var0;
                                                        let var535 = self.func54(imports, var534, 91i32); // wasm/cpu/opcodes/isOpcode
                                                        if var535 != 0 {
                                                            var3 = 4i32;
                                                        } else {
                                                            let var536 = var0;
                                                            let var537 = self.func54(imports, var536, 92i32); // wasm/cpu/opcodes/isOpcode
                                                            if var537 != 0 {
                                                                let var538 = self.global7;
                                                                self.global6 = var538;
                                                                var3 = 4i32;
                                                            } else {
                                                                let var539 = var0;
                                                                let var540 = self.func54(imports, var539, 93i32); // wasm/cpu/opcodes/isOpcode
                                                                if var540 != 0 {
                                                                    let var541 = self.global8;
                                                                    self.global6 = var541;
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var542 = var0;
                                                                    let var543 = self.func54(imports, var542, 94i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var543 != 0 {
                                                                        let var544 = self.global7;
                                                                        let var545 = self.global8;
                                                                        let var546 = self.func53(imports, var544, var545); // wasm/helpers/index/concatenateBytes
                                                                        let var547 = self.func18(imports, var546); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.global6 = var547;
                                                                        var3 = 4i32;
                                                                    } else {
                                                                        let var548 = var0;
                                                                        let var549 = self.func54(imports, var548, 95i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var549 != 0 {
                                                                            let var550 = self.global1;
                                                                            self.global6 = var550;
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var551 = var6;
        if (var551 == 6i32) as i32 != 0 {
            let var552 = var0;
            let var553 = self.func54(imports, var552, 96i32); // wasm/cpu/opcodes/isOpcode
            if var553 != 0 {
                let var554 = self.global3;
                self.global7 = var554;
                var3 = 4i32;
            } else {
                let var555 = var0;
                let var556 = self.func54(imports, var555, 97i32); // wasm/cpu/opcodes/isOpcode
                if var556 != 0 {
                    let var557 = self.global4;
                    self.global7 = var557;
                    var3 = 4i32;
                } else {
                    let var558 = var0;
                    let var559 = self.func54(imports, var558, 98i32); // wasm/cpu/opcodes/isOpcode
                    if var559 != 0 {
                        let var560 = self.global5;
                        self.global7 = var560;
                        var3 = 4i32;
                    } else {
                        let var561 = var0;
                        let var562 = self.func54(imports, var561, 99i32); // wasm/cpu/opcodes/isOpcode
                        if var562 != 0 {
                            let var563 = self.global6;
                            self.global7 = var563;
                            var3 = 4i32;
                        } else {
                            let var564 = var0;
                            let var565 = self.func54(imports, var564, 100i32); // wasm/cpu/opcodes/isOpcode
                            if var565 != 0 {
                                var3 = 4i32;
                            } else {
                                let var566 = var0;
                                let var567 = self.func54(imports, var566, 101i32); // wasm/cpu/opcodes/isOpcode
                                if var567 != 0 {
                                    let var568 = self.global8;
                                    self.global7 = var568;
                                    var3 = 4i32;
                                } else {
                                    let var569 = var0;
                                    let var570 = self.func54(imports, var569, 102i32); // wasm/cpu/opcodes/isOpcode
                                    if var570 != 0 {
                                        let var571 = self.global7;
                                        let var572 = self.global8;
                                        let var573 = self.func53(imports, var571, var572); // wasm/helpers/index/concatenateBytes
                                        let var574 = self.func18(imports, var573); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.global7 = var574;
                                        var3 = 8i32;
                                    } else {
                                        let var575 = var0;
                                        let var576 = self.func54(imports, var575, 103i32); // wasm/cpu/opcodes/isOpcode
                                        if var576 != 0 {
                                            let var577 = self.global1;
                                            self.global7 = var577;
                                            var3 = 4i32;
                                        } else {
                                            let var578 = var0;
                                            let var579 = self.func54(imports, var578, 104i32); // wasm/cpu/opcodes/isOpcode
                                            if var579 != 0 {
                                                let var580 = self.global3;
                                                self.global8 = var580;
                                                var3 = 4i32;
                                            } else {
                                                let var581 = var0;
                                                let var582 = self.func54(imports, var581, 105i32); // wasm/cpu/opcodes/isOpcode
                                                if var582 != 0 {
                                                    let var583 = self.global4;
                                                    self.global8 = var583;
                                                    var3 = 4i32;
                                                } else {
                                                    let var584 = var0;
                                                    let var585 = self.func54(imports, var584, 106i32); // wasm/cpu/opcodes/isOpcode
                                                    if var585 != 0 {
                                                        let var586 = self.global5;
                                                        self.global8 = var586;
                                                        var3 = 4i32;
                                                    } else {
                                                        let var587 = var0;
                                                        let var588 = self.func54(imports, var587, 107i32); // wasm/cpu/opcodes/isOpcode
                                                        if var588 != 0 {
                                                            let var589 = self.global6;
                                                            self.global8 = var589;
                                                            var3 = 4i32;
                                                        } else {
                                                            let var590 = var0;
                                                            let var591 = self.func54(imports, var590, 108i32); // wasm/cpu/opcodes/isOpcode
                                                            if var591 != 0 {
                                                                let var592 = self.global7;
                                                                self.global8 = var592;
                                                                var3 = 4i32;
                                                            } else {
                                                                let var593 = var0;
                                                                let var594 = self.func54(imports, var593, 109i32); // wasm/cpu/opcodes/isOpcode
                                                                if var594 != 0 {
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var595 = var0;
                                                                    let var596 = self.func54(imports, var595, 110i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var596 != 0 {
                                                                        let var597 = self.global7;
                                                                        let var598 = self.global8;
                                                                        let var599 = self.func53(imports, var597, var598); // wasm/helpers/index/concatenateBytes
                                                                        let var600 = self.func18(imports, var599); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.global8 = var600;
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var601 = var0;
                                                                        let var602 = self.func54(imports, var601, 111i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var602 != 0 {
                                                                            let var603 = self.global1;
                                                                            self.global8 = var603;
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var604 = var6;
        if (var604 == 7i32) as i32 != 0 {
            let var605 = var0;
            let var606 = self.func54(imports, var605, 112i32); // wasm/cpu/opcodes/isOpcode
            if var606 != 0 {
                let var607 = self.global7;
                let var608 = self.global8;
                let var609 = self.func53(imports, var607, var608); // wasm/helpers/index/concatenateBytes
                let var610 = self.global3;
                self.func45(imports, var609, var610); // wasm/memory/store/eightBitStoreIntoGBMemory
                var3 = 8i32;
            } else {
                let var611 = var0;
                let var612 = self.func54(imports, var611, 113i32); // wasm/cpu/opcodes/isOpcode
                if var612 != 0 {
                    let var613 = self.global7;
                    let var614 = self.global8;
                    let var615 = self.func53(imports, var613, var614); // wasm/helpers/index/concatenateBytes
                    let var616 = self.global4;
                    self.func45(imports, var615, var616); // wasm/memory/store/eightBitStoreIntoGBMemory
                    var3 = 8i32;
                } else {
                    let var617 = var0;
                    let var618 = self.func54(imports, var617, 114i32); // wasm/cpu/opcodes/isOpcode
                    if var618 != 0 {
                        let var619 = self.global7;
                        let var620 = self.global8;
                        let var621 = self.func53(imports, var619, var620); // wasm/helpers/index/concatenateBytes
                        let var622 = self.global5;
                        self.func45(imports, var621, var622); // wasm/memory/store/eightBitStoreIntoGBMemory
                        var3 = 8i32;
                    } else {
                        let var623 = var0;
                        let var624 = self.func54(imports, var623, 115i32); // wasm/cpu/opcodes/isOpcode
                        if var624 != 0 {
                            let var625 = self.global7;
                            let var626 = self.global8;
                            let var627 = self.func53(imports, var625, var626); // wasm/helpers/index/concatenateBytes
                            let var628 = self.global6;
                            self.func45(imports, var627, var628); // wasm/memory/store/eightBitStoreIntoGBMemory
                            var3 = 8i32;
                        } else {
                            let var629 = var0;
                            let var630 = self.func54(imports, var629, 116i32); // wasm/cpu/opcodes/isOpcode
                            if var630 != 0 {
                                let var631 = self.global7;
                                let var632 = self.global8;
                                let var633 = self.func53(imports, var631, var632); // wasm/helpers/index/concatenateBytes
                                let var634 = self.global7;
                                self.func45(imports, var633, var634); // wasm/memory/store/eightBitStoreIntoGBMemory
                                var3 = 8i32;
                            } else {
                                let var635 = var0;
                                let var636 = self.func54(imports, var635, 117i32); // wasm/cpu/opcodes/isOpcode
                                if var636 != 0 {
                                    let var637 = self.global7;
                                    let var638 = self.global8;
                                    let var639 = self.func53(imports, var637, var638); // wasm/helpers/index/concatenateBytes
                                    let var640 = self.global8;
                                    self.func45(imports, var639, var640); // wasm/memory/store/eightBitStoreIntoGBMemory
                                    var3 = 8i32;
                                } else {
                                    let var641 = var0;
                                    let var642 = self.func54(imports, var641, 118i32); // wasm/cpu/opcodes/isOpcode
                                    if var642 != 0 {
                                        self.global53 = 1i32;
                                        var3 = 4i32;
                                    } else {
                                        let var643 = var0;
                                        let var644 = self.func54(imports, var643, 119i32); // wasm/cpu/opcodes/isOpcode
                                        if var644 != 0 {
                                            let var645 = self.global7;
                                            let var646 = self.global8;
                                            let var647 = self.func53(imports, var645, var646); // wasm/helpers/index/concatenateBytes
                                            let var648 = self.global1;
                                            self.func45(imports, var647, var648); // wasm/memory/store/eightBitStoreIntoGBMemory
                                            var3 = 8i32;
                                        } else {
                                            let var649 = var0;
                                            let var650 = self.func54(imports, var649, 120i32); // wasm/cpu/opcodes/isOpcode
                                            if var650 != 0 {
                                                let var651 = self.global3;
                                                self.global1 = var651;
                                                var3 = 4i32;
                                            } else {
                                                let var652 = var0;
                                                let var653 = self.func54(imports, var652, 121i32); // wasm/cpu/opcodes/isOpcode
                                                if var653 != 0 {
                                                    let var654 = self.global4;
                                                    self.global1 = var654;
                                                    var3 = 4i32;
                                                } else {
                                                    let var655 = var0;
                                                    let var656 = self.func54(imports, var655, 122i32); // wasm/cpu/opcodes/isOpcode
                                                    if var656 != 0 {
                                                        let var657 = self.global5;
                                                        self.global1 = var657;
                                                        var3 = 4i32;
                                                    } else {
                                                        let var658 = var0;
                                                        let var659 = self.func54(imports, var658, 123i32); // wasm/cpu/opcodes/isOpcode
                                                        if var659 != 0 {
                                                            let var660 = self.global6;
                                                            self.global1 = var660;
                                                            var3 = 4i32;
                                                        } else {
                                                            let var661 = var0;
                                                            let var662 = self.func54(imports, var661, 124i32); // wasm/cpu/opcodes/isOpcode
                                                            if var662 != 0 {
                                                                let var663 = self.global7;
                                                                self.global1 = var663;
                                                                var3 = 4i32;
                                                            } else {
                                                                let var664 = var0;
                                                                let var665 = self.func54(imports, var664, 125i32); // wasm/cpu/opcodes/isOpcode
                                                                if var665 != 0 {
                                                                    let var666 = self.global8;
                                                                    self.global1 = var666;
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var667 = var0;
                                                                    let var668 = self.func54(imports, var667, 126i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var668 != 0 {
                                                                        let var669 = self.global7;
                                                                        let var670 = self.global8;
                                                                        let var671 = self.func53(imports, var669, var670); // wasm/helpers/index/concatenateBytes
                                                                        let var672 = self.func18(imports, var671); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.global1 = var672;
                                                                        var3 = 4i32;
                                                                    } else {
                                                                        let var673 = var0;
                                                                        let var674 = self.func54(imports, var673, 127i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var674 != 0 {
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var675 = var6;
        if (var675 == 8i32) as i32 != 0 {
            let var676 = var0;
            let var677 = self.func54(imports, var676, 128i32); // wasm/cpu/opcodes/isOpcode
            if var677 != 0 {
                let var678 = self.global3;
                self.func73(imports, var678); // wasm/cpu/instructions/addARegister
                var3 = 4i32;
            } else {
                let var679 = var0;
                let var680 = self.func54(imports, var679, 129i32); // wasm/cpu/opcodes/isOpcode
                if var680 != 0 {
                    let var681 = self.global4;
                    self.func73(imports, var681); // wasm/cpu/instructions/addARegister
                    var3 = 4i32;
                } else {
                    let var682 = var0;
                    let var683 = self.func54(imports, var682, 130i32); // wasm/cpu/opcodes/isOpcode
                    if var683 != 0 {
                        let var684 = self.global5;
                        self.func73(imports, var684); // wasm/cpu/instructions/addARegister
                        var3 = 4i32;
                    } else {
                        let var685 = var0;
                        let var686 = self.func54(imports, var685, 131i32); // wasm/cpu/opcodes/isOpcode
                        if var686 != 0 {
                            let var687 = self.global6;
                            self.func73(imports, var687); // wasm/cpu/instructions/addARegister
                            var3 = 4i32;
                        } else {
                            let var688 = var0;
                            let var689 = self.func54(imports, var688, 132i32); // wasm/cpu/opcodes/isOpcode
                            if var689 != 0 {
                                let var690 = self.global7;
                                self.func73(imports, var690); // wasm/cpu/instructions/addARegister
                                var3 = 4i32;
                            } else {
                                let var691 = var0;
                                let var692 = self.func54(imports, var691, 133i32); // wasm/cpu/opcodes/isOpcode
                                if var692 != 0 {
                                    let var693 = self.global8;
                                    self.func73(imports, var693); // wasm/cpu/instructions/addARegister
                                    var3 = 4i32;
                                } else {
                                    let var694 = var0;
                                    let var695 = self.func54(imports, var694, 134i32); // wasm/cpu/opcodes/isOpcode
                                    if var695 != 0 {
                                        let var696 = self.global7;
                                        let var697 = self.global8;
                                        let var698 = self.func53(imports, var696, var697); // wasm/helpers/index/concatenateBytes
                                        let var699 = self.func18(imports, var698); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.func73(imports, var699); // wasm/cpu/instructions/addARegister
                                        var3 = 8i32;
                                    } else {
                                        let var700 = var0;
                                        let var701 = self.func54(imports, var700, 135i32); // wasm/cpu/opcodes/isOpcode
                                        if var701 != 0 {
                                            let var702 = self.global1;
                                            self.func73(imports, var702); // wasm/cpu/instructions/addARegister
                                            var3 = 4i32;
                                        } else {
                                            let var703 = var0;
                                            let var704 = self.func54(imports, var703, 136i32); // wasm/cpu/opcodes/isOpcode
                                            if var704 != 0 {
                                                let var705 = self.global3;
                                                self.func74(imports, var705); // wasm/cpu/instructions/addAThroughCarryRegister
                                                var3 = 4i32;
                                            } else {
                                                let var706 = var0;
                                                let var707 = self.func54(imports, var706, 137i32); // wasm/cpu/opcodes/isOpcode
                                                if var707 != 0 {
                                                    let var708 = self.global4;
                                                    self.func74(imports, var708); // wasm/cpu/instructions/addAThroughCarryRegister
                                                    var3 = 4i32;
                                                } else {
                                                    let var709 = var0;
                                                    let var710 = self.func54(imports, var709, 138i32); // wasm/cpu/opcodes/isOpcode
                                                    if var710 != 0 {
                                                        let var711 = self.global5;
                                                        self.func74(imports, var711); // wasm/cpu/instructions/addAThroughCarryRegister
                                                        var3 = 4i32;
                                                    } else {
                                                        let var712 = var0;
                                                        let var713 = self.func54(imports, var712, 139i32); // wasm/cpu/opcodes/isOpcode
                                                        if var713 != 0 {
                                                            let var714 = self.global6;
                                                            self.func74(imports, var714); // wasm/cpu/instructions/addAThroughCarryRegister
                                                            var3 = 4i32;
                                                        } else {
                                                            let var715 = var0;
                                                            let var716 = self.func54(imports, var715, 140i32); // wasm/cpu/opcodes/isOpcode
                                                            if var716 != 0 {
                                                                let var717 = self.global7;
                                                                self.func74(imports, var717); // wasm/cpu/instructions/addAThroughCarryRegister
                                                                var3 = 4i32;
                                                            } else {
                                                                let var718 = var0;
                                                                let var719 = self.func54(imports, var718, 141i32); // wasm/cpu/opcodes/isOpcode
                                                                if var719 != 0 {
                                                                    let var720 = self.global8;
                                                                    self.func74(imports, var720); // wasm/cpu/instructions/addAThroughCarryRegister
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var721 = var0;
                                                                    let var722 = self.func54(imports, var721, 142i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var722 != 0 {
                                                                        let var723 = self.global7;
                                                                        let var724 = self.global8;
                                                                        let var725 = self.func53(imports, var723, var724); // wasm/helpers/index/concatenateBytes
                                                                        let var726 = self.func18(imports, var725); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.func74(imports, var726); // wasm/cpu/instructions/addAThroughCarryRegister
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var727 = var0;
                                                                        let var728 = self.func54(imports, var727, 143i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var728 != 0 {
                                                                            let var729 = self.global1;
                                                                            self.func74(imports, var729); // wasm/cpu/instructions/addAThroughCarryRegister
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var730 = var6;
        if (var730 == 9i32) as i32 != 0 {
            let var731 = var0;
            let var732 = self.func54(imports, var731, 144i32); // wasm/cpu/opcodes/isOpcode
            if var732 != 0 {
                let var733 = self.global3;
                self.func75(imports, var733); // wasm/cpu/instructions/subARegister
                var3 = 4i32;
            } else {
                let var734 = var0;
                let var735 = self.func54(imports, var734, 145i32); // wasm/cpu/opcodes/isOpcode
                if var735 != 0 {
                    let var736 = self.global4;
                    self.func75(imports, var736); // wasm/cpu/instructions/subARegister
                    var3 = 4i32;
                } else {
                    let var737 = var0;
                    let var738 = self.func54(imports, var737, 146i32); // wasm/cpu/opcodes/isOpcode
                    if var738 != 0 {
                        let var739 = self.global5;
                        self.func75(imports, var739); // wasm/cpu/instructions/subARegister
                        var3 = 4i32;
                    } else {
                        let var740 = var0;
                        let var741 = self.func54(imports, var740, 147i32); // wasm/cpu/opcodes/isOpcode
                        if var741 != 0 {
                            let var742 = self.global6;
                            self.func75(imports, var742); // wasm/cpu/instructions/subARegister
                            var3 = 4i32;
                        } else {
                            let var743 = var0;
                            let var744 = self.func54(imports, var743, 148i32); // wasm/cpu/opcodes/isOpcode
                            if var744 != 0 {
                                let var745 = self.global7;
                                self.func75(imports, var745); // wasm/cpu/instructions/subARegister
                                var3 = 4i32;
                            } else {
                                let var746 = var0;
                                let var747 = self.func54(imports, var746, 149i32); // wasm/cpu/opcodes/isOpcode
                                if var747 != 0 {
                                    let var748 = self.global8;
                                    self.func75(imports, var748); // wasm/cpu/instructions/subARegister
                                    var3 = 4i32;
                                } else {
                                    let var749 = var0;
                                    let var750 = self.func54(imports, var749, 150i32); // wasm/cpu/opcodes/isOpcode
                                    if var750 != 0 {
                                        let var751 = self.global7;
                                        let var752 = self.global8;
                                        let var753 = self.func53(imports, var751, var752); // wasm/helpers/index/concatenateBytes
                                        let var754 = self.func18(imports, var753); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.func75(imports, var754); // wasm/cpu/instructions/subARegister
                                        var3 = 8i32;
                                    } else {
                                        let var755 = var0;
                                        let var756 = self.func54(imports, var755, 151i32); // wasm/cpu/opcodes/isOpcode
                                        if var756 != 0 {
                                            let var757 = self.global1;
                                            self.func75(imports, var757); // wasm/cpu/instructions/subARegister
                                            var3 = 4i32;
                                        } else {
                                            let var758 = var0;
                                            let var759 = self.func54(imports, var758, 152i32); // wasm/cpu/opcodes/isOpcode
                                            if var759 != 0 {
                                                let var760 = self.global3;
                                                self.func76(imports, var760); // wasm/cpu/instructions/subAThroughCarryRegister
                                                var3 = 4i32;
                                            } else {
                                                let var761 = var0;
                                                let var762 = self.func54(imports, var761, 153i32); // wasm/cpu/opcodes/isOpcode
                                                if var762 != 0 {
                                                    let var763 = self.global4;
                                                    self.func76(imports, var763); // wasm/cpu/instructions/subAThroughCarryRegister
                                                    var3 = 4i32;
                                                } else {
                                                    let var764 = var0;
                                                    let var765 = self.func54(imports, var764, 154i32); // wasm/cpu/opcodes/isOpcode
                                                    if var765 != 0 {
                                                        let var766 = self.global5;
                                                        self.func76(imports, var766); // wasm/cpu/instructions/subAThroughCarryRegister
                                                        var3 = 4i32;
                                                    } else {
                                                        let var767 = var0;
                                                        let var768 = self.func54(imports, var767, 155i32); // wasm/cpu/opcodes/isOpcode
                                                        if var768 != 0 {
                                                            let var769 = self.global6;
                                                            self.func76(imports, var769); // wasm/cpu/instructions/subAThroughCarryRegister
                                                            var3 = 4i32;
                                                        } else {
                                                            let var770 = var0;
                                                            let var771 = self.func54(imports, var770, 156i32); // wasm/cpu/opcodes/isOpcode
                                                            if var771 != 0 {
                                                                let var772 = self.global7;
                                                                self.func76(imports, var772); // wasm/cpu/instructions/subAThroughCarryRegister
                                                                var3 = 4i32;
                                                            } else {
                                                                let var773 = var0;
                                                                let var774 = self.func54(imports, var773, 157i32); // wasm/cpu/opcodes/isOpcode
                                                                if var774 != 0 {
                                                                    let var775 = self.global8;
                                                                    self.func76(imports, var775); // wasm/cpu/instructions/subAThroughCarryRegister
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var776 = var0;
                                                                    let var777 = self.func54(imports, var776, 158i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var777 != 0 {
                                                                        let var778 = self.global7;
                                                                        let var779 = self.global8;
                                                                        let var780 = self.func53(imports, var778, var779); // wasm/helpers/index/concatenateBytes
                                                                        let var781 = self.func18(imports, var780); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.func76(imports, var781); // wasm/cpu/instructions/subAThroughCarryRegister
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var782 = var0;
                                                                        let var783 = self.func54(imports, var782, 159i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var783 != 0 {
                                                                            let var784 = self.global1;
                                                                            self.func76(imports, var784); // wasm/cpu/instructions/subAThroughCarryRegister
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var785 = var6;
        if (var785 == 10i32) as i32 != 0 {
            let var786 = var0;
            let var787 = self.func54(imports, var786, 160i32); // wasm/cpu/opcodes/isOpcode
            if var787 != 0 {
                let var788 = self.global3;
                self.func77(imports, var788); // wasm/cpu/instructions/andARegister
                var3 = 4i32;
            } else {
                let var789 = var0;
                let var790 = self.func54(imports, var789, 161i32); // wasm/cpu/opcodes/isOpcode
                if var790 != 0 {
                    let var791 = self.global4;
                    self.func77(imports, var791); // wasm/cpu/instructions/andARegister
                    var3 = 4i32;
                } else {
                    let var792 = var0;
                    let var793 = self.func54(imports, var792, 162i32); // wasm/cpu/opcodes/isOpcode
                    if var793 != 0 {
                        let var794 = self.global5;
                        self.func77(imports, var794); // wasm/cpu/instructions/andARegister
                        var3 = 4i32;
                    } else {
                        let var795 = var0;
                        let var796 = self.func54(imports, var795, 163i32); // wasm/cpu/opcodes/isOpcode
                        if var796 != 0 {
                            let var797 = self.global6;
                            self.func77(imports, var797); // wasm/cpu/instructions/andARegister
                            var3 = 4i32;
                        } else {
                            let var798 = var0;
                            let var799 = self.func54(imports, var798, 164i32); // wasm/cpu/opcodes/isOpcode
                            if var799 != 0 {
                                let var800 = self.global7;
                                self.func77(imports, var800); // wasm/cpu/instructions/andARegister
                                var3 = 4i32;
                            } else {
                                let var801 = var0;
                                let var802 = self.func54(imports, var801, 165i32); // wasm/cpu/opcodes/isOpcode
                                if var802 != 0 {
                                    let var803 = self.global8;
                                    self.func77(imports, var803); // wasm/cpu/instructions/andARegister
                                    var3 = 4i32;
                                } else {
                                    let var804 = var0;
                                    let var805 = self.func54(imports, var804, 166i32); // wasm/cpu/opcodes/isOpcode
                                    if var805 != 0 {
                                        let var806 = self.global7;
                                        let var807 = self.global8;
                                        let var808 = self.func53(imports, var806, var807); // wasm/helpers/index/concatenateBytes
                                        let var809 = self.func18(imports, var808); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.func77(imports, var809); // wasm/cpu/instructions/andARegister
                                        var3 = 8i32;
                                    } else {
                                        let var810 = var0;
                                        let var811 = self.func54(imports, var810, 167i32); // wasm/cpu/opcodes/isOpcode
                                        if var811 != 0 {
                                            let var812 = self.global1;
                                            self.func77(imports, var812); // wasm/cpu/instructions/andARegister
                                            var3 = 4i32;
                                        } else {
                                            let var813 = var0;
                                            let var814 = self.func54(imports, var813, 168i32); // wasm/cpu/opcodes/isOpcode
                                            if var814 != 0 {
                                                let var815 = self.global3;
                                                self.func78(imports, var815); // wasm/cpu/instructions/xorARegister
                                                var3 = 4i32;
                                            } else {
                                                let var816 = var0;
                                                let var817 = self.func54(imports, var816, 169i32); // wasm/cpu/opcodes/isOpcode
                                                if var817 != 0 {
                                                    let var818 = self.global4;
                                                    self.func78(imports, var818); // wasm/cpu/instructions/xorARegister
                                                    var3 = 4i32;
                                                } else {
                                                    let var819 = var0;
                                                    let var820 = self.func54(imports, var819, 170i32); // wasm/cpu/opcodes/isOpcode
                                                    if var820 != 0 {
                                                        let var821 = self.global5;
                                                        self.func78(imports, var821); // wasm/cpu/instructions/xorARegister
                                                        var3 = 4i32;
                                                    } else {
                                                        let var822 = var0;
                                                        let var823 = self.func54(imports, var822, 171i32); // wasm/cpu/opcodes/isOpcode
                                                        if var823 != 0 {
                                                            let var824 = self.global6;
                                                            self.func78(imports, var824); // wasm/cpu/instructions/xorARegister
                                                            var3 = 4i32;
                                                        } else {
                                                            let var825 = var0;
                                                            let var826 = self.func54(imports, var825, 172i32); // wasm/cpu/opcodes/isOpcode
                                                            if var826 != 0 {
                                                                let var827 = self.global7;
                                                                self.func78(imports, var827); // wasm/cpu/instructions/xorARegister
                                                                var3 = 4i32;
                                                            } else {
                                                                let var828 = var0;
                                                                let var829 = self.func54(imports, var828, 173i32); // wasm/cpu/opcodes/isOpcode
                                                                if var829 != 0 {
                                                                    let var830 = self.global8;
                                                                    self.func78(imports, var830); // wasm/cpu/instructions/xorARegister
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var831 = var0;
                                                                    let var832 = self.func54(imports, var831, 174i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var832 != 0 {
                                                                        let var833 = self.global7;
                                                                        let var834 = self.global8;
                                                                        let var835 = self.func53(imports, var833, var834); // wasm/helpers/index/concatenateBytes
                                                                        let var836 = self.func18(imports, var835); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.func78(imports, var836); // wasm/cpu/instructions/xorARegister
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var837 = var0;
                                                                        let var838 = self.func54(imports, var837, 175i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var838 != 0 {
                                                                            let var839 = self.global1;
                                                                            self.func78(imports, var839); // wasm/cpu/instructions/xorARegister
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var840 = var6;
        if (var840 == 11i32) as i32 != 0 {
            let var841 = var0;
            let var842 = self.func54(imports, var841, 176i32); // wasm/cpu/opcodes/isOpcode
            if var842 != 0 {
                let var843 = self.global3;
                self.func79(imports, var843); // wasm/cpu/instructions/orARegister
                var3 = 4i32;
            } else {
                let var844 = var0;
                let var845 = self.func54(imports, var844, 177i32); // wasm/cpu/opcodes/isOpcode
                if var845 != 0 {
                    let var846 = self.global4;
                    self.func79(imports, var846); // wasm/cpu/instructions/orARegister
                    var3 = 4i32;
                } else {
                    let var847 = var0;
                    let var848 = self.func54(imports, var847, 178i32); // wasm/cpu/opcodes/isOpcode
                    if var848 != 0 {
                        let var849 = self.global5;
                        self.func79(imports, var849); // wasm/cpu/instructions/orARegister
                        var3 = 4i32;
                    } else {
                        let var850 = var0;
                        let var851 = self.func54(imports, var850, 179i32); // wasm/cpu/opcodes/isOpcode
                        if var851 != 0 {
                            let var852 = self.global6;
                            self.func79(imports, var852); // wasm/cpu/instructions/orARegister
                            var3 = 4i32;
                        } else {
                            let var853 = var0;
                            let var854 = self.func54(imports, var853, 180i32); // wasm/cpu/opcodes/isOpcode
                            if var854 != 0 {
                                let var855 = self.global7;
                                self.func79(imports, var855); // wasm/cpu/instructions/orARegister
                                var3 = 4i32;
                            } else {
                                let var856 = var0;
                                let var857 = self.func54(imports, var856, 181i32); // wasm/cpu/opcodes/isOpcode
                                if var857 != 0 {
                                    let var858 = self.global8;
                                    self.func79(imports, var858); // wasm/cpu/instructions/orARegister
                                    var3 = 4i32;
                                } else {
                                    let var859 = var0;
                                    let var860 = self.func54(imports, var859, 182i32); // wasm/cpu/opcodes/isOpcode
                                    if var860 != 0 {
                                        let var861 = self.global7;
                                        let var862 = self.global8;
                                        let var863 = self.func53(imports, var861, var862); // wasm/helpers/index/concatenateBytes
                                        let var864 = self.func18(imports, var863); // wasm/memory/load/eightBitLoadFromGBMemory
                                        self.func79(imports, var864); // wasm/cpu/instructions/orARegister
                                        var3 = 8i32;
                                    } else {
                                        let var865 = var0;
                                        let var866 = self.func54(imports, var865, 183i32); // wasm/cpu/opcodes/isOpcode
                                        if var866 != 0 {
                                            let var867 = self.global1;
                                            self.func79(imports, var867); // wasm/cpu/instructions/orARegister
                                            var3 = 4i32;
                                        } else {
                                            let var868 = var0;
                                            let var869 = self.func54(imports, var868, 184i32); // wasm/cpu/opcodes/isOpcode
                                            if var869 != 0 {
                                                let var870 = self.global3;
                                                self.func80(imports, var870); // wasm/cpu/instructions/cpARegister
                                                var3 = 4i32;
                                            } else {
                                                let var871 = var0;
                                                let var872 = self.func54(imports, var871, 185i32); // wasm/cpu/opcodes/isOpcode
                                                if var872 != 0 {
                                                    let var873 = self.global4;
                                                    self.func80(imports, var873); // wasm/cpu/instructions/cpARegister
                                                    var3 = 4i32;
                                                } else {
                                                    let var874 = var0;
                                                    let var875 = self.func54(imports, var874, 186i32); // wasm/cpu/opcodes/isOpcode
                                                    if var875 != 0 {
                                                        let var876 = self.global5;
                                                        self.func80(imports, var876); // wasm/cpu/instructions/cpARegister
                                                        var3 = 4i32;
                                                    } else {
                                                        let var877 = var0;
                                                        let var878 = self.func54(imports, var877, 187i32); // wasm/cpu/opcodes/isOpcode
                                                        if var878 != 0 {
                                                            let var879 = self.global6;
                                                            self.func80(imports, var879); // wasm/cpu/instructions/cpARegister
                                                            var3 = 4i32;
                                                        } else {
                                                            let var880 = var0;
                                                            let var881 = self.func54(imports, var880, 188i32); // wasm/cpu/opcodes/isOpcode
                                                            if var881 != 0 {
                                                                let var882 = self.global7;
                                                                self.func80(imports, var882); // wasm/cpu/instructions/cpARegister
                                                                var3 = 4i32;
                                                            } else {
                                                                let var883 = var0;
                                                                let var884 = self.func54(imports, var883, 189i32); // wasm/cpu/opcodes/isOpcode
                                                                if var884 != 0 {
                                                                    let var885 = self.global8;
                                                                    self.func80(imports, var885); // wasm/cpu/instructions/cpARegister
                                                                    var3 = 4i32;
                                                                } else {
                                                                    let var886 = var0;
                                                                    let var887 = self.func54(imports, var886, 190i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var887 != 0 {
                                                                        let var888 = self.global7;
                                                                        let var889 = self.global8;
                                                                        let var890 = self.func53(imports, var888, var889); // wasm/helpers/index/concatenateBytes
                                                                        let var891 = self.func18(imports, var890); // wasm/memory/load/eightBitLoadFromGBMemory
                                                                        self.func80(imports, var891); // wasm/cpu/instructions/cpARegister
                                                                        var3 = 8i32;
                                                                    } else {
                                                                        let var892 = var0;
                                                                        let var893 = self.func54(imports, var892, 191i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var893 != 0 {
                                                                            let var894 = self.global1;
                                                                            self.func80(imports, var894); // wasm/cpu/instructions/cpARegister
                                                                            var3 = 4i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var895 = var6;
        if (var895 == 12i32) as i32 != 0 {
            let var896 = var0;
            let var897 = self.func54(imports, var896, 192i32); // wasm/cpu/opcodes/isOpcode
            if var897 != 0 {
                let var898 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                let var899: i32;
                if var898 != 0 {
                    var899 = 8i32;
                } else {
                    let var900 = self.global9;
                    let var901 = self.func81(imports, var900); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    self.global0 = var901;
                    let var902 = self.global9;
                    self.global9 = var902.wrapping_add(2i32) & 65535i32;
                    var899 = 20i32;
                }
                var3 = var899;
            } else {
                let var903 = var0;
                let var904 = self.func54(imports, var903, 193i32); // wasm/cpu/opcodes/isOpcode
                if var904 != 0 {
                    let var905 = self.global3;
                    let var906 = self.global4;
                    let var907 = self.func53(imports, var905, var906); // wasm/helpers/index/concatenateBytes
                    let var908 = self.global9;
                    let var909 = self.func81(imports, var908); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    var2 = var909;
                    let var910 = self.global9;
                    self.global9 = var910.wrapping_add(2i32) & 65535i32;
                    let var911 = var2;
                    let var912 = self.func9(imports, var911); // wasm/helpers/index/splitHighByte
                    self.global3 = var912;
                    let var913 = var2;
                    let var914 = self.func10(imports, var913); // wasm/helpers/index/splitLowByte
                    self.global4 = var914;
                    var3 = 12i32;
                } else {
                    let var915 = var0;
                    let var916 = self.func54(imports, var915, 194i32); // wasm/cpu/opcodes/isOpcode
                    if var916 != 0 {
                        let var917 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                        let var918: i32;
                        if var917 != 0 {
                            let var919 = self.global0;
                            self.global0 = var919.wrapping_add(2i32) & 65535i32;
                            var918 = 12i32;
                        } else {
                            let var920 = var5;
                            self.global0 = var920;
                            var918 = 16i32;
                        }
                        var3 = var918;
                    } else {
                        let var921 = var0;
                        let var922 = self.func54(imports, var921, 195i32); // wasm/cpu/opcodes/isOpcode
                        if var922 != 0 {
                            let var923 = var5;
                            self.global0 = var923;
                            var3 = 16i32;
                        } else {
                            let var924 = var0;
                            let var925 = self.func54(imports, var924, 196i32); // wasm/cpu/opcodes/isOpcode
                            if var925 != 0 {
                                let var926 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                                let var927: i32;
                                if var926 != 0 {
                                    let var928 = self.global0;
                                    self.global0 = var928.wrapping_add(2i32) & 65535i32;
                                    var927 = 12i32;
                                } else {
                                    let var929 = self.global9;
                                    self.global9 = var929.wrapping_sub(2i32) & 65535i32;
                                    let var930 = self.global9;
                                    let var931 = self.global0;
                                    self.func62(imports, var930, var931.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                    let var932 = self.global0;
                                    let var933 = self.func81(imports, var932); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                    self.global0 = var933;
                                    var927 = 24i32;
                                }
                                var3 = var927;
                            } else {
                                let var934 = var0;
                                let var935 = self.func54(imports, var934, 197i32); // wasm/cpu/opcodes/isOpcode
                                if var935 != 0 {
                                    let var936 = self.global3;
                                    let var937 = self.global4;
                                    let var938 = self.func53(imports, var936, var937); // wasm/helpers/index/concatenateBytes
                                    var2 = var938;
                                    let var939 = self.global9;
                                    self.global9 = var939.wrapping_sub(2i32) & 65535i32;
                                    let var940 = self.global9;
                                    let var941 = var2;
                                    self.func62(imports, var940, var941); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                    var3 = 16i32;
                                } else {
                                    let var942 = var0;
                                    let var943 = self.func54(imports, var942, 198i32); // wasm/cpu/opcodes/isOpcode
                                    if var943 != 0 {
                                        let var944 = var1;
                                        self.func73(imports, var944); // wasm/cpu/instructions/addARegister
                                        let var945 = self.global0;
                                        self.global0 = var945.wrapping_add(1i32) & 65535i32;
                                        var3 = 4i32;
                                    } else {
                                        let var946 = var0;
                                        let var947 = self.func54(imports, var946, 199i32); // wasm/cpu/opcodes/isOpcode
                                        if var947 != 0 {
                                            let var948 = self.global9;
                                            self.global9 = var948.wrapping_sub(2i32) & 65535i32;
                                            let var949 = self.global9;
                                            let var950 = self.global0;
                                            self.func62(imports, var949, var950); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                            self.global0 = 0i32;
                                            var3 = 16i32;
                                        } else {
                                            let var951 = var0;
                                            let var952 = self.func54(imports, var951, 200i32); // wasm/cpu/opcodes/isOpcode
                                            if var952 != 0 {
                                                let var953 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                                                let var954: i32;
                                                if (var953 == 1i32) as i32 != 0 {
                                                    let var955 = self.global9;
                                                    let var956 = self.func81(imports, var955); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                    self.global0 = var956;
                                                    let var957 = self.global9;
                                                    self.global9 = var957.wrapping_add(2i32) & 65535i32;
                                                    var954 = 20i32;
                                                } else {
                                                    var954 = 8i32;
                                                }
                                                var3 = var954;
                                            } else {
                                                let var958 = var0;
                                                let var959 = self.func54(imports, var958, 201i32); // wasm/cpu/opcodes/isOpcode
                                                if var959 != 0 {
                                                    let var960 = self.global9;
                                                    let var961 = self.func81(imports, var960); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                    self.global0 = var961;
                                                    let var962 = self.global9;
                                                    self.global9 = var962.wrapping_add(2i32) & 65535i32;
                                                    var3 = 16i32;
                                                } else {
                                                    let var963 = var0;
                                                    let var964 = self.func54(imports, var963, 202i32); // wasm/cpu/opcodes/isOpcode
                                                    if var964 != 0 {
                                                        let var965 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                                                        let var966: i32;
                                                        if (var965 == 1i32) as i32 != 0 {
                                                            let var967 = var5;
                                                            self.global0 = var967;
                                                            var966 = 16i32;
                                                        } else {
                                                            let var968 = self.global0;
                                                            self.global0 = var968.wrapping_add(2i32) & 65535i32;
                                                            var966 = 12i32;
                                                        }
                                                        var3 = var966;
                                                    } else {
                                                        let var969 = var0;
                                                        let var970 = self.func54(imports, var969, 203i32); // wasm/cpu/opcodes/isOpcode
                                                        if var970 != 0 {
                                                            let var971 = var1;
                                                            let var972 = self.func92(imports, var971); // wasm/cpu/cbOpcodes/handleCbOpcode
                                                            var3 = var972;
                                                            let var973 = var3;
                                                            if (var973 > 0i32) as i32 != 0 {
                                                                let var974 = var3;
                                                                var3 = var974.wrapping_add(4i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                                            }
                                                        } else {
                                                            let var975 = var0;
                                                            let var976 = self.func54(imports, var975, 204i32); // wasm/cpu/opcodes/isOpcode
                                                            if var976 != 0 {
                                                                let var977 = self.func69(imports); // wasm/cpu/flags/getZeroFlag
                                                                let var978: i32;
                                                                if (var977 == 1i32) as i32 != 0 {
                                                                    let var979 = self.global9;
                                                                    self.global9 = var979.wrapping_sub(2i32) & 65535i32;
                                                                    let var980 = self.global9;
                                                                    let var981 = self.global0;
                                                                    self.func62(imports, var980, var981.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                                    let var982 = self.global0;
                                                                    let var983 = self.func81(imports, var982); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                                    self.global0 = var983;
                                                                    var978 = 24i32;
                                                                } else {
                                                                    let var984 = self.global0;
                                                                    self.global0 = var984.wrapping_add(2i32) & 65535i32;
                                                                    var978 = 12i32;
                                                                }
                                                                var3 = var978;
                                                            } else {
                                                                let var985 = var0;
                                                                let var986 = self.func54(imports, var985, 205i32); // wasm/cpu/opcodes/isOpcode
                                                                if var986 != 0 {
                                                                    let var987 = self.global9;
                                                                    self.global9 = var987.wrapping_sub(2i32) & 65535i32;
                                                                    let var988 = self.global9;
                                                                    let var989 = self.global0;
                                                                    self.func62(imports, var988, var989.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                                    let var990 = self.global0;
                                                                    let var991 = self.func81(imports, var990); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                                    self.global0 = var991;
                                                                    var3 = 24i32;
                                                                } else {
                                                                    let var992 = var0;
                                                                    let var993 = self.func54(imports, var992, 206i32); // wasm/cpu/opcodes/isOpcode
                                                                    if var993 != 0 {
                                                                        let var994 = var1;
                                                                        self.func74(imports, var994); // wasm/cpu/instructions/addAThroughCarryRegister
                                                                        let var995 = self.global0;
                                                                        self.global0 = var995.wrapping_add(1i32) & 65535i32;
                                                                        var3 = 4i32;
                                                                    } else {
                                                                        let var996 = var0;
                                                                        let var997 = self.func54(imports, var996, 207i32); // wasm/cpu/opcodes/isOpcode
                                                                        if var997 != 0 {
                                                                            let var998 = self.global9;
                                                                            self.global9 = var998.wrapping_sub(2i32) & 65535i32;
                                                                            let var999 = self.global9;
                                                                            let var1000 = self.global0;
                                                                            self.func62(imports, var999, var1000); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                                            self.global0 = 8i32;
                                                                            var3 = 16i32;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var1001 = var6;
        if (var1001 == 13i32) as i32 != 0 {
            let var1002 = var0;
            let var1003 = self.func54(imports, var1002, 208i32); // wasm/cpu/opcodes/isOpcode
            if var1003 != 0 {
                let var1004 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                let var1005: i32;
                if var1004 != 0 {
                    var1005 = 8i32;
                } else {
                    let var1006 = self.global9;
                    let var1007 = self.func81(imports, var1006); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    self.global0 = var1007;
                    let var1008 = self.global9;
                    self.global9 = var1008.wrapping_add(2i32) & 65535i32;
                    var1005 = 20i32;
                }
                var3 = var1005;
            } else {
                let var1009 = var0;
                let var1010 = self.func54(imports, var1009, 209i32); // wasm/cpu/opcodes/isOpcode
                if var1010 != 0 {
                    let var1011 = self.global5;
                    let var1012 = self.global6;
                    let var1013 = self.func53(imports, var1011, var1012); // wasm/helpers/index/concatenateBytes
                    let var1014 = self.global9;
                    let var1015 = self.func81(imports, var1014); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    var2 = var1015;
                    let var1016 = self.global9;
                    self.global9 = var1016.wrapping_add(2i32) & 65535i32;
                    let var1017 = var2;
                    let var1018 = self.func9(imports, var1017); // wasm/helpers/index/splitHighByte
                    self.global5 = var1018;
                    let var1019 = var2;
                    let var1020 = self.func10(imports, var1019); // wasm/helpers/index/splitLowByte
                    self.global6 = var1020;
                    var3 = 12i32;
                } else {
                    let var1021 = var0;
                    let var1022 = self.func54(imports, var1021, 210i32); // wasm/cpu/opcodes/isOpcode
                    if var1022 != 0 {
                        let var1023 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                        let var1024: i32;
                        if var1023 != 0 {
                            let var1025 = self.global0;
                            self.global0 = var1025.wrapping_add(2i32) & 65535i32;
                            var1024 = 12i32;
                        } else {
                            let var1026 = var5;
                            self.global0 = var1026;
                            var1024 = 16i32;
                        }
                        var3 = var1024;
                    } else {
                        let var1027 = var0;
                        let var1028 = self.func54(imports, var1027, 212i32); // wasm/cpu/opcodes/isOpcode
                        if var1028 != 0 {
                            let var1029 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                            let var1030: i32;
                            if var1029 != 0 {
                                let var1031 = self.global0;
                                self.global0 = var1031.wrapping_add(2i32) & 65535i32;
                                var1030 = 12i32;
                            } else {
                                let var1032 = self.global9;
                                self.global9 = var1032.wrapping_sub(2i32) & 65535i32;
                                let var1033 = self.global9;
                                let var1034 = self.global0;
                                self.func62(imports, var1033, var1034.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                let var1035 = self.global0;
                                let var1036 = self.func81(imports, var1035); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                self.global0 = var1036;
                                var1030 = 24i32;
                            }
                            var3 = var1030;
                        } else {
                            let var1037 = var0;
                            let var1038 = self.func54(imports, var1037, 213i32); // wasm/cpu/opcodes/isOpcode
                            if var1038 != 0 {
                                let var1039 = self.global5;
                                let var1040 = self.global6;
                                let var1041 = self.func53(imports, var1039, var1040); // wasm/helpers/index/concatenateBytes
                                var2 = var1041;
                                let var1042 = self.global9;
                                self.global9 = var1042.wrapping_sub(2i32) & 65535i32;
                                let var1043 = self.global9;
                                let var1044 = var2;
                                self.func62(imports, var1043, var1044); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                var3 = 16i32;
                            } else {
                                let var1045 = var0;
                                let var1046 = self.func54(imports, var1045, 214i32); // wasm/cpu/opcodes/isOpcode
                                if var1046 != 0 {
                                    let var1047 = var1;
                                    self.func75(imports, var1047); // wasm/cpu/instructions/subARegister
                                    let var1048 = self.global0;
                                    self.global0 = var1048.wrapping_add(1i32) & 65535i32;
                                    var3 = 8i32;
                                } else {
                                    let var1049 = var0;
                                    let var1050 = self.func54(imports, var1049, 215i32); // wasm/cpu/opcodes/isOpcode
                                    if var1050 != 0 {
                                        let var1051 = self.global9;
                                        self.global9 = var1051.wrapping_sub(2i32) & 65535i32;
                                        let var1052 = self.global9;
                                        let var1053 = self.global0;
                                        self.func62(imports, var1052, var1053); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                        self.global0 = 16i32;
                                        var3 = 16i32;
                                    } else {
                                        let var1054 = var0;
                                        let var1055 = self.func54(imports, var1054, 216i32); // wasm/cpu/opcodes/isOpcode
                                        if var1055 != 0 {
                                            let var1056 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                            let var1057: i32;
                                            if (var1056 == 1i32) as i32 != 0 {
                                                let var1058 = self.global9;
                                                let var1059 = self.func81(imports, var1058); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                self.global0 = var1059;
                                                let var1060 = self.global9;
                                                self.global9 = var1060.wrapping_add(2i32) & 65535i32;
                                                var1057 = 20i32;
                                            } else {
                                                var1057 = 8i32;
                                            }
                                            var3 = var1057;
                                        } else {
                                            let var1061 = var0;
                                            let var1062 = self.func54(imports, var1061, 217i32); // wasm/cpu/opcodes/isOpcode
                                            if var1062 != 0 {
                                                let var1063 = self.global9;
                                                let var1064 = self.func81(imports, var1063); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                self.global0 = var1064;
                                                self.func93(imports, 1i32); // wasm/interrupts/index/setInterrupts
                                                let var1065 = self.global9;
                                                self.global9 = var1065.wrapping_add(2i32) & 65535i32;
                                                var3 = 16i32;
                                            } else {
                                                let var1066 = var0;
                                                let var1067 = self.func54(imports, var1066, 218i32); // wasm/cpu/opcodes/isOpcode
                                                if var1067 != 0 {
                                                    let var1068 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                                    let var1069: i32;
                                                    if (var1068 == 1i32) as i32 != 0 {
                                                        let var1070 = var5;
                                                        self.global0 = var1070;
                                                        var1069 = 16i32;
                                                    } else {
                                                        let var1071 = self.global0;
                                                        self.global0 = var1071.wrapping_add(2i32) & 65535i32;
                                                        var1069 = 12i32;
                                                    }
                                                    var3 = var1069;
                                                } else {
                                                    let var1072 = var0;
                                                    let var1073 = self.func54(imports, var1072, 220i32); // wasm/cpu/opcodes/isOpcode
                                                    if var1073 != 0 {
                                                        let var1074 = self.func65(imports); // wasm/cpu/flags/getCarryFlag
                                                        let var1075: i32;
                                                        if (var1074 == 1i32) as i32 != 0 {
                                                            let var1076 = self.global9;
                                                            self.global9 = var1076.wrapping_sub(2i32) & 65535i32;
                                                            let var1077 = self.global9;
                                                            let var1078 = self.global0;
                                                            self.func62(imports, var1077, var1078.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                            let var1079 = self.global0;
                                                            let var1080 = self.func81(imports, var1079); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                            self.global0 = var1080;
                                                            var1075 = 24i32;
                                                        } else {
                                                            let var1081 = self.global0;
                                                            self.global0 = var1081.wrapping_add(2i32) & 65535i32;
                                                            var1075 = 12i32;
                                                        }
                                                        var3 = var1075;
                                                    } else {
                                                        let var1082 = var0;
                                                        let var1083 = self.func54(imports, var1082, 222i32); // wasm/cpu/opcodes/isOpcode
                                                        if var1083 != 0 {
                                                            let var1084 = var1;
                                                            self.func76(imports, var1084); // wasm/cpu/instructions/subAThroughCarryRegister
                                                            let var1085 = self.global0;
                                                            self.global0 = var1085.wrapping_add(1i32) & 65535i32;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var1086 = var0;
                                                            let var1087 = self.func54(imports, var1086, 223i32); // wasm/cpu/opcodes/isOpcode
                                                            if var1087 != 0 {
                                                                let var1088 = self.global9;
                                                                self.global9 = var1088.wrapping_sub(2i32) & 65535i32;
                                                                let var1089 = self.global9;
                                                                let var1090 = self.global0;
                                                                self.func62(imports, var1089, var1090); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                                self.global0 = 24i32;
                                                                var3 = 16i32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var1091 = var6;
        if (var1091 == 14i32) as i32 != 0 {
            let var1092 = var0;
            let var1093 = self.func54(imports, var1092, 224i32); // wasm/cpu/opcodes/isOpcode
            if var1093 != 0 {
                let var1094 = var1;
                let var1095 = self.global1;
                self.func45(imports, var1094.wrapping_add(65280i32) & 65535i32, var1095); // wasm/memory/store/eightBitStoreIntoGBMemory
                let var1096 = self.global0;
                self.global0 = var1096.wrapping_add(1i32) & 65535i32;
                var3 = 12i32;
            } else {
                let var1097 = var0;
                let var1098 = self.func54(imports, var1097, 225i32); // wasm/cpu/opcodes/isOpcode
                if var1098 != 0 {
                    let var1099 = self.global7;
                    let var1100 = self.global8;
                    let var1101 = self.func53(imports, var1099, var1100); // wasm/helpers/index/concatenateBytes
                    let var1102 = self.global9;
                    let var1103 = self.func81(imports, var1102); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    var2 = var1103;
                    let var1104 = self.global9;
                    self.global9 = var1104.wrapping_add(2i32) & 65535i32;
                    let var1105 = var2;
                    let var1106 = self.func9(imports, var1105); // wasm/helpers/index/splitHighByte
                    self.global7 = var1106;
                    let var1107 = var2;
                    let var1108 = self.func10(imports, var1107); // wasm/helpers/index/splitLowByte
                    self.global8 = var1108;
                    var3 = 12i32;
                } else {
                    let var1109 = var0;
                    let var1110 = self.func54(imports, var1109, 226i32); // wasm/cpu/opcodes/isOpcode
                    if var1110 != 0 {
                        let var1111 = self.global4;
                        let var1112 = self.global1;
                        self.func45(imports, var1111.wrapping_add(65280i32) & 65535i32, var1112); // wasm/memory/store/eightBitStoreIntoGBMemory
                        var3 = 8i32;
                    } else {
                        let var1113 = var0;
                        let var1114 = self.func54(imports, var1113, 229i32); // wasm/cpu/opcodes/isOpcode
                        if var1114 != 0 {
                            let var1115 = self.global7;
                            let var1116 = self.global8;
                            let var1117 = self.func53(imports, var1115, var1116); // wasm/helpers/index/concatenateBytes
                            var2 = var1117;
                            let var1118 = self.global9;
                            self.global9 = var1118.wrapping_sub(2i32) & 65535i32;
                            let var1119 = self.global9;
                            let var1120 = var2;
                            self.func62(imports, var1119, var1120); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                            var3 = 16i32;
                        } else {
                            let var1121 = var0;
                            let var1122 = self.func54(imports, var1121, 230i32); // wasm/cpu/opcodes/isOpcode
                            if var1122 != 0 {
                                let var1123 = var1;
                                self.func77(imports, var1123); // wasm/cpu/instructions/andARegister
                                let var1124 = self.global0;
                                self.global0 = var1124.wrapping_add(1i32) & 65535i32;
                                var3 = 8i32;
                            } else {
                                let var1125 = var0;
                                let var1126 = self.func54(imports, var1125, 231i32); // wasm/cpu/opcodes/isOpcode
                                if var1126 != 0 {
                                    let var1127 = self.global9;
                                    self.global9 = var1127.wrapping_sub(2i32) & 65535i32;
                                    let var1128 = self.global9;
                                    let var1129 = self.global0;
                                    self.func62(imports, var1128, var1129); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                    self.global0 = 32i32;
                                    var3 = 16i32;
                                } else {
                                    let var1130 = var0;
                                    let var1131 = self.func54(imports, var1130, 232i32); // wasm/cpu/opcodes/isOpcode
                                    if var1131 != 0 {
                                        let var1132 = self.global9;
                                        let var1133 = var1;
                                        var2 = var1133.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                        let var1134 = var2;
                                        self.func63(imports, var1132, var1134, 1i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                        let var1135 = self.global9;
                                        let var1136 = var2;
                                        self.global9 = var1135.wrapping_add(var1136) & 65535i32;
                                        self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                        self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                        let var1137 = self.global0;
                                        self.global0 = var1137.wrapping_add(1i32) & 65535i32;
                                        var3 = 16i32;
                                    } else {
                                        let var1138 = var0;
                                        let var1139 = self.func54(imports, var1138, 233i32); // wasm/cpu/opcodes/isOpcode
                                        if var1139 != 0 {
                                            let var1140 = self.global7;
                                            let var1141 = self.global8;
                                            let var1142 = self.func53(imports, var1140, var1141); // wasm/helpers/index/concatenateBytes
                                            self.global0 = var1142;
                                            var3 = 4i32;
                                        } else {
                                            let var1143 = var0;
                                            let var1144 = self.func54(imports, var1143, 234i32); // wasm/cpu/opcodes/isOpcode
                                            if var1144 != 0 {
                                                let var1145 = var5;
                                                let var1146 = self.global1;
                                                self.func45(imports, var1145, var1146); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                let var1147 = self.global0;
                                                self.global0 = var1147.wrapping_add(2i32) & 65535i32;
                                                var3 = 16i32;
                                            } else {
                                                let var1148 = var0;
                                                let var1149 = self.func54(imports, var1148, 238i32); // wasm/cpu/opcodes/isOpcode
                                                if var1149 != 0 {
                                                    let var1150 = var1;
                                                    self.func78(imports, var1150); // wasm/cpu/instructions/xorARegister
                                                    let var1151 = self.global0;
                                                    self.global0 = var1151.wrapping_add(1i32) & 65535i32;
                                                    var3 = 8i32;
                                                } else {
                                                    let var1152 = var0;
                                                    let var1153 = self.func54(imports, var1152, 239i32); // wasm/cpu/opcodes/isOpcode
                                                    if var1153 != 0 {
                                                        let var1154 = self.global9;
                                                        self.global9 = var1154.wrapping_sub(2i32) & 65535i32;
                                                        let var1155 = self.global9;
                                                        let var1156 = self.global0;
                                                        self.func62(imports, var1155, var1156); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                        self.global0 = 40i32;
                                                        var3 = 16i32;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var1157 = var6;
        if (var1157 == 15i32) as i32 != 0 {
            let var1158 = var0;
            let var1159 = self.func54(imports, var1158, 240i32); // wasm/cpu/opcodes/isOpcode
            if var1159 != 0 {
                let var1160 = var1;
                let var1161 = self.func18(imports, var1160.wrapping_add(65280i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                self.global1 = var1161;
                let var1162 = self.global0;
                self.global0 = var1162.wrapping_add(1i32) & 65535i32;
                var3 = 12i32;
            } else {
                let var1163 = var0;
                let var1164 = self.func54(imports, var1163, 241i32); // wasm/cpu/opcodes/isOpcode
                if var1164 != 0 {
                    let var1165 = self.global1;
                    let var1166 = self.global2;
                    let var1167 = self.func53(imports, var1165, var1166); // wasm/helpers/index/concatenateBytes
                    let var1168 = self.global9;
                    let var1169 = self.func81(imports, var1168); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    var2 = var1169;
                    let var1170 = self.global9;
                    self.global9 = var1170.wrapping_add(2i32) & 65535i32;
                    let var1171 = var2;
                    let var1172 = self.func9(imports, var1171); // wasm/helpers/index/splitHighByte
                    self.global1 = var1172;
                    let var1173 = var2;
                    let var1174 = self.func10(imports, var1173); // wasm/helpers/index/splitLowByte
                    self.global2 = var1174;
                    var3 = 12i32;
                } else {
                    let var1175 = var0;
                    let var1176 = self.func54(imports, var1175, 242i32); // wasm/cpu/opcodes/isOpcode
                    if var1176 != 0 {
                        let var1177 = self.global4;
                        let var1178 = self.func18(imports, var1177.wrapping_add(65280i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                        self.global1 = var1178;
                        let var1179 = self.global0;
                        self.global0 = var1179.wrapping_add(1i32) & 65535i32;
                        var3 = 8i32;
                    } else {
                        let var1180 = var0;
                        let var1181 = self.func54(imports, var1180, 243i32); // wasm/cpu/opcodes/isOpcode
                        if var1181 != 0 {
                            self.func93(imports, 0i32); // wasm/interrupts/index/setInterrupts
                            var3 = 4i32;
                        } else {
                            let var1182 = var0;
                            let var1183 = self.func54(imports, var1182, 245i32); // wasm/cpu/opcodes/isOpcode
                            if var1183 != 0 {
                                let var1184 = self.global1;
                                let var1185 = self.global2;
                                let var1186 = self.func53(imports, var1184, var1185); // wasm/helpers/index/concatenateBytes
                                var2 = var1186;
                                let var1187 = self.global9;
                                self.global9 = var1187.wrapping_sub(2i32) & 65535i32;
                                let var1188 = self.global9;
                                let var1189 = var2;
                                self.func62(imports, var1188, var1189); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                var3 = 16i32;
                            } else {
                                let var1190 = var0;
                                let var1191 = self.func54(imports, var1190, 246i32); // wasm/cpu/opcodes/isOpcode
                                if var1191 != 0 {
                                    let var1192 = var1;
                                    self.func79(imports, var1192); // wasm/cpu/instructions/orARegister
                                    let var1193 = self.global0;
                                    self.global0 = var1193.wrapping_add(1i32) & 65535i32;
                                    var3 = 8i32;
                                } else {
                                    let var1194 = var0;
                                    let var1195 = self.func54(imports, var1194, 247i32); // wasm/cpu/opcodes/isOpcode
                                    if var1195 != 0 {
                                        let var1196 = self.global9;
                                        self.global9 = var1196.wrapping_sub(2i32) & 65535i32;
                                        let var1197 = self.global9;
                                        let var1198 = self.global0;
                                        self.func62(imports, var1197, var1198); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                        self.global0 = 48i32;
                                        var3 = 16i32;
                                    } else {
                                        let var1199 = var0;
                                        let var1200 = self.func54(imports, var1199, 248i32); // wasm/cpu/opcodes/isOpcode
                                        if var1200 != 0 {
                                            self.func58(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            self.func59(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            let var1201 = self.global9;
                                            let var1202 = var1;
                                            var2 = var1202.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                            let var1203 = var2;
                                            self.func63(imports, var1201, var1203, 1i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                            let var1204 = self.global9;
                                            let var1205 = var2;
                                            var3 = var1204.wrapping_add(var1205) & 65535i32;
                                            let var1206 = var3;
                                            let var1207 = self.func9(imports, var1206); // wasm/helpers/index/splitHighByte
                                            self.global7 = var1207;
                                            let var1208 = var3;
                                            let var1209 = self.func10(imports, var1208); // wasm/helpers/index/splitLowByte
                                            self.global8 = var1209;
                                            let var1210 = self.global0;
                                            self.global0 = var1210.wrapping_add(1i32) & 65535i32;
                                            var3 = 12i32;
                                        } else {
                                            let var1211 = var0;
                                            let var1212 = self.func54(imports, var1211, 249i32); // wasm/cpu/opcodes/isOpcode
                                            if var1212 != 0 {
                                                let var1213 = self.global7;
                                                let var1214 = self.global8;
                                                let var1215 = self.func53(imports, var1213, var1214); // wasm/helpers/index/concatenateBytes
                                                self.global9 = var1215;
                                                var3 = 8i32;
                                            } else {
                                                let var1216 = var0;
                                                let var1217 = self.func54(imports, var1216, 250i32); // wasm/cpu/opcodes/isOpcode
                                                if var1217 != 0 {
                                                    let var1218 = var5;
                                                    let var1219 = self.func18(imports, var1218); // wasm/memory/load/eightBitLoadFromGBMemory
                                                    self.global1 = var1219;
                                                    let var1220 = self.global0;
                                                    self.global0 = var1220.wrapping_add(2i32) & 65535i32;
                                                    var3 = 16i32;
                                                } else {
                                                    let var1221 = var0;
                                                    let var1222 = self.func54(imports, var1221, 251i32); // wasm/cpu/opcodes/isOpcode
                                                    if var1222 != 0 {
                                                        self.func93(imports, 1i32); // wasm/interrupts/index/setInterrupts
                                                        var3 = 4i32;
                                                    } else {
                                                        let var1223 = var0;
                                                        let var1224 = self.func54(imports, var1223, 254i32); // wasm/cpu/opcodes/isOpcode
                                                        if var1224 != 0 {
                                                            let var1225 = var1;
                                                            self.func80(imports, var1225); // wasm/cpu/instructions/cpARegister
                                                            let var1226 = self.global0;
                                                            self.global0 = var1226.wrapping_add(1i32) & 65535i32;
                                                            var3 = 8i32;
                                                        } else {
                                                            let var1227 = var0;
                                                            let var1228 = self.func54(imports, var1227, 255i32); // wasm/cpu/opcodes/isOpcode
                                                            if var1228 != 0 {
                                                                let var1229 = self.global9;
                                                                self.global9 = var1229.wrapping_sub(2i32) & 65535i32;
                                                                let var1230 = self.global9;
                                                                let var1231 = self.global0;
                                                                self.func62(imports, var1230, var1231); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                                self.global0 = 56i32;
                                                                var3 = 16i32;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let var1232 = var3;
        var1232
    }
    // wasm/interrupts/index/areInterruptsEnabled
    fn func95<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global55;
        var0
    }
    // wasm/interrupts/index/areInterruptsPending
    fn func96<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var1 = self.func18(imports, 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var2: i32;
        if var0 & var1 & 255i32 != 0 {
            var2 = 1i32;
        } else {
            var2 = 0i32;
        }
        var2
    }
    // wasm/timers/index/_checkDividerRegister
    fn func97<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global57;
        let var2 = var0;
        self.global57 = var1.wrapping_add(var2);
        let var3 = self.global57;
        if (var3 >= 255i32) as i32 != 0 {
            let var4 = self.global57;
            self.global57 = var4.wrapping_sub(255i32);
            let var5 = self.func18(imports, 65284i32); // wasm/memory/load/eightBitLoadFromGBMemory
            self.func8(imports, 65284i32, var5.wrapping_add(1i32) & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        }
    }
    // wasm/timers/index/_isTimerEnabled
    fn func98<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65287i32); // wasm/memory/load/eightBitLoadFromGBMemory
        ((var0 & 4i32) as u32 > 0i32 as u32) as i32
    }
    // wasm/timers/index/_getCurrentCycleCounterFrequency
    fn func99<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.func18(imports, 65287i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var0 = var1 & 3i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = var0;
            let var5: i32;
            if (var4 == 1i32) as i32 != 0 {
                var5 = 16i32;
            } else {
                let var6 = var0;
                let var7: i32;
                if (var6 == 2i32) as i32 != 0 {
                    var7 = 64i32;
                } else {
                    var7 = 256i32;
                }
                var5 = var7;
            }
            var3 = var5;
        } else {
            var3 = 1024i32;
        }
        var0 = var3;
        let var8 = var0;
        let var9 = self.global59;
        if (var8 != var9) as i32 != 0 {
            self.global58 = 0i32;
            let var10 = var0;
            self.global59 = var10;
        }
        let var11 = var0;
        var11
    }
    // wasm/interrupts/index/_requestInterrupt
    fn func100<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func18(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var3 = self.func15(imports, var1, var2); // wasm/helpers/index/setBitOnByte
        self.func8(imports, 65295i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/interrupts/index/requestTimerInterrupt
    fn func101<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func100(imports, 2i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/timers/index/updateTimers
    fn func102<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = var0;
        self.func97(imports, var2); // wasm/timers/index/_checkDividerRegister
        let var3 = self.func98(imports); // wasm/timers/index/_isTimerEnabled
        if var3 != 0 {
            let var4 = self.global58;
            let var5 = var0;
            self.global58 = var4.wrapping_add(var5);
            let var6 = self.global58;
            let var7 = self.func99(imports); // wasm/timers/index/_getCurrentCycleCounterFrequency
            if (var6 > var7) as i32 != 0 {
                let var8 = self.global58;
                let var9 = self.func99(imports); // wasm/timers/index/_getCurrentCycleCounterFrequency
                self.global58 = var8.wrapping_sub(var9);
                let var10 = self.func18(imports, 65285i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var1 = var10;
                let var11 = var1;
                if (var11 == 255i32) as i32 != 0 {
                    let var12 = self.func18(imports, 65286i32); // wasm/memory/load/eightBitLoadFromGBMemory
                    self.func8(imports, 65285i32, var12); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func101(imports); // wasm/interrupts/index/requestTimerInterrupt
                } else {
                    let var13 = var1;
                    self.func8(imports, 65285i32, var13.wrapping_add(1i32) & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                }
            }
        }
    }
    // wasm/graphics/lcd/isLcdEnabled
    fn func103<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65344i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var1 = self.func2(imports, 7i32, var0); // wasm/helpers/index/checkBitOnByte
        var1
    }
    // wasm/interrupts/index/requestLcdInterrupt
    fn func104<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func100(imports, 1i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/graphics/lcd/setLcdStatus
    fn func105<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.func18(imports, 65345i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var1 = var4;
        let var5 = var0;
        if (var5 == 0) as i32 != 0 {
            self.global60 = 0i32;
            self.func8(imports, 65348i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            let var6 = var1;
            let var7 = self.func14(imports, 1i32, var6); // wasm/helpers/index/resetBitOnByte
            let var8 = self.func14(imports, 0i32, var7); // wasm/helpers/index/resetBitOnByte
            var1 = var8;
            self.global19 = 0i32;
            let var9 = var1;
            self.func8(imports, 65345i32, var9); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return;
        }
        let var10 = var1;
        let var11 = self.func18(imports, 65348i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var3 = var11;
        let var12 = var3;
        let var13: i32;
        if (var12 as u32 >= 144i32 as u32) as i32 != 0 {
            let var14 = var1;
            let var15 = self.func14(imports, 1i32, var14); // wasm/helpers/index/resetBitOnByte
            let var16 = self.func15(imports, 0i32, var15); // wasm/helpers/index/setBitOnByte
            var1 = var16;
            let var17 = var1;
            let var18 = self.func2(imports, 4i32, var17); // wasm/helpers/index/checkBitOnByte
            var2 = var18;
            var13 = 1i32;
        } else {
            let var19 = self.global60;
            let var20: i32;
            if (var19 >= 376i32) as i32 != 0 {
                let var21 = var1;
                let var22 = self.func14(imports, 0i32, var21); // wasm/helpers/index/resetBitOnByte
                let var23 = self.func15(imports, 1i32, var22); // wasm/helpers/index/setBitOnByte
                var1 = var23;
                let var24 = var1;
                let var25 = self.func2(imports, 5i32, var24); // wasm/helpers/index/checkBitOnByte
                var2 = var25;
                var20 = 2i32;
            } else {
                let var26 = self.global60;
                let var27: i32;
                if (var26 >= 249i32) as i32 != 0 {
                    let var28 = var1;
                    let var29 = self.func15(imports, 0i32, var28); // wasm/helpers/index/setBitOnByte
                    let var30 = self.func15(imports, 1i32, var29); // wasm/helpers/index/setBitOnByte
                    var1 = var30;
                    var27 = 3i32;
                } else {
                    let var31 = var1;
                    let var32 = self.func14(imports, 0i32, var31); // wasm/helpers/index/resetBitOnByte
                    let var33 = self.func14(imports, 1i32, var32); // wasm/helpers/index/resetBitOnByte
                    var1 = var33;
                    let var34 = var1;
                    let var35 = self.func2(imports, 3i32, var34); // wasm/helpers/index/checkBitOnByte
                    var2 = var35;
                    var27 = 0i32;
                }
                var20 = var27;
            }
            var13 = var20;
        }
        var0 = var13;
        let var36 = var0;
        if (var10 & 3i32 != var36) as i32 != 0 {
            let var37 = var2;
            if var37 != 0 {
                self.func104(imports); // wasm/interrupts/index/requestLcdInterrupt
            }
            let var38 = var0;
            var2 = (var38 == 0) as i32;
            let var39 = var2;
            let var40: i32;
            if var39 != 0 {
                let var41 = var3;
                let var42 = self.func18(imports, 65349i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var40 = (var41 == var42) as i32;
            } else {
                let var43 = var2;
                var40 = var43;
            }
            if var40 & 1i32 != 0 {
                let var44 = var1;
                let var45 = self.func15(imports, 2i32, var44); // wasm/helpers/index/setBitOnByte
                var1 = var45;
                let var46 = var1;
                let var47 = self.func2(imports, 6i32, var46); // wasm/helpers/index/checkBitOnByte
                if var47 != 0 {
                    self.func104(imports); // wasm/interrupts/index/requestLcdInterrupt
                }
            } else {
                let var48 = var1;
                let var49 = self.func14(imports, 2i32, var48); // wasm/helpers/index/resetBitOnByte
                var1 = var49;
            }
        }
        let var50 = var0;
        self.global19 = var50;
        let var51 = var1;
        self.func8(imports, 65345i32, var51); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/graphics/renderUtils/getTileDataAddress
    fn func106<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4: i32;
        if (var3 == 34816i32) as i32 != 0 {
            let var5 = var1;
            var2 = var5.wrapping_add(128i32) & 255i32;
            let var6 = var1;
            let var7 = self.func2(imports, 7i32, var6); // wasm/helpers/index/checkBitOnByte
            if var7 != 0 {
                let var8 = var1;
                var2 = var8.wrapping_sub(128i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
            }
            let var9 = var0;
            let var10 = var2;
            var4 = var9.wrapping_add(var10.wrapping_mul(16i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32) & 65535i32) & 65535i32;
        } else {
            let var11 = var0;
            let var12 = var1;
            var4 = var11.wrapping_add(var12.wrapping_mul(16i32)) & 65535i32;
        }
        var4
    }
    // wasm/graphics/renderUtils/getColorFromPalette
    fn func107<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = self.func13(imports, var2); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var4 = var0;
        (var3 as u32).wrapping_shr((var4.wrapping_mul(2i32) & 255i32) as u32) as i32 & 3i32
    }
    // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
    fn func108<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = var1;
        self.memory.store8(var2 as usize, var3.wrapping_add(1i32) as u8);
    }
    // wasm/graphics/background/renderBackground
    fn func109<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let var10 = self.func13(imports, 65347i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var7 = var10;
        let var11 = var0;
        let var12 = self.func13(imports, 65346i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5 = var11.wrapping_add(var12) & 65535i32;
        let var13 = var5;
        if (var13 as u32 >= 256i32 as u32) as i32 != 0 {
            let var14 = var5;
            var5 = var14.wrapping_sub(256i32) & 65535i32;
        }
        let var15 = var0;
        var8 = var15.wrapping_mul(160i32).wrapping_add(187904i32);
        'label0: loop {
            let var16 = var4;
            if ((var16) < 160i32) as i32 != 0 {
                let var17 = var4;
                let var18 = var7;
                var3 = var17.wrapping_add(var18);
                let var19 = var3;
                if (var19 >= 256i32) as i32 != 0 {
                    let var20 = var3;
                    var3 = var20.wrapping_sub(256i32);
                }
                let var21 = var1;
                let var22 = var2;
                let var23 = var5;
                let var24 = var3;
                let var25 = self.func13(imports, var22.wrapping_add(((var23 as u32).wrapping_shr(3i32 as u32) as i32).wrapping_mul(32i32) & 65535i32).wrapping_add(var24.wrapping_shr(3i32 as u32) & 65535i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                let var26 = self.func106(imports, var21, var25); // wasm/graphics/renderUtils/getTileDataAddress
                var0 = var26;
                let var27 = var0;
                let var28 = var5;
                var6 = (var28 as u32).wrapping_rem(8i32 as u32) as i32;
                let var29 = var6;
                let var30 = self.func13(imports, var27.wrapping_add(var29.wrapping_mul(2i32) & 65535i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var9 = var30;
                let var31 = var0;
                let var32 = var6;
                let var33 = self.func13(imports, var31.wrapping_add(var32.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var6 = var33;
                var0 = 0i32;
                let var34 = var3;
                var3 = var34.wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_rem(8i32).wrapping_sub(7i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32) & 255i32;
                let var35 = var3;
                let var36 = var6;
                let var37 = self.func2(imports, var35, var36); // wasm/helpers/index/checkBitOnByte
                if var37 != 0 {
                    var0 = 2i32;
                }
                let var38 = var3;
                let var39 = var9;
                let var40 = self.func2(imports, var38, var39); // wasm/helpers/index/checkBitOnByte
                if var40 != 0 {
                    let var41 = var0;
                    var0 = var41.wrapping_add(1i32) & 255i32;
                }
                let var42 = var8;
                let var43 = var4;
                let var44 = var0;
                let var45 = self.func107(imports, var44, 65351i32); // wasm/graphics/renderUtils/getColorFromPalette
                self.func108(imports, var42.wrapping_add(var43), var45); // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
                let var46 = var4;
                var4 = var46.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/graphics/window/renderWindow
    fn func110<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        let mut var8: i32 = 0;
        let mut var9: i32 = 0;
        let var10 = self.func13(imports, 65355i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5 = var10;
        let var11 = var0;
        let var12 = self.func13(imports, 65354i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var3 = var12;
        let var13 = var3;
        if ((var11 as u32) < (var13 & 255i32) as u32) as i32 != 0 {
            return;
        }
        let var14 = var0;
        let var15 = var3;
        var7 = var14.wrapping_sub(var15) & 65535i32;
        let var16 = var0;
        var8 = var16.wrapping_mul(160i32).wrapping_add(187904i32);
        let var17 = var5;
        var5 = var17.wrapping_sub(7i32) & 65535i32;
        let var18 = var5;
        var3 = var18;
        'label0: loop {
            let var19 = var3;
            if ((var19 as u32) < 160i32 as u32) as i32 != 0 {
                let var20 = var3;
                let var21 = var5;
                var4 = var20.wrapping_sub(var21) & 65535i32;
                let var22 = var4;
                if (var22 as u32 >= 256i32 as u32) as i32 != 0 {
                    let var23 = var4;
                    var4 = var23.wrapping_sub(256i32) & 65535i32;
                }
                let var24 = var1;
                let var25 = var2;
                let var26 = var7;
                let var27 = var4;
                let var28 = self.func13(imports, var25.wrapping_add(((var26 as u32).wrapping_shr(3i32 as u32) as i32).wrapping_mul(32i32) & 65535i32).wrapping_add((var27 as u32).wrapping_shr(3i32 as u32) as i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                let var29 = self.func106(imports, var24, var28); // wasm/graphics/renderUtils/getTileDataAddress
                var0 = var29;
                let var30 = var0;
                let var31 = var7;
                var6 = (var31 as u32).wrapping_rem(8i32 as u32) as i32;
                let var32 = var6;
                let var33 = self.func13(imports, var30.wrapping_add(var32.wrapping_mul(2i32) & 65535i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var9 = var33;
                let var34 = var0;
                let var35 = var6;
                let var36 = self.func13(imports, var34.wrapping_add(var35.wrapping_mul(2i32) & 65535i32).wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var6 = var36;
                var0 = 0i32;
                let var37 = var4;
                var4 = var37.wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_rem(8i32).wrapping_sub(7i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32) & 255i32;
                let var38 = var4;
                let var39 = var6;
                let var40 = self.func2(imports, var38, var39); // wasm/helpers/index/checkBitOnByte
                if var40 != 0 {
                    var0 = 2i32;
                }
                let var41 = var4;
                let var42 = var9;
                let var43 = self.func2(imports, var41, var42); // wasm/helpers/index/checkBitOnByte
                if var43 != 0 {
                    let var44 = var0;
                    var0 = var44.wrapping_add(1i32) & 255i32;
                }
                let var45 = var8;
                let var46 = var3;
                let var47 = var0;
                let var48 = self.func107(imports, var47, 65351i32); // wasm/graphics/renderUtils/getColorFromPalette
                self.func108(imports, var45.wrapping_add(var46), var48); // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
                let var49 = var3;
                var3 = var49.wrapping_add(1i32) & 65535i32;
                continue 'label0;
            }
            break;
        }
    }
    // wasm/memory/memory/getPixelOnFrame
    fn func111<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        let var4 = self.memory.load8(var2.wrapping_mul(160i32).wrapping_add(187904i32).wrapping_add(var3) as usize) as i32;
        var4
    }
    // wasm/memory/memory/setPixelOnFrame
    fn func112<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let var3 = var1;
        let var4 = var0;
        let var5 = var2;
        self.memory.store8(var3.wrapping_mul(160i32).wrapping_add(187904i32).wrapping_add(var4) as usize, var5.wrapping_add(1i32) as u8);
    }
    // wasm/graphics/sprites/renderSprites
    fn func113<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
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
        'label0: loop {
            let var13 = var5;
            if ((var13) < 40i32) as i32 != 0 {
                let var14 = var5;
                var2 = var14.wrapping_mul(4i32) & 65535i32;
                let var15 = var2;
                let var16 = self.func13(imports, var15.wrapping_add(65024i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var4 = var16.wrapping_sub(16i32) & 255i32;
                let var17 = var2;
                let var18 = self.func13(imports, var17.wrapping_add(65025i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var9 = var18.wrapping_sub(8i32) & 255i32;
                let var19 = var2;
                let var20 = self.func13(imports, var19.wrapping_add(65026i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var6 = var20;
                let var21 = var2;
                let var22 = self.func13(imports, var21.wrapping_add(65027i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var2 = var22;
                let var23 = var2;
                let var24 = self.func2(imports, 7i32, var23); // wasm/helpers/index/checkBitOnByte
                var10 = var24;
                let var25 = var2;
                let var26 = self.func2(imports, 6i32, var25); // wasm/helpers/index/checkBitOnByte
                var7 = var26;
                let var27 = var2;
                let var28 = self.func2(imports, 5i32, var27); // wasm/helpers/index/checkBitOnByte
                var11 = var28;
                var8 = 65352i32;
                let var29 = var2;
                let var30 = self.func2(imports, 4i32, var29); // wasm/helpers/index/checkBitOnByte
                if var30 != 0 {
                    var8 = 65353i32;
                }
                var3 = 8i32;
                let var31 = var1;
                if var31 != 0 {
                    var3 = 16i32;
                }
                let var32 = var0;
                let var33 = var4;
                var2 = (var32 as u32 >= var33 as u32) as i32;
                let var34 = var2;
                let var35: i32;
                if var34 != 0 {
                    let var36 = var0;
                    let var37 = var4;
                    let var38 = var3;
                    var35 = ((var36 as u32) < (var37.wrapping_add(var38) & 255i32) as u32) as i32;
                } else {
                    let var39 = var2;
                    var35 = var39;
                }
                if var35 & 1i32 != 0 {
                    let var40 = var0;
                    let var41 = var4;
                    var2 = var40.wrapping_sub(var41) & 255i32;
                    let var42 = var7;
                    if var42 != 0 {
                        let var43 = var2;
                        let var44 = var3;
                        var2 = var43.wrapping_sub(var44).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32).wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
                    }
                    let var45 = var6;
                    let var46 = self.func106(imports, 32768i32, var45); // wasm/graphics/renderUtils/getTileDataAddress
                    let var47 = var2;
                    var2 = var46.wrapping_add(var47.wrapping_mul(2i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32)) & 65535i32;
                    let var48 = var2;
                    let var49 = self.func13(imports, var48); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    var6 = var49;
                    let var50 = var2;
                    let var51 = self.func13(imports, var50.wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    var7 = var51;
                    var2 = 7i32;
                    'label1: loop {
                        let var52 = var2;
                        if (var52 >= 0i32) as i32 != 0 {
                            let var53 = var2;
                            var4 = var53;
                            let var54 = var11;
                            if var54 != 0 {
                                let var55 = var4;
                                var4 = var55.wrapping_sub(7i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32).wrapping_mul(-1i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            }
                            var3 = 0i32;
                            let var56 = var4;
                            let var57 = var7;
                            let var58 = self.func2(imports, var56 & 255i32, var57); // wasm/helpers/index/checkBitOnByte
                            if var58 != 0 {
                                var3 = 2i32;
                            }
                            let var59 = var4;
                            let var60 = var6;
                            let var61 = self.func2(imports, var59 & 255i32, var60); // wasm/helpers/index/checkBitOnByte
                            if var61 != 0 {
                                let var62 = var3;
                                var3 = var62.wrapping_add(1i32) & 255i32;
                            }
                            let var63 = var3;
                            if var63 != 0 {
                                let var64 = var3;
                                let var65 = var8;
                                let var66 = self.func107(imports, var64, var65); // wasm/graphics/renderUtils/getColorFromPalette
                                var3 = var66;
                                let var67 = var9;
                                let var68 = var2;
                                var4 = var67.wrapping_add(7i32.wrapping_sub(var68 & 255i32) & 255i32) & 255i32;
                                let var69 = var10;
                                var12 = (var69 == 0) as i32;
                                let var70 = var12;
                                let var71: i32;
                                if var70 != 0 {
                                    let var72 = var12;
                                    var71 = var72;
                                } else {
                                    let var73 = var4;
                                    let var74 = var0;
                                    let var75 = self.func111(imports, var73, var74); // wasm/memory/memory/getPixelOnFrame
                                    var71 = (var75 as u32 <= 1i32 as u32) as i32;
                                }
                                if var71 & 1i32 != 0 {
                                    let var76 = var4;
                                    let var77 = var0;
                                    let var78 = var3;
                                    self.func112(imports, var76, var77, var78); // wasm/memory/memory/setPixelOnFrame
                                }
                            }
                            let var79 = var2;
                            var2 = var79.wrapping_sub(1i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            continue 'label1;
                        }
                        break;
                    }
                }
                let var80 = var5;
                var5 = var80.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/graphics/graphics/_drawScanline
    fn func114<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        var3 = 34816i32;
        let var4 = self.func18(imports, 65344i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var1 = var4;
        let var5 = var1;
        let var6 = self.func2(imports, 4i32, var5); // wasm/helpers/index/checkBitOnByte
        if var6 != 0 {
            var3 = 32768i32;
        }
        let var7 = var1;
        let var8 = self.func2(imports, 0i32, var7); // wasm/helpers/index/checkBitOnByte
        if var8 != 0 {
            var2 = 38912i32;
            let var9 = var1;
            let var10 = self.func2(imports, 3i32, var9); // wasm/helpers/index/checkBitOnByte
            if var10 != 0 {
                var2 = 39936i32;
            }
            let var11 = var0;
            let var12 = var3;
            let var13 = var2;
            self.func109(imports, var11, var12, var13); // wasm/graphics/background/renderBackground
        }
        let var14 = var1;
        let var15 = self.func2(imports, 5i32, var14); // wasm/helpers/index/checkBitOnByte
        if var15 != 0 {
            var2 = 38912i32;
            let var16 = var1;
            let var17 = self.func2(imports, 6i32, var16); // wasm/helpers/index/checkBitOnByte
            if var17 != 0 {
                var2 = 39936i32;
            }
            let var18 = var0;
            let var19 = var3;
            let var20 = var2;
            self.func110(imports, var18, var19, var20); // wasm/graphics/window/renderWindow
        }
        let var21 = var1;
        let var22 = self.func2(imports, 1i32, var21); // wasm/helpers/index/checkBitOnByte
        if var22 != 0 {
            let var23 = var0;
            let var24 = var1;
            let var25 = self.func2(imports, 2i32, var24); // wasm/helpers/index/checkBitOnByte
            self.func113(imports, var23, var25); // wasm/graphics/sprites/renderSprites
        }
    }
    // wasm/memory/memory/storeFrameToBeRendered
    fn func115<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        var2 = 164864i32;
        var3 = 187904i32;
        'label0: loop {
            let var4 = var0;
            if ((var4) < 144i32) as i32 != 0 {
                var1 = 0i32;
                'label1: loop {
                    let var5 = var1;
                    if ((var5) < 160i32) as i32 != 0 {
                        let var6 = var1;
                        let var7 = var0;
                        let var8 = var0;
                        let var9 = var1;
                        let var10 = self.memory.load8(187904i32.wrapping_add(var8.wrapping_mul(160i32)).wrapping_add(var9) as usize) as i32;
                        self.memory.store8(164864i32.wrapping_add(var6).wrapping_add(var7.wrapping_mul(160i32)) as usize, var10 as u8);
                        let var11 = var1;
                        var1 = var11.wrapping_add(1i32);
                        continue 'label1;
                    }
                    break;
                }
                let var12 = var0;
                var0 = var12.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/interrupts/index/requestVBlankInterrupt
    fn func116<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func100(imports, 0i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/graphics/graphics/updateGraphics
    fn func117<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.func103(imports); // wasm/graphics/lcd/isLcdEnabled
        var1 = var2;
        let var3 = var1;
        self.func105(imports, var3); // wasm/graphics/lcd/setLcdStatus
        let var4 = var1;
        if var4 != 0 {
            let var5 = self.global60;
            let var6 = var0;
            self.global60 = var5.wrapping_add(var6);
            let var7 = self.global60;
            if (var7 >= 456i32) as i32 != 0 {
                let var8 = self.global60;
                self.global60 = var8.wrapping_sub(456i32);
                let var9 = self.func18(imports, 65348i32); // wasm/memory/load/eightBitLoadFromGBMemory
                var0 = var9;
                let var10 = var0;
                if (var10 == 144i32) as i32 != 0 {
                    let var11 = var0;
                    self.func114(imports, var11); // wasm/graphics/graphics/_drawScanline
                    self.func115(imports); // wasm/memory/memory/storeFrameToBeRendered
                    self.func116(imports); // wasm/interrupts/index/requestVBlankInterrupt
                } else {
                    let var12 = var0;
                    if ((var12 as u32) < 144i32 as u32) as i32 != 0 {
                        let var13 = var0;
                        self.func114(imports, var13); // wasm/graphics/graphics/_drawScanline
                    }
                }
                let var14 = var0;
                let var15: i32;
                if (var14 as u32 > 153i32 as u32) as i32 != 0 {
                    var15 = 0i32;
                } else {
                    let var16 = var0;
                    var15 = var16.wrapping_add(1i32) & 255i32;
                }
                var0 = var15;
                let var17 = var0;
                self.func8(imports, 65348i32, var17); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            }
        }
    }
    // wasm/interrupts/index/_handleInterrupt
    fn func118<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.func93(imports, 0i32); // wasm/interrupts/index/setInterrupts
        let var1 = var0;
        let var2 = self.func18(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var3 = self.func14(imports, var1, var2); // wasm/helpers/index/resetBitOnByte
        self.func8(imports, 65295i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        let var4 = self.global9;
        self.global9 = var4.wrapping_sub(2i32) & 65535i32;
        let var5 = self.global9;
        let var6 = self.global0;
        self.func11(imports, var5, var6); // wasm/memory/store/sixteenBitStoreIntoGBMemorySkipTraps
        let var7 = var0;
        if var7 != 0 {
            let var8 = var0;
            if (var8 == 1i32) as i32 != 0 {
                self.global0 = 72i32;
            } else {
                let var9 = var0;
                if (var9 == 2i32) as i32 != 0 {
                    self.global0 = 80i32;
                } else {
                    let var10 = var0;
                    if (var10 == 4i32) as i32 != 0 {
                        self.global0 = 96i32;
                    }
                }
            }
        } else {
            self.global0 = 64i32;
        }
        self.global53 = 0i32;
    }
    // wasm/interrupts/index/checkInterrupts
    fn func119<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global55;
        if var4 != 0 {
            let var5 = self.func18(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = var5;
            let var6 = self.func18(imports, 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var3 = var6;
            let var7 = var2;
            if (var7 as u32 > 0i32 as u32) as i32 != 0 {
                let var8 = var2;
                let var9 = self.func2(imports, 0i32, var8); // wasm/helpers/index/checkBitOnByte
                var0 = var9;
                let var10 = var0;
                let var11: i32;
                if var10 != 0 {
                    let var12 = var3;
                    let var13 = self.func2(imports, 0i32, var12); // wasm/helpers/index/checkBitOnByte
                    var11 = var13;
                } else {
                    let var14 = var0;
                    var11 = var14;
                }
                if var11 & 1i32 != 0 {
                    self.func118(imports, 0i32); // wasm/interrupts/index/_handleInterrupt
                    var1 = 1i32;
                } else {
                    let var15 = var2;
                    let var16 = self.func2(imports, 1i32, var15); // wasm/helpers/index/checkBitOnByte
                    var0 = var16;
                    let var17 = var0;
                    let var18: i32;
                    if var17 != 0 {
                        let var19 = var3;
                        let var20 = self.func2(imports, 1i32, var19); // wasm/helpers/index/checkBitOnByte
                        var18 = var20;
                    } else {
                        let var21 = var0;
                        var18 = var21;
                    }
                    if var18 & 1i32 != 0 {
                        self.func118(imports, 1i32); // wasm/interrupts/index/_handleInterrupt
                        var1 = 1i32;
                    } else {
                        let var22 = var2;
                        let var23 = self.func2(imports, 2i32, var22); // wasm/helpers/index/checkBitOnByte
                        var0 = var23;
                        let var24 = var0;
                        let var25: i32;
                        if var24 != 0 {
                            let var26 = var3;
                            let var27 = self.func2(imports, 2i32, var26); // wasm/helpers/index/checkBitOnByte
                            var25 = var27;
                        } else {
                            let var28 = var0;
                            var25 = var28;
                        }
                        if var25 & 1i32 != 0 {
                            self.func118(imports, 2i32); // wasm/interrupts/index/_handleInterrupt
                            var1 = 1i32;
                        } else {
                            let var29 = var2;
                            let var30 = self.func2(imports, 4i32, var29); // wasm/helpers/index/checkBitOnByte
                            var0 = var30;
                            let var31 = var0;
                            let var32: i32;
                            if var31 != 0 {
                                let var33 = var3;
                                let var34 = self.func2(imports, 4i32, var33); // wasm/helpers/index/checkBitOnByte
                                var32 = var34;
                            } else {
                                let var35 = var0;
                                var32 = var35;
                            }
                            if var32 & 1i32 != 0 {
                                self.func118(imports, 4i32); // wasm/interrupts/index/_handleInterrupt
                                var1 = 1i32;
                            }
                        }
                    }
                }
            }
        }
        let var36 = var1;
        let var37: i32;
        if var36 != 0 {
            var37 = 20i32;
        } else {
            var37 = 0i32;
        }
        var37
    }
    // wasm/sound/length/isChannelLengthEnabled
    fn func120<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func21(imports, var1); // wasm/sound/registers/getRegister4OfChannel
        let var3 = self.func2(imports, 6i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/channel1/Channel1.updateLength
    fn func121<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global28;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func120(imports, 1i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global28;
            self.global28 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global28;
        if (var7 == 0) as i32 != 0 {
            self.global32 = 0i32;
        }
    }
    // wasm/sound/channel2/Channel2.updateLength
    fn func122<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global29;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func120(imports, 2i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global29;
            self.global29 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global29;
        if (var7 == 0) as i32 != 0 {
            self.global39 = 0i32;
        }
    }
    // wasm/sound/channel3/Channel3.updateLength
    fn func123<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global30;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func120(imports, 3i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global30;
            self.global30 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global30;
        if (var7 == 0) as i32 != 0 {
            self.global43 = 0i32;
        }
    }
    // wasm/sound/channel4/Channel4.updateLength
    fn func124<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global31;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func120(imports, 4i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global31;
            self.global31 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global31;
        if (var7 == 0) as i32 != 0 {
            self.global46 = 0i32;
        }
    }
    // wasm/sound/channel1/Channel1.updateSweep
    fn func125<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global37;
        self.global37 = var0.wrapping_sub(1i32);
        let var1 = self.global37;
        if (var1 <= 0i32) as i32 != 0 {
            let var2 = self.func27(imports); // wasm/sound/channel1/getSweepPeriod
            self.global37 = var2;
            let var3 = self.global38;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.func27(imports); // wasm/sound/channel1/getSweepPeriod
                var4 = (var5 as u32 > 0i32 as u32) as i32;
            } else {
                let var6 = self.global38;
                var4 = var6;
            }
            if var4 & 1i32 != 0 {
                self.func33(imports); // wasm/sound/channel1/calculateSweepAndCheckOverflow
            }
        }
    }
    // wasm/sound/envelope/getChannelEnvelopeAddMode
    fn func126<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func24(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        let var3 = self.func2(imports, 3i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/channel1/Channel1.updateEnvelope
    fn func127<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global34;
        self.global34 = var1.wrapping_sub(1i32);
        let var2 = self.global34;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func25(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global34 = var3;
            let var4 = self.global34;
            if var4 != 0 {
                let var5 = self.func126(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global35;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global35;
                    self.global35 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func126(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global35;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global35;
                        self.global35 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/channel2/Channel2.updateEnvelope
    fn func128<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global41;
        self.global41 = var1.wrapping_sub(1i32);
        let var2 = self.global41;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func25(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global41 = var3;
            let var4 = self.global41;
            if var4 != 0 {
                let var5 = self.func126(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global42;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global42;
                    self.global42 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func126(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global42;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global42;
                        self.global42 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/channel4/Channel4.updateEnvelope
    fn func129<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global48;
        self.global48 = var1.wrapping_sub(1i32);
        let var2 = self.global48;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func25(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global48 = var3;
            let var4 = self.global48;
            if var4 != 0 {
                let var5 = self.func126(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global49;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global49;
                    self.global49 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func126(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global49;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global49;
                        self.global49 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/duty/getChannelDuty
    fn func130<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func19(imports, var1); // wasm/sound/registers/getRegister1OfChannel
        (var2 as u32).wrapping_shr(6i32 as u32) as i32 & 3i32
    }
    // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
    fn func131<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func130(imports, var3); // wasm/sound/duty/getChannelDuty
        var2 = var4;
        let var5 = var2;
        let var6: i32;
        if (var5 == 1i32) as i32 != 0 {
            let var7 = var1;
            let var8 = self.func2(imports, var7, 129i32); // wasm/helpers/index/checkBitOnByte
            var6 = var8;
        } else {
            let var9 = var2;
            let var10: i32;
            if (var9 == 2i32) as i32 != 0 {
                let var11 = var1;
                let var12 = self.func2(imports, var11, 135i32); // wasm/helpers/index/checkBitOnByte
                var10 = var12;
            } else {
                let var13 = var2;
                let var14: i32;
                if (var13 == 3i32) as i32 != 0 {
                    let var15 = var1;
                    let var16 = self.func2(imports, var15, 126i32); // wasm/helpers/index/checkBitOnByte
                    var14 = var16;
                } else {
                    let var17 = var1;
                    let var18 = self.func2(imports, var17, 1i32); // wasm/helpers/index/checkBitOnByte
                    var14 = var18;
                }
                var10 = var14;
            }
            var6 = var10;
        }
        var6
    }
    // wasm/sound/channel1/Channel1.getSample
    fn func132<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global33;
        let var3 = var0;
        self.global33 = var2.wrapping_sub(var3);
        let var4 = self.global33;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global33;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func23(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
            self.global33 = 2048i32.wrapping_sub(var9).wrapping_mul(4i32);
            let var10 = self.global33;
            let var11 = var0;
            self.global33 = var10.wrapping_sub(var11);
            let var12 = self.global63;
            self.global63 = var12.wrapping_add(1i32) & 255i32;
            let var13 = self.global63;
            if (var13 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global63 = 0i32;
            }
        }
        let var14 = self.global32;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.func34(imports, 1i32); // wasm/sound/registers/isChannelDacEnabled
            var15 = var16;
        } else {
            let var17 = self.global32;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global35;
            var0 = var18;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var19 = self.global63;
        let var20 = self.func131(imports, 1i32, var19); // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
        if (var20 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        let var21 = var1;
        let var22 = var0;
        var21.wrapping_mul(var22).wrapping_add(15i32)
    }
    // wasm/sound/channel2/Channel2.getSample
    fn func133<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global40;
        let var3 = var0;
        self.global40 = var2.wrapping_sub(var3);
        let var4 = self.global40;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global40;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func23(imports, 2i32); // wasm/sound/frequency/getChannelFrequency
            self.global40 = 2048i32.wrapping_sub(var9).wrapping_mul(4i32);
            let var10 = self.global40;
            let var11 = var0;
            self.global40 = var10.wrapping_sub(var11);
            let var12 = self.global64;
            self.global64 = var12.wrapping_add(1i32) & 255i32;
            let var13 = self.global64;
            if (var13 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global64 = 0i32;
            }
        }
        let var14 = self.global39;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.func34(imports, 2i32); // wasm/sound/registers/isChannelDacEnabled
            var15 = var16;
        } else {
            let var17 = self.global39;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global42;
            var0 = var18;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var19 = self.global64;
        let var20 = self.func131(imports, 1i32, var19); // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
        if (var20 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        let var21 = var1;
        let var22 = var0;
        var21.wrapping_mul(var22).wrapping_add(15i32)
    }
    // wasm/sound/channel3/Channel3.getSample
    fn func134<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global44;
        let var4 = var0;
        self.global44 = var3.wrapping_sub(var4);
        let var5 = self.global44;
        if (var5 <= 0i32) as i32 != 0 {
            let var6 = self.global44;
            var1 = var6;
            let var7 = var1;
            let var8 = var1;
            let var9 = var1;
            var1 = if (var9 > 0i32) as i32 != 0 { var7 } else { 0i32.wrapping_sub(var8) };
            let var10 = self.func23(imports, 3i32); // wasm/sound/frequency/getChannelFrequency
            self.global44 = 2048i32.wrapping_sub(var10).wrapping_mul(2i32);
            let var11 = self.global44;
            let var12 = var1;
            self.global44 = var11.wrapping_sub(var12);
            let var13 = self.global45;
            self.global45 = var13.wrapping_add(1i32) & 65535i32;
            let var14 = self.global45;
            if (var14 as u32 >= 32i32 as u32) as i32 != 0 {
                self.global45 = 0i32;
            }
        }
        var1 = 0i32;
        let var15 = self.global43;
        let var16: i32;
        if var15 != 0 {
            let var17 = self.func34(imports, 3i32); // wasm/sound/registers/isChannelDacEnabled
            var16 = var17;
        } else {
            let var18 = self.global43;
            var16 = var18;
        }
        if var16 & 1i32 != 0 {
            let var19 = self.func18(imports, 65308i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var2 = (var19 as u32).wrapping_shr(5i32 as u32) as i32 & 15i32;
        } else {
            return 15i32;
        }
        let var20 = self.global45;
        let var21 = self.func18(imports, ((var20 as u32 / 2i32 as u32) as i32 & 65535i32).wrapping_add(65328i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
        var0 = var21;
        let var22 = self.global45;
        let var23: i32;
        if (var22 as u32).wrapping_rem(2i32 as u32) as i32 != 0 {
            let var24 = var0;
            var23 = var24 & 15i32;
        } else {
            let var25 = var0;
            var23 = var25.wrapping_shr(4i32 as u32) & 15i32;
        }
        var0 = var23;
        let var26 = var2;
        if (var26 as u32 <= 0i32 as u32) as i32 != 0 {
            let var27 = var0;
            var0 = var27.wrapping_shr(4i32 as u32);
        } else {
            let var28 = var2;
            if (var28 == 1i32) as i32 != 0 {
                var1 = 1i32;
            } else {
                let var29 = var2;
                let var30: i32;
                if (var29 == 2i32) as i32 != 0 {
                    var1 = 2i32;
                    let var31 = var0;
                    var30 = var31.wrapping_shr(1i32 as u32);
                } else {
                    var1 = 4i32;
                    let var32 = var0;
                    var30 = var32.wrapping_shr(2i32 as u32);
                }
                var0 = var30;
            }
        }
        let var33 = var1;
        let var34: i32;
        if (var33 > 0i32) as i32 != 0 {
            let var35 = var0;
            let var36 = var1;
            var34 = (var35 / var36).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        } else {
            var34 = 0i32;
        }
        var0 = var34;
        let var37 = var0;
        var37.wrapping_add(15i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32)
    }
    // wasm/sound/channel4/Channel4.isNoiseChannelWidthModeSet
    fn func135<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func18(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var1 = self.func2(imports, 3i32, var0); // wasm/helpers/index/checkBitOnByte
        var1
    }
    // wasm/sound/channel4/Channel4.getSample
    fn func136<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global47;
        let var3 = var0;
        self.global47 = var2.wrapping_sub(var3);
        let var4 = self.global47;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global47;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func40(imports); // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
            self.global47 = var9;
            let var10 = self.global47;
            let var11 = var0;
            self.global47 = var10.wrapping_sub(var11);
            let var12 = self.global50;
            let var13 = self.global50;
            var1 = var12 & 1i32 ^ (var13 as u32).wrapping_shr(1i32 as u32) as i32 & 1i32;
            let var14 = self.global50;
            self.global50 = (var14 as u32).wrapping_shr(1i32 as u32) as i32;
            let var15 = self.global50;
            let var16 = var1;
            self.global50 = (var15 | var16.wrapping_shl(14i32 as u32) & 65535i32) & 65535i32;
            let var17 = self.func135(imports); // wasm/sound/channel4/Channel4.isNoiseChannelWidthModeSet
            if var17 != 0 {
                let var18 = self.global50;
                self.global50 = var18 & 65471i32;
                let var19 = self.global50;
                let var20 = var1;
                self.global50 = (var19 | var20.wrapping_shl(6i32 as u32) & 65535i32) & 65535i32;
            }
        }
        let var21 = self.global46;
        let var22: i32;
        if var21 != 0 {
            let var23 = self.func34(imports, 4i32); // wasm/sound/registers/isChannelDacEnabled
            var22 = var23;
        } else {
            let var24 = self.global46;
            var22 = var24;
        }
        if var22 & 1i32 != 0 {
            let var25 = self.global49;
            var1 = var25;
        } else {
            return 15i32;
        }
        let var26 = self.global50;
        let var27 = self.func2(imports, 0i32, var26 & 255i32); // wasm/helpers/index/checkBitOnByte
        let var28: i32;
        if var27 != 0 {
            var28 = -1i32;
        } else {
            var28 = 1i32;
        }
        var0 = var28;
        let var29 = var0;
        let var30 = var1;
        var29.wrapping_mul(var30).wrapping_add(15i32)
    }
    // wasm/sound/registers/isChannelEnabledOnLeftOutput
    fn func137<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func18(imports, 65317i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var3 = self.func2(imports, ((var1 & 255i32).wrapping_sub(1i32) & 255i32).wrapping_add(4i32) & 255i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/registers/isChannelEnabledOnRightOutput
    fn func138<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func18(imports, 65317i32); // wasm/memory/load/eightBitLoadFromGBMemory
        let var3 = self.func2(imports, (var1 & 255i32).wrapping_sub(1i32) & 255i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/sound/getSampleAsUnsignedByte
    fn func139<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        var1.wrapping_mul(1000i32) / 3779i32 & 255i32
    }
    // wasm/memory/memory/setLeftAndRightOutputForAudioQueue
    fn func140<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let mut var3: i32 = 0;
        let var4 = var2;
        var3 = var4.wrapping_mul(2i32).wrapping_add(342016i32);
        let var5 = var3;
        let var6 = var0;
        self.memory.store8(var5 as usize, var6.wrapping_add(1i32) as u8);
        let var7 = var3;
        let var8 = var1;
        self.memory.store8(var7.wrapping_add(1i32) as usize, var8.wrapping_add(1i32) as u8);
    }
    // wasm/sound/sound/resetAudioQueue
    fn func141<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global52 = 0i32;
    }
    // wasm/sound/sound/updateSound
    fn func142<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
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
        let var12 = self.global61;
        let var13 = var0;
        self.global61 = var12.wrapping_add(var13);
        let var14 = self.global61;
        if (var14 >= 8192i32) as i32 != 0 {
            let var15 = self.global61;
            self.global61 = var15.wrapping_sub(8192i32);
            let var16 = self.global62;
            if var16 != 0 {
                let var17 = self.global62;
                if (var17 == 2i32) as i32 != 0 {
                    self.func121(imports); // wasm/sound/channel1/Channel1.updateLength
                    self.func122(imports); // wasm/sound/channel2/Channel2.updateLength
                    self.func123(imports); // wasm/sound/channel3/Channel3.updateLength
                    self.func124(imports); // wasm/sound/channel4/Channel4.updateLength
                    self.func125(imports); // wasm/sound/channel1/Channel1.updateSweep
                } else {
                    let var18 = self.global62;
                    if (var18 == 4i32) as i32 != 0 {
                        self.func121(imports); // wasm/sound/channel1/Channel1.updateLength
                        self.func122(imports); // wasm/sound/channel2/Channel2.updateLength
                        self.func123(imports); // wasm/sound/channel3/Channel3.updateLength
                        self.func124(imports); // wasm/sound/channel4/Channel4.updateLength
                    } else {
                        let var19 = self.global62;
                        if (var19 == 6i32) as i32 != 0 {
                            self.func121(imports); // wasm/sound/channel1/Channel1.updateLength
                            self.func122(imports); // wasm/sound/channel2/Channel2.updateLength
                            self.func123(imports); // wasm/sound/channel3/Channel3.updateLength
                            self.func124(imports); // wasm/sound/channel4/Channel4.updateLength
                            self.func125(imports); // wasm/sound/channel1/Channel1.updateSweep
                        } else {
                            let var20 = self.global62;
                            if (var20 == 7i32) as i32 != 0 {
                                self.func127(imports); // wasm/sound/channel1/Channel1.updateEnvelope
                                self.func128(imports); // wasm/sound/channel2/Channel2.updateEnvelope
                                self.func129(imports); // wasm/sound/channel4/Channel4.updateEnvelope
                            }
                        }
                    }
                }
            } else {
                self.func121(imports); // wasm/sound/channel1/Channel1.updateLength
                self.func122(imports); // wasm/sound/channel2/Channel2.updateLength
                self.func123(imports); // wasm/sound/channel3/Channel3.updateLength
                self.func124(imports); // wasm/sound/channel4/Channel4.updateLength
            }
            let var21 = self.global62;
            self.global62 = var21.wrapping_add(1i32) & 255i32;
            let var22 = self.global62;
            if (var22 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global62 = 0i32;
            }
        }
        let var23 = var0;
        let var24 = self.func132(imports, var23); // wasm/sound/channel1/Channel1.getSample
        var3 = var24;
        let var25 = var0;
        let var26 = self.func133(imports, var25); // wasm/sound/channel2/Channel2.getSample
        var4 = var26;
        let var27 = var0;
        let var28 = self.func134(imports, var27); // wasm/sound/channel3/Channel3.getSample
        var5 = var28;
        let var29 = var0;
        let var30 = self.func136(imports, var29); // wasm/sound/channel4/Channel4.getSample
        var6 = var30;
        let var31 = self.global65;
        let var32 = var0;
        self.global65 = var31.wrapping_add(var32) & 255i32;
        let var33 = self.global65;
        if (var33 as u32 >= 87i32 as u32) as i32 != 0 {
            let var34 = self.global65;
            self.global65 = var34.wrapping_sub(87i32) & 255i32;
            let var35 = self.func18(imports, 65316i32); // wasm/memory/load/eightBitLoadFromGBMemory
            var0 = var35;
            let var36 = var0;
            var7 = (var36 as u32).wrapping_shr(4i32 as u32) as i32 & 7i32;
            let var37 = var0;
            var0 = var37 & 7i32;
            var8 = 1i32;
            let var38 = var8;
            let var39 = self.func137(imports, var38); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            if var39 != 0 {
                let var40 = var3;
                var1 = 0i32.wrapping_add(var40);
            }
            var9 = 2i32;
            let var41 = var9;
            let var42 = self.func137(imports, var41); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            if var42 != 0 {
                let var43 = var1;
                let var44 = var4;
                var1 = var43.wrapping_add(var44);
            }
            var10 = 3i32;
            let var45 = var10;
            let var46 = self.func137(imports, var45); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            if var46 != 0 {
                let var47 = var1;
                let var48 = var5;
                var1 = var47.wrapping_add(var48);
            }
            var11 = 4i32;
            let var49 = var11;
            let var50 = self.func137(imports, var49); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            if var50 != 0 {
                let var51 = var1;
                let var52 = var6;
                var1 = var51.wrapping_add(var52);
            }
            let var53 = self.func138(imports, 1i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            if var53 != 0 {
                let var54 = var3;
                var2 = 0i32.wrapping_add(var54);
            }
            let var55 = self.func138(imports, 2i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            if var55 != 0 {
                let var56 = var2;
                let var57 = var4;
                var2 = var56.wrapping_add(var57);
            }
            let var58 = self.func138(imports, 3i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            if var58 != 0 {
                let var59 = var2;
                let var60 = var5;
                var2 = var59.wrapping_add(var60);
            }
            let var61 = self.func138(imports, 4i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            if var61 != 0 {
                let var62 = var2;
                let var63 = var6;
                var2 = var62.wrapping_add(var63);
            }
            let var64 = var1;
            let var65 = var7;
            let var66 = self.func139(imports, var64.wrapping_mul(var65.wrapping_add(1i32))); // wasm/sound/sound/getSampleAsUnsignedByte
            let var67 = var2;
            let var68 = var0;
            let var69 = self.func139(imports, var67.wrapping_mul(var68.wrapping_add(1i32))); // wasm/sound/sound/getSampleAsUnsignedByte
            let var70 = self.global52;
            self.func140(imports, var66.wrapping_add(1i32) & 255i32, var69.wrapping_add(1i32) & 255i32, var70); // wasm/memory/memory/setLeftAndRightOutputForAudioQueue
            let var71 = self.global52;
            self.global52 = var71.wrapping_add(1i32);
            let var72 = self.global52;
            if (var72 >= 32766i32) as i32 != 0 {
                self.func141(imports); // wasm/sound/sound/resetAudioQueue
            }
        }
    }
    // wasm/cpu/opcodes/emulationStep
    fn func143<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        var0 = 4i32;
        let var3 = self.global53;
        var2 = (var3 == 0) as i32;
        let var4 = var2;
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global54;
            var5 = (var6 == 0) as i32;
        } else {
            let var7 = var2;
            var5 = var7;
        }
        if var5 & 1i32 != 0 {
            let var8 = self.global0;
            let var9 = self.func18(imports, var8); // wasm/memory/load/eightBitLoadFromGBMemory
            var1 = var9;
            let var10 = var1;
            let var11 = self.global0;
            let var12 = self.func18(imports, var11.wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
            let var13 = self.global0;
            let var14 = self.func18(imports, var13.wrapping_add(2i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
            let var15 = self.func94(imports, var10, var12, var14); // wasm/cpu/opcodes/executeOpcode
            var0 = var15;
            let var16 = var1;
            self.global56 = var16;
        } else {
            let var17 = self.global53;
            let var18: i32;
            if var17 != 0 {
                let var19 = self.func95(imports); // wasm/interrupts/index/areInterruptsEnabled
                var18 = (var19 == 0) as i32;
            } else {
                let var20 = self.global53;
                var18 = var20;
            }
            var2 = var18 & 1i32;
            let var21 = var2;
            let var22: i32;
            if var21 != 0 {
                let var23 = self.func96(imports); // wasm/interrupts/index/areInterruptsPending
                var22 = var23;
            } else {
                let var24 = var2;
                var22 = var24;
            }
            if var22 & 1i32 != 0 {
                self.global53 = 0i32;
                self.global54 = 0i32;
                let var25 = self.global0;
                let var26 = self.func18(imports, var25); // wasm/memory/load/eightBitLoadFromGBMemory
                var1 = var26;
                let var27 = var1;
                let var28 = self.global0;
                let var29 = self.func18(imports, var28); // wasm/memory/load/eightBitLoadFromGBMemory
                let var30 = self.global0;
                let var31 = self.func18(imports, var30.wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                let var32 = self.func94(imports, var27, var29, var31); // wasm/cpu/opcodes/executeOpcode
                var0 = var32;
                let var33 = self.global0;
                self.global0 = var33.wrapping_sub(1i32) & 65535i32;
            }
        }
        let var34 = self.global2;
        self.global2 = var34 & 240i32;
        let var35 = var0;
        self.func102(imports, var35 & 255i32); // wasm/timers/index/updateTimers
        let var36 = self.global54;
        if (var36 == 0) as i32 != 0 {
            let var37 = var0;
            self.func117(imports, var37 & 255i32); // wasm/graphics/graphics/updateGraphics
        }
        let var38 = var0;
        let var39 = self.func119(imports); // wasm/interrupts/index/checkInterrupts
        var0 = var38.wrapping_add(var39).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
        let var40 = var0;
        self.func142(imports, var40 & 255i32); // wasm/sound/sound/updateSound
        let var41 = var0;
        if (var41 <= 0i32) as i32 != 0 {
            let var42 = var1;
            self.func1(imports, 8861764i32, 1i32, var42, 0i32, 0i32, 0i32, 0i32, 0i32); // wasm/helpers/index/log
        }
        let var43 = var0;
        var43
    }
    // wasm/cpu/opcodes/update
    fn func144<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        'label0: loop {
            let var2 = var1;
            var0 = (var2 == 0) as i32;
            let var3 = var0;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.global51;
                var4 = ((var5) < 69905i32) as i32;
            } else {
                let var6 = var0;
                var4 = var6;
            }
            var0 = var4 & 1i32;
            let var7 = var0;
            let var8: i32;
            if var7 != 0 {
                let var9 = self.global52;
                var8 = ((var9) < 4096i32) as i32;
            } else {
                let var10 = var0;
                var8 = var10;
            }
            if var8 & 1i32 != 0 {
                let var11 = self.func143(imports); // wasm/cpu/opcodes/emulationStep
                var0 = var11;
                let var12 = var0;
                if (var12 >= 0i32) as i32 != 0 {
                    let var13 = self.global51;
                    let var14 = var0;
                    self.global51 = var13.wrapping_add(var14);
                } else {
                    var1 = 1i32;
                }
                continue 'label0;
            }
            break;
        }
        let var15 = self.global51;
        if (var15 >= 69905i32) as i32 != 0 {
            self.global51 = 0i32;
            return 1i32;
        } else {
            let var16 = self.global52;
            if (var16 >= 4096i32) as i32 != 0 {
                return 2i32;
            }
        }
        let var17 = self.global0;
        self.global0 = var17.wrapping_sub(1i32) & 65535i32;
        -1i32
    }
    // wasm/joypad/index/_getJoypadButtonStateFromButtonId
    fn func145<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        if var1 != 0 {
            let var2 = var0;
            if (var2 == 1i32) as i32 != 0 {
                let var3 = self.global25;
                return var3;
            } else {
                let var4 = var0;
                if (var4 == 2i32) as i32 != 0 {
                    let var5 = self.global26;
                    return var5;
                } else {
                    let var6 = var0;
                    if (var6 == 3i32) as i32 != 0 {
                        let var7 = self.global27;
                        return var7;
                    } else {
                        let var8 = var0;
                        if (var8 == 4i32) as i32 != 0 {
                            let var9 = self.global20;
                            return var9;
                        } else {
                            let var10 = var0;
                            if (var10 == 5i32) as i32 != 0 {
                                let var11 = self.global21;
                                return var11;
                            } else {
                                let var12 = var0;
                                if (var12 == 6i32) as i32 != 0 {
                                    let var13 = self.global22;
                                    return var13;
                                } else {
                                    let var14 = var0;
                                    if (var14 == 7i32) as i32 != 0 {
                                        let var15 = self.global23;
                                        return var15;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let var16 = self.global24;
            return var16;
        }
        0i32
    }
    // wasm/joypad/index/_setJoypadButtonStateFromButtonId
    fn func146<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        if var2 != 0 {
            let var3 = var0;
            if (var3 == 1i32) as i32 != 0 {
                let var4 = var1;
                self.global25 = var4;
            } else {
                let var5 = var0;
                if (var5 == 2i32) as i32 != 0 {
                    let var6 = var1;
                    self.global26 = var6;
                } else {
                    let var7 = var0;
                    if (var7 == 3i32) as i32 != 0 {
                        let var8 = var1;
                        self.global27 = var8;
                    } else {
                        let var9 = var0;
                        if (var9 == 4i32) as i32 != 0 {
                            let var10 = var1;
                            self.global20 = var10;
                        } else {
                            let var11 = var0;
                            if (var11 == 5i32) as i32 != 0 {
                                let var12 = var1;
                                self.global21 = var12;
                            } else {
                                let var13 = var0;
                                if (var13 == 6i32) as i32 != 0 {
                                    let var14 = var1;
                                    self.global22 = var14;
                                } else {
                                    let var15 = var0;
                                    if (var15 == 7i32) as i32 != 0 {
                                        let var16 = var1;
                                        self.global23 = var16;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let var17 = var1;
            self.global24 = var17;
        }
    }
    // wasm/interrupts/index/requestJoypadInterrupt
    fn func147<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func100(imports, 4i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/joypad/index/_pressJoypadButton
    fn func148<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        self.global54 = 0i32;
        let var4 = var0;
        let var5 = self.func145(imports, var4); // wasm/joypad/index/_getJoypadButtonStateFromButtonId
        if (var5 == 0) as i32 != 0 {
            var1 = 1i32;
        }
        let var6 = var0;
        self.func146(imports, var6, 1i32); // wasm/joypad/index/_setJoypadButtonStateFromButtonId
        let var7 = var1;
        if var7 != 0 {
            var1 = 0i32;
            let var8 = var0;
            if (var8 as u32 <= 3i32 as u32) as i32 != 0 {
                var1 = 1i32;
            }
            var0 = 0i32;
            let var9 = self.func13(imports, 65280i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            var3 = var9;
            let var10 = var3;
            let var11 = self.func2(imports, 4i32, var10); // wasm/helpers/index/checkBitOnByte
            var2 = var11;
            let var12 = var2;
            let var13: i32;
            if var12 != 0 {
                let var14 = var1;
                var13 = var14;
            } else {
                let var15 = var2;
                var13 = var15;
            }
            if var13 & 1i32 != 0 {
                var0 = 1i32;
            }
            let var16 = var3;
            let var17 = self.func2(imports, 5i32, var16); // wasm/helpers/index/checkBitOnByte
            var2 = var17;
            let var18 = var2;
            let var19: i32;
            if var18 != 0 {
                let var20 = var1;
                var19 = (var20 == 0) as i32;
            } else {
                let var21 = var2;
                var19 = var21;
            }
            if var19 & 1i32 != 0 {
                var0 = 1i32;
            }
            let var22 = var0;
            if var22 != 0 {
                self.func147(imports); // wasm/interrupts/index/requestJoypadInterrupt
            }
        }
    }
    // wasm/joypad/index/_releaseJoypadButton
    fn func149<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.func146(imports, var1, 0i32); // wasm/joypad/index/_setJoypadButtonStateFromButtonId
    }
    // wasm/joypad/index/setJoypadState
    fn func150<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32) {
        let var8 = var0;
        if (var8 > 0i32) as i32 != 0 {
            self.func148(imports, 0i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 0i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var9 = var1;
        if (var9 > 0i32) as i32 != 0 {
            self.func148(imports, 1i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 1i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var10 = var2;
        if (var10 > 0i32) as i32 != 0 {
            self.func148(imports, 2i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 2i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var11 = var3;
        if (var11 > 0i32) as i32 != 0 {
            self.func148(imports, 3i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 3i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var12 = var4;
        if (var12 > 0i32) as i32 != 0 {
            self.func148(imports, 4i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 4i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var13 = var5;
        if (var13 > 0i32) as i32 != 0 {
            self.func148(imports, 5i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 5i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var14 = var6;
        if (var14 > 0i32) as i32 != 0 {
            self.func148(imports, 6i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 6i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var15 = var7;
        if (var15 > 0i32) as i32 != 0 {
            self.func148(imports, 7i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func149(imports, 7i32); // wasm/joypad/index/_releaseJoypadButton
        }
    }
    // wasm/sound/sound/getAudioQueueIndex
    fn func151<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global52;
        var0
    }
    // wasm/memory/memory/getSaveStateMemoryOffset
    fn func152<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = var1;
        var2.wrapping_add(var3.wrapping_mul(50i32) & 65535i32) & 65535i32
    }
    // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    fn func153<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var1;
        if var2 != 0 {
            let var3 = var0;
            self.memory.store8(var3 as usize, 1i32 as u8);
        } else {
            let var4 = var0;
            self.memory.store8(var4 as usize, 0i32 as u8);
        }
    }
    // wasm/cpu/index/Cpu.saveState
    fn func154<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global1;
        self.memory.store8(var0 as usize, var1 as u8);
        let var2 = self.func152(imports, 1i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global3;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.func152(imports, 2i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global4;
        self.memory.store8(var4 as usize, var5 as u8);
        let var6 = self.func152(imports, 3i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global5;
        self.memory.store8(var6 as usize, var7 as u8);
        let var8 = self.func152(imports, 4i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global6;
        self.memory.store8(var8 as usize, var9 as u8);
        let var10 = self.func152(imports, 5i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global7;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func152(imports, 6i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global8;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.func152(imports, 7i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global2;
        self.memory.store8(var14 as usize, var15 as u8);
        let var16 = self.func152(imports, 8i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global9;
        self.memory.store16(var16 as usize, var17 as u16);
        let var18 = self.func152(imports, 10i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.global0;
        self.memory.store16(var18 as usize, var19 as u16);
        let var20 = self.func152(imports, 12i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var21 = self.global51;
        self.memory.store32(var20 as usize, var21 as u32);
        let var22 = self.func152(imports, 17i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var23 = self.global53;
        self.func153(imports, var22, var23); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var24 = self.func152(imports, 18i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var25 = self.global54;
        self.func153(imports, var24, var25); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/graphics/graphics/Graphics.saveState
    fn func155<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global60;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func152(imports, 4i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global19;
        self.memory.store8(var2 as usize, var3 as u8);
    }
    // wasm/interrupts/index/Interrupts.saveState
    fn func156<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global55;
        self.func153(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func152(imports, 1i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global66;
        self.func153(imports, var2, var3); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/joypad/index/Joypad.saveState
    fn func157<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    // wasm/memory/memory/Memory.saveState
    fn func158<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global14;
        self.memory.store16(var0 as usize, var1 as u16);
        let var2 = self.func152(imports, 2i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global18;
        self.memory.store16(var2 as usize, var3 as u16);
        let var4 = self.func152(imports, 4i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global12;
        self.func153(imports, var4, var5); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var6 = self.func152(imports, 5i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global17;
        self.func153(imports, var6, var7); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var8 = self.func152(imports, 6i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global10;
        self.func153(imports, var8, var9); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var10 = self.func152(imports, 7i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global15;
        self.func153(imports, var10, var11); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var12 = self.func152(imports, 8i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global11;
        self.func153(imports, var12, var13); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var14 = self.func152(imports, 9i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global16;
        self.func153(imports, var14, var15); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var16 = self.func152(imports, 10i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global13;
        self.func153(imports, var16, var17); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/timers/index/Timers.saveState
    fn func159<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global58;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func152(imports, 4i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global59;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func152(imports, 8i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global57;
        self.memory.store32(var4 as usize, var5 as u32);
    }
    // wasm/sound/sound/Sound.saveState
    fn func160<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global61;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func152(imports, 4i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global65;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.func152(imports, 5i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global62;
        self.memory.store8(var4 as usize, var5 as u8);
    }
    // wasm/sound/channel1/Channel1.saveState
    fn func161<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global32;
        self.func153(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func152(imports, 1i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global33;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func152(imports, 5i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global34;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func152(imports, 9i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global28;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func152(imports, 14i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global35;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func152(imports, 19i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global67;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func152(imports, 20i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global63;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.func152(imports, 25i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global38;
        self.func153(imports, var14, var15); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var16 = self.func152(imports, 26i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global37;
        self.memory.store32(var16 as usize, var17 as u32);
        let var18 = self.func152(imports, 31i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.global36;
        self.memory.store16(var18 as usize, var19 as u16);
    }
    // wasm/sound/channel2/Channel2.saveState
    fn func162<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global39;
        self.func153(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func152(imports, 1i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global40;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func152(imports, 5i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global41;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func152(imports, 9i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global29;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func152(imports, 14i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global42;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func152(imports, 19i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global68;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func152(imports, 20i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global64;
        self.memory.store8(var12 as usize, var13 as u8);
    }
    // wasm/sound/channel3/Channel3.saveState
    fn func163<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global43;
        self.func153(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func152(imports, 1i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global44;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func152(imports, 5i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global30;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func152(imports, 9i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global45;
        self.memory.store16(var6 as usize, var7 as u16);
    }
    // wasm/sound/channel4/Channel4.saveState
    fn func164<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global46;
        self.func153(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func152(imports, 1i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global47;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func152(imports, 5i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global48;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func152(imports, 9i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global31;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func152(imports, 14i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global49;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func152(imports, 19i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global50;
        self.memory.store16(var10 as usize, var11 as u16);
    }
    // wasm/index/saveState
    fn func165<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func154(imports); // wasm/cpu/index/Cpu.saveState
        self.func155(imports); // wasm/graphics/graphics/Graphics.saveState
        self.func156(imports); // wasm/interrupts/index/Interrupts.saveState
        self.func157(imports); // wasm/joypad/index/Joypad.saveState
        self.func158(imports); // wasm/memory/memory/Memory.saveState
        self.func159(imports); // wasm/timers/index/Timers.saveState
        self.func160(imports); // wasm/sound/sound/Sound.saveState
        self.func161(imports); // wasm/sound/channel1/Channel1.saveState
        self.func162(imports); // wasm/sound/channel2/Channel2.saveState
        self.func163(imports); // wasm/sound/channel3/Channel3.saveState
        self.func164(imports); // wasm/sound/channel4/Channel4.saveState
    }
    // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
    fn func166<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.memory.load8(var1 as usize) as i32;
        if (var2 as u32 > 0i32 as u32) as i32 != 0 {
            return 1i32;
        }
        0i32
    }
    // wasm/cpu/index/Cpu.loadState
    fn func167<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load8(var0 as usize) as i32;
        self.global1 = var1;
        let var2 = self.func152(imports, 1i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global3 = var3;
        let var4 = self.func152(imports, 2i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global4 = var5;
        let var6 = self.func152(imports, 3i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load8(var6 as usize) as i32;
        self.global5 = var7;
        let var8 = self.func152(imports, 4i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load8(var8 as usize) as i32;
        self.global6 = var9;
        let var10 = self.func152(imports, 5i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global7 = var11;
        let var12 = self.func152(imports, 6i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global8 = var13;
        let var14 = self.func152(imports, 7i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.memory.load8(var14 as usize) as i32;
        self.global2 = var15;
        let var16 = self.func152(imports, 8i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.memory.load16(var16 as usize) as i32;
        self.global9 = var17;
        let var18 = self.func152(imports, 10i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global0 = var19;
        let var20 = self.func152(imports, 12i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var21 = self.memory.load32(var20 as usize) as i32;
        self.global51 = var21;
        let var22 = self.func152(imports, 17i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var23 = self.func166(imports, var22); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global53 = var23;
        let var24 = self.func152(imports, 18i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var25 = self.func166(imports, var24); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global54 = var25;
    }
    // wasm/graphics/graphics/Graphics.loadState
    fn func168<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global60 = var1;
        let var2 = self.func152(imports, 4i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global19 = var3;
    }
    // wasm/interrupts/index/Interrupts.loadState
    fn func169<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func166(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global55 = var1;
        let var2 = self.func152(imports, 1i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.func166(imports, var2); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global66 = var3;
    }
    // wasm/memory/memory/Memory.loadState
    fn func170<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load16(var0 as usize) as i32;
        self.global14 = var1;
        let var2 = self.func152(imports, 2i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load16(var2 as usize) as i32;
        self.global18 = var3;
        let var4 = self.func152(imports, 4i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.func166(imports, var4); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global12 = var5;
        let var6 = self.func152(imports, 5i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.func166(imports, var6); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global17 = var7;
        let var8 = self.func152(imports, 6i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.func166(imports, var8); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global10 = var9;
        let var10 = self.func152(imports, 7i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.func166(imports, var10); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global15 = var11;
        let var12 = self.func152(imports, 8i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.func166(imports, var12); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global11 = var13;
        let var14 = self.func152(imports, 9i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.func166(imports, var14); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global16 = var15;
        let var16 = self.func152(imports, 10i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.func166(imports, var16); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global13 = var17;
    }
    // wasm/timers/index/Timers.loadState
    fn func171<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global58 = var1;
        let var2 = self.func152(imports, 4i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global59 = var3;
        let var4 = self.func152(imports, 8i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global57 = var5;
    }
    // wasm/sound/sound/Sound.loadState
    fn func172<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global61 = var1;
        let var2 = self.func152(imports, 4i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global65 = var3;
        let var4 = self.func152(imports, 5i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global62 = var5;
        self.func141(imports); // wasm/sound/sound/resetAudioQueue
    }
    // wasm/sound/channel1/Channel1.loadState
    fn func173<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func166(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global32 = var1;
        let var2 = self.func152(imports, 1i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global33 = var3;
        let var4 = self.func152(imports, 5i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global34 = var5;
        let var6 = self.func152(imports, 9i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global28 = var7;
        let var8 = self.func152(imports, 14i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global35 = var9;
        let var10 = self.func152(imports, 19i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global67 = var11;
        let var12 = self.func152(imports, 20i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global63 = var13;
        let var14 = self.func152(imports, 25i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.func166(imports, var14); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global38 = var15;
        let var16 = self.func152(imports, 26i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.memory.load32(var16 as usize) as i32;
        self.global37 = var17;
        let var18 = self.func152(imports, 31i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global36 = var19;
    }
    // wasm/sound/channel2/Channel2.loadState
    fn func174<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func166(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global39 = var1;
        let var2 = self.func152(imports, 1i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global40 = var3;
        let var4 = self.func152(imports, 5i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global41 = var5;
        let var6 = self.func152(imports, 9i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global29 = var7;
        let var8 = self.func152(imports, 14i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global42 = var9;
        let var10 = self.func152(imports, 19i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global68 = var11;
        let var12 = self.func152(imports, 20i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global64 = var13;
    }
    // wasm/sound/channel3/Channel3.loadState
    fn func175<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func166(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global43 = var1;
        let var2 = self.func152(imports, 1i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global44 = var3;
        let var4 = self.func152(imports, 5i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global30 = var5;
        let var6 = self.func152(imports, 9i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load16(var6 as usize) as i32;
        self.global45 = var7;
    }
    // wasm/sound/channel4/Channel4.loadState
    fn func176<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func152(imports, 0i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func166(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global46 = var1;
        let var2 = self.func152(imports, 1i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global47 = var3;
        let var4 = self.func152(imports, 5i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global48 = var5;
        let var6 = self.func152(imports, 9i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global31 = var7;
        let var8 = self.func152(imports, 14i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global49 = var9;
        let var10 = self.func152(imports, 19i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load16(var10 as usize) as i32;
        self.global50 = var11;
    }
    // wasm/index/loadState
    fn func177<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func167(imports); // wasm/cpu/index/Cpu.loadState
        self.func168(imports); // wasm/graphics/graphics/Graphics.loadState
        self.func169(imports); // wasm/interrupts/index/Interrupts.loadState
        self.func157(imports); // wasm/joypad/index/Joypad.saveState
        self.func170(imports); // wasm/memory/memory/Memory.loadState
        self.func171(imports); // wasm/timers/index/Timers.loadState
        self.func172(imports); // wasm/sound/sound/Sound.loadState
        self.func173(imports); // wasm/sound/channel1/Channel1.loadState
        self.func174(imports); // wasm/sound/channel2/Channel2.loadState
        self.func175(imports); // wasm/sound/channel3/Channel3.loadState
        self.func176(imports); // wasm/sound/channel4/Channel4.loadState
    }
    // wasm/index/getRegisterA
    fn func178<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global1;
        var0
    }
    // wasm/index/getRegisterB
    fn func179<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global3;
        var0
    }
    // wasm/index/getRegisterC
    fn func180<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global4;
        var0
    }
    // wasm/index/getRegisterD
    fn func181<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global5;
        var0
    }
    // wasm/index/getRegisterE
    fn func182<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global6;
        var0
    }
    // wasm/index/getRegisterH
    fn func183<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global7;
        var0
    }
    // wasm/index/getRegisterL
    fn func184<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global8;
        var0
    }
    // wasm/index/getRegisterF
    fn func185<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        var0
    }
    // wasm/index/getProgramCounter
    fn func186<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global0;
        var0
    }
    // wasm/index/getStackPointer
    fn func187<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global9;
        var0
    }
    // wasm/index/getPreviousOpcode
    fn func188<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global56;
        var0
    }
    // wasm/index/getOpcodeAtProgramCounter
    fn func189<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global0;
        let var1 = self.func18(imports, var0); // wasm/memory/load/eightBitLoadFromGBMemory
        var1
    }
}

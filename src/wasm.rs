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
}

impl<I: Imports<Memory = M>, M: Memory> Instance<I, M> {
    pub fn new(imports: I, mut memory: M) -> Self {
        let current_pages = memory.size() as usize;
        if current_pages < 136 {
            memory.grow(136 - current_pages);
            assert_eq!(memory.size(), 136, "Not enough memory pages allocated");
        }
        memory.store_slice(8861696, b" \0\0\0i\0n\0i\0t\0i\0a\0l\0i\0z\0i\0n\0g\0 \0(\0i\0n\0c\0l\0u\0d\0e\0B\0o\0o\0t\0R\0o\0m\0=\0$\00\0)");
        memory.store_slice(8861764, b"\0\0\0O\0p\0c\0o\0d\0e\0 \0a\0t\0 \0c\0r\0a\0s\0h\0:\0 \0$\00");
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
                global20: 255,
                global21: 1024,
                global22: 0,
                global23: 0,
                global24: 0,
                global25: 0,
                global26: 87,
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
                global56: 131072,
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
                global74: 456,
                global75: 0,
                global76: 0,
                global77: 0,
                global78: 0,
            },
        };
        instance
    }
    pub fn initialize(&mut self, var0: i32) {
        self.context.func82(&mut self.imports, var0)
    }
    pub fn config(&mut self, var0: i32, var1: i32, var2: i32) {
        self.context.func83(&mut self.imports, var0, var1, var2)
    }
    pub fn update(&mut self) -> i32 {
        self.context.func162(&mut self.imports)
    }
    pub fn emulationStep(&mut self, var0: i32, var1: i32, var2: i32) -> i32 {
        self.context.func160(&mut self.imports, var0, var1, var2)
    }
    pub fn areInterruptsEnabled(&mut self) -> i32 {
        self.context.func141(&mut self.imports)
    }
    pub fn setJoypadState(&mut self, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.context.func168(&mut self.imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex(&mut self) -> i32 {
        self.context.func169(&mut self.imports)
    }
    pub fn resetAudioQueue(&mut self) {
        self.context.func170(&mut self.imports)
    }
    pub fn saveState(&mut self) {
        self.context.func184(&mut self.imports)
    }
    pub fn loadState(&mut self) {
        self.context.func196(&mut self.imports)
    }
    pub fn getRegisterA(&mut self) -> i32 {
        self.context.func197(&mut self.imports)
    }
    pub fn getRegisterB(&mut self) -> i32 {
        self.context.func198(&mut self.imports)
    }
    pub fn getRegisterC(&mut self) -> i32 {
        self.context.func199(&mut self.imports)
    }
    pub fn getRegisterD(&mut self) -> i32 {
        self.context.func200(&mut self.imports)
    }
    pub fn getRegisterE(&mut self) -> i32 {
        self.context.func201(&mut self.imports)
    }
    pub fn getRegisterH(&mut self) -> i32 {
        self.context.func202(&mut self.imports)
    }
    pub fn getRegisterL(&mut self) -> i32 {
        self.context.func203(&mut self.imports)
    }
    pub fn getRegisterF(&mut self) -> i32 {
        self.context.func204(&mut self.imports)
    }
    pub fn getProgramCounter(&mut self) -> i32 {
        self.context.func205(&mut self.imports)
    }
    pub fn getStackPointer(&mut self) -> i32 {
        self.context.func206(&mut self.imports)
    }
    pub fn getPreviousOpcode(&mut self) -> i32 {
        self.context.func207(&mut self.imports)
    }
    pub fn getOpcodeAtProgramCounter(&mut self) -> i32 {
        self.context.func208(&mut self.imports)
    }
}

impl<M: Memory> Context<M> {
    pub fn initialize<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32) {
        self.func82(imports, var0)
    }
    pub fn config<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32) {
        self.func83(imports, var0, var1, var2)
    }
    pub fn update<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func162(imports)
    }
    pub fn emulationStep<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32) -> i32 {
        self.func160(imports, var0, var1, var2)
    }
    pub fn areInterruptsEnabled<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func141(imports)
    }
    pub fn setJoypadState<I: Imports<Memory = M>>(&mut self, imports: &mut I, var0: i32, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: i32) {
        self.func168(imports, var0, var1, var2, var3, var4, var5, var6, var7)
    }
    pub fn getAudioQueueIndex<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func169(imports)
    }
    pub fn resetAudioQueue<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func170(imports)
    }
    pub fn saveState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func184(imports)
    }
    pub fn loadState<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func196(imports)
    }
    pub fn getRegisterA<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func197(imports)
    }
    pub fn getRegisterB<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func198(imports)
    }
    pub fn getRegisterC<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func199(imports)
    }
    pub fn getRegisterD<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func200(imports)
    }
    pub fn getRegisterE<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func201(imports)
    }
    pub fn getRegisterH<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func202(imports)
    }
    pub fn getRegisterL<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func203(imports)
    }
    pub fn getRegisterF<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func204(imports)
    }
    pub fn getProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func205(imports)
    }
    pub fn getStackPointer<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func206(imports)
    }
    pub fn getPreviousOpcode<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func207(imports)
    }
    pub fn getOpcodeAtProgramCounter<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        self.func208(imports)
    }
    // wasm/helpers/index/log
    fn func1<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32) {
        let var7 = var0;
        let var8 = var1;
        let var9 = var2;
        let var10 = var3;
        let var11 = var4;
        let var12 = var5;
        let var13 = var6;
        imports.log(self, var7, var8, var9, var10, var11, var12, var13);
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
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var1 = var0;
                                match var1.wrapping_shr(12i32 as u32) {
                                    0 => break 'label4,
                                    1 => break 'label4,
                                    2 => break 'label4,
                                    3 => break 'label4,
                                    4 => break 'label3,
                                    5 => break 'label3,
                                    6 => break 'label3,
                                    7 => break 'label3,
                                    8 => break 'label2,
                                    9 => break 'label2,
                                    10 => break 'label1,
                                    11 => break 'label1,
                                    _ => break 'label5,
                                }
                                break;
                            }
                            break 'label0;
                            break;
                        }
                        let var2 = var0;
                        return var2.wrapping_add(473088i32);
                        break;
                    }
                    let var3 = var0;
                    let var4 = self.func4(imports, var3); // wasm/memory/banking/getRomBankAddress
                    return var4.wrapping_add(473088i32);
                    break;
                }
                let var5 = var0;
                return var5.wrapping_add(-31744i32);
                break;
            }
            let var6 = var0;
            let var7 = self.func5(imports, var6); // wasm/memory/banking/getRamBankAddress
            return var7.wrapping_add(33792i32);
            break;
        }
        let var8 = var0;
        var8.wrapping_add(-31744i32)
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
    // wasm/timers/index/_isTimerEnabled
    fn func14<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65287i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        ((var0 & 4i32) as u32 > 0i32 as u32) as i32
    }
    // wasm/timers/index/_checkDividerRegister
    fn func15<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global23;
        let var2 = var0;
        self.global23 = var1.wrapping_add(var2);
        let var3 = self.global23;
        if (var3 >= 255i32) as i32 != 0 {
            let var4 = self.global23;
            self.global23 = var4.wrapping_sub(255i32);
            let var5 = self.func13(imports, 65284i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            self.func8(imports, 65284i32, var5.wrapping_add(1i32) & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        }
    }
    // wasm/timers/index/_getCurrentCycleCounterFrequency
    fn func16<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        var0 = 256i32;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var1 = self.func13(imports, 65287i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                            match var1 & 3i32 {
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
                    break 'label0;
                    break;
                }
                var0 = 16i32;
                break 'label0;
                break;
            }
            var0 = 64i32;
            break;
        }
        let var2 = var0;
        let var3 = self.global21;
        if (var2 != var3) as i32 != 0 {
            self.global24 = 0i32;
            let var4 = var0;
            self.global21 = var4;
        }
        let var5 = var0;
        var5
    }
    // wasm/helpers/index/setBitOnByte
    fn func17<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        (var2 | 1i32.wrapping_shl(var3 as u32) & 255i32) & 255i32
    }
    // wasm/interrupts/index/_requestInterrupt
    fn func18<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func13(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var3 = self.func17(imports, var1, var2); // wasm/helpers/index/setBitOnByte
        self.func8(imports, 65295i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/interrupts/index/requestTimerInterrupt
    fn func19<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func18(imports, 2i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/timers/index/updateTimers
    fn func20<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = var0;
        self.func15(imports, var2); // wasm/timers/index/_checkDividerRegister
        let var3 = self.func14(imports); // wasm/timers/index/_isTimerEnabled
        if var3 != 0 {
            let var4 = self.global24;
            let var5 = var0;
            self.global24 = var4.wrapping_add(var5);
            let var6 = self.global24;
            let var7 = self.func16(imports); // wasm/timers/index/_getCurrentCycleCounterFrequency
            if (var6 > var7) as i32 != 0 {
                let var8 = self.global24;
                let var9 = self.func16(imports); // wasm/timers/index/_getCurrentCycleCounterFrequency
                self.global24 = var8.wrapping_sub(var9);
                let var10 = self.func13(imports, 65285i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var1 = var10;
                let var11 = var1;
                if (var11 == 255i32) as i32 != 0 {
                    let var12 = self.func13(imports, 65286i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    self.func8(imports, 65285i32, var12); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func19(imports); // wasm/interrupts/index/requestTimerInterrupt
                } else {
                    let var13 = var1;
                    self.func8(imports, 65285i32, var13.wrapping_add(1i32) & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                }
            }
        }
    }
    // wasm/timers/index/batchProcessTimers
    fn func21<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.global20;
        var0 = var2;
        let var3 = self.func14(imports); // wasm/timers/index/_isTimerEnabled
        var1 = var3;
        let var4 = var1;
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global21;
            let var7 = var0;
            var5 = ((var6) < var7) as i32;
        } else {
            let var8 = var1;
            var5 = var8;
        }
        if var5 & 1i32 != 0 {
            let var9 = self.global21;
            var0 = var9;
        }
        let var10 = self.global22;
        let var11 = var0;
        if ((var10) < var11) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var12 = self.global22;
            let var13 = var0;
            if (var12 >= var13) as i32 != 0 {
                let var14 = var0;
                self.func20(imports, var14); // wasm/timers/index/updateTimers
                let var15 = self.global22;
                let var16 = var0;
                self.global22 = var15.wrapping_sub(var16);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/sound/registers/getRegister4OfChannel
    fn func22<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var1 = var0;
                            match var1.wrapping_sub(1i32) {
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
                    let var2 = self.func13(imports, 65300i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    return var2;
                    break;
                }
                let var3 = self.func13(imports, 65305i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                return var3;
                break;
            }
            let var4 = self.func13(imports, 65310i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            return var4;
            break;
        }
        let var5 = self.func13(imports, 65315i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5
    }
    // wasm/sound/length/isChannelLengthEnabled
    fn func23<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func22(imports, var1); // wasm/sound/registers/getRegister4OfChannel
        let var3 = self.func2(imports, 6i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/channel1/Channel1.updateLength
    fn func24<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global29;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func23(imports, 1i32); // wasm/sound/length/isChannelLengthEnabled
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
            self.global30 = 0i32;
        }
    }
    // wasm/sound/channel2/Channel2.updateLength
    fn func25<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global31;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func23(imports, 2i32); // wasm/sound/length/isChannelLengthEnabled
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
            self.global32 = 0i32;
        }
    }
    // wasm/sound/channel3/Channel3.updateLength
    fn func26<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global33;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func23(imports, 3i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global33;
            self.global33 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global33;
        if (var7 == 0) as i32 != 0 {
            self.global34 = 0i32;
        }
    }
    // wasm/sound/channel4/Channel4.updateLength
    fn func27<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global35;
        var0 = (var1 > 0i32) as i32;
        let var2 = var0;
        let var3: i32;
        if var2 != 0 {
            let var4 = self.func23(imports, 4i32); // wasm/sound/length/isChannelLengthEnabled
            var3 = var4;
        } else {
            let var5 = var0;
            var3 = var5;
        }
        if var3 & 1i32 != 0 {
            let var6 = self.global35;
            self.global35 = var6.wrapping_sub(1i32);
        }
        let var7 = self.global35;
        if (var7 == 0) as i32 != 0 {
            self.global36 = 0i32;
        }
    }
    // wasm/sound/channel1/getSweepPeriod
    fn func28<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        ((var0 & 112i32) as u32).wrapping_shr(4i32 as u32) as i32
    }
    // wasm/sound/channel1/getSweepShift
    fn func29<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var0 & 7i32
    }
    // wasm/sound/channel1/getNewFrequencyFromSweep
    fn func30<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let var1 = self.global39;
        let var2 = self.func29(imports); // wasm/sound/channel1/getSweepShift
        var0 = (var1 as u32).wrapping_shr(var2 as u32) as i32;
        let var3 = self.func13(imports, 65296i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var4 = self.func2(imports, 3i32, var3); // wasm/helpers/index/checkBitOnByte
        let var5: i32;
        if var4 != 0 {
            let var6 = self.global39;
            let var7 = var0;
            var5 = var6.wrapping_sub(var7) & 65535i32;
        } else {
            let var8 = self.global39;
            let var9 = var0;
            var5 = var8.wrapping_add(var9) & 65535i32;
        }
        var0 = var5;
        let var10 = var0;
        var10
    }
    // wasm/sound/registers/setRegister3OfChannel
    fn func31<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var2 = var0;
                                match var2.wrapping_sub(1i32) {
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
                        let var3 = var1;
                        self.func8(imports, 65299i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        break 'label0;
                        break;
                    }
                    let var4 = var1;
                    self.func8(imports, 65304i32, var4); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    break 'label0;
                    break;
                }
                let var5 = var1;
                self.func8(imports, 65309i32, var5); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                break 'label0;
                break;
            }
            let var6 = var1;
            self.func8(imports, 65314i32, var6); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            break;
        }
    }
    // wasm/sound/registers/setRegister4OfChannel
    fn func32<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var2 = var0;
                                match var2.wrapping_sub(1i32) {
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
                        let var3 = var1;
                        self.func8(imports, 65300i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        break 'label0;
                        break;
                    }
                    let var4 = var1;
                    self.func8(imports, 65305i32, var4); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    break 'label0;
                    break;
                }
                let var5 = var1;
                self.func8(imports, 65310i32, var5); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                break 'label0;
                break;
            }
            let var6 = var1;
            self.func8(imports, 65315i32, var6); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            break;
        }
    }
    // wasm/sound/frequency/setChannelFrequency
    fn func33<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func22(imports, var3); // wasm/sound/registers/getRegister4OfChannel
        let var5 = var1;
        var2 = var4 & 248i32 | (var5 as u32).wrapping_shr(8i32 as u32) as i32 & 255i32;
        let var6 = var0;
        let var7 = var1;
        self.func31(imports, var6, var7 & 255i32); // wasm/sound/registers/setRegister3OfChannel
        let var8 = var0;
        let var9 = var2;
        self.func32(imports, var8, var9); // wasm/sound/registers/setRegister4OfChannel
    }
    // wasm/sound/channel1/calculateSweepAndCheckOverflow
    fn func34<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func30(imports); // wasm/sound/channel1/getNewFrequencyFromSweep
        var0 = var2;
        let var3 = var0;
        var1 = (var3 as u32 <= 2047i32 as u32) as i32;
        let var4 = var1;
        let var5: i32;
        if var4 != 0 {
            let var6 = self.func29(imports); // wasm/sound/channel1/getSweepShift
            var5 = (var6 as u32 > 0i32 as u32) as i32;
        } else {
            let var7 = var1;
            var5 = var7;
        }
        if var5 & 1i32 != 0 {
            let var8 = var0;
            self.global39 = var8;
            let var9 = var0;
            self.func33(imports, 1i32, var9); // wasm/sound/frequency/setChannelFrequency
            let var10 = self.func30(imports); // wasm/sound/channel1/getNewFrequencyFromSweep
            var0 = var10;
        }
        let var11 = var0;
        if (var11 as u32 > 2047i32 as u32) as i32 != 0 {
            self.global30 = 0i32;
        }
    }
    // wasm/sound/channel1/Channel1.updateSweep
    fn func35<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global37;
        self.global37 = var0.wrapping_sub(1i32);
        let var1 = self.global37;
        if (var1 <= 0i32) as i32 != 0 {
            let var2 = self.func28(imports); // wasm/sound/channel1/getSweepPeriod
            self.global37 = var2;
            let var3 = self.global38;
            let var4: i32;
            if var3 != 0 {
                let var5 = self.func28(imports); // wasm/sound/channel1/getSweepPeriod
                var4 = (var5 as u32 > 0i32 as u32) as i32;
            } else {
                let var6 = self.global38;
                var4 = var6;
            }
            if var4 & 1i32 != 0 {
                self.func34(imports); // wasm/sound/channel1/calculateSweepAndCheckOverflow
            }
        }
    }
    // wasm/sound/registers/getRegister2OfChannel
    fn func36<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var1 = var0;
                            match var1.wrapping_sub(1i32) {
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
                    let var2 = self.func13(imports, 65298i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    return var2;
                    break;
                }
                let var3 = self.func13(imports, 65303i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                return var3;
                break;
            }
            let var4 = self.func13(imports, 65308i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            return var4;
            break;
        }
        let var5 = self.func13(imports, 65313i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5
    }
    // wasm/sound/envelope/getChannelEnvelopePeriod
    fn func37<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func36(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        var2 & 7i32
    }
    // wasm/sound/envelope/getChannelEnvelopeAddMode
    fn func38<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func36(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        let var3 = self.func2(imports, 3i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/channel1/Channel1.updateEnvelope
    fn func39<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global40;
        self.global40 = var1.wrapping_sub(1i32);
        let var2 = self.global40;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func37(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global40 = var3;
            let var4 = self.global40;
            if var4 != 0 {
                let var5 = self.func38(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global41;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global41;
                    self.global41 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func38(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global41;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global41;
                        self.global41 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/channel2/Channel2.updateEnvelope
    fn func40<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global42;
        self.global42 = var1.wrapping_sub(1i32);
        let var2 = self.global42;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func37(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global42 = var3;
            let var4 = self.global42;
            if var4 != 0 {
                let var5 = self.func38(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global43;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global43;
                    self.global43 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func38(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global43;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global43;
                        self.global43 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/channel4/Channel4.updateEnvelope
    fn func41<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let var1 = self.global44;
        self.global44 = var1.wrapping_sub(1i32);
        let var2 = self.global44;
        if (var2 <= 0i32) as i32 != 0 {
            let var3 = self.func37(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopePeriod
            self.global44 = var3;
            let var4 = self.global44;
            if var4 != 0 {
                let var5 = self.func38(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                var0 = var5;
                let var6 = var0;
                let var7: i32;
                if var6 != 0 {
                    let var8 = self.global45;
                    var7 = ((var8) < 15i32) as i32;
                } else {
                    let var9 = var0;
                    var7 = var9;
                }
                if var7 & 1i32 != 0 {
                    let var10 = self.global45;
                    self.global45 = var10.wrapping_add(1i32);
                } else {
                    let var11 = self.func38(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopeAddMode
                    var0 = (var11 == 0) as i32;
                    let var12 = var0;
                    let var13: i32;
                    if var12 != 0 {
                        let var14 = self.global45;
                        var13 = (var14 > 0i32) as i32;
                    } else {
                        let var15 = var0;
                        var13 = var15;
                    }
                    if var13 & 1i32 != 0 {
                        let var16 = self.global45;
                        self.global45 = var16.wrapping_sub(1i32);
                    }
                }
            }
        }
    }
    // wasm/sound/registers/getRegister3OfChannel
    fn func42<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var1 = var0;
                            match var1.wrapping_sub(1i32) {
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
                    let var2 = self.func13(imports, 65299i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    return var2;
                    break;
                }
                let var3 = self.func13(imports, 65304i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                return var3;
                break;
            }
            let var4 = self.func13(imports, 65309i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            return var4;
            break;
        }
        let var5 = self.func13(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5
    }
    // wasm/sound/frequency/getChannelFrequency
    fn func43<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func22(imports, var1); // wasm/sound/registers/getRegister4OfChannel
        let var3 = var0;
        let var4 = self.func42(imports, var3); // wasm/sound/registers/getRegister3OfChannel
        ((var2 & 7i32).wrapping_shl(8i32 as u32) | var4) & 65535i32
    }
    // wasm/sound/registers/isChannelDacEnabled
    fn func44<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2: i32;
        if (var1 != 3i32) as i32 != 0 {
            let var3 = var0;
            let var4 = self.func36(imports, var3); // wasm/sound/registers/getRegister2OfChannel
            let var5: i32;
            if ((var4 & 248i32) as u32 > 0i32 as u32) as i32 != 0 {
                var5 = 1i32;
            } else {
                var5 = 0i32;
            }
            var2 = var5;
        } else {
            let var6 = self.func13(imports, 65306i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            let var7 = self.func2(imports, 7i32, var6); // wasm/helpers/index/checkBitOnByte
            var2 = var7;
        }
        var2
    }
    // wasm/sound/registers/getRegister1OfChannel
    fn func45<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var1 = var0;
                            match var1.wrapping_sub(1i32) {
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
                    let var2 = self.func13(imports, 65297i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                    return var2;
                    break;
                }
                let var3 = self.func13(imports, 65302i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                return var3;
                break;
            }
            let var4 = self.func13(imports, 65307i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            return var4;
            break;
        }
        let var5 = self.func13(imports, 65312i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var5
    }
    // wasm/sound/duty/getChannelDuty
    fn func46<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func45(imports, var1); // wasm/sound/registers/getRegister1OfChannel
        (var2 as u32).wrapping_shr(6i32 as u32) as i32 & 3i32
    }
    // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
    fn func47<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = self.func46(imports, var2); // wasm/sound/duty/getChannelDuty
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            let var4 = var0;
                            let var5 = self.func46(imports, var4); // wasm/sound/duty/getChannelDuty
                            match var5.wrapping_sub(1i32) {
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
                    let var6 = var1;
                    let var7 = self.func2(imports, var6, 129i32); // wasm/helpers/index/checkBitOnByte
                    return var7;
                    break;
                }
                let var8 = var1;
                let var9 = self.func2(imports, var8, 135i32); // wasm/helpers/index/checkBitOnByte
                return var9;
                break;
            }
            let var10 = var1;
            let var11 = self.func2(imports, var10, 126i32); // wasm/helpers/index/checkBitOnByte
            return var11;
            break;
        }
        let var12 = var1;
        let var13 = self.func2(imports, var12, 1i32); // wasm/helpers/index/checkBitOnByte
        var13
    }
    // wasm/sound/channel1/Channel1.getSample
    fn func48<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global46;
        let var3 = var0;
        self.global46 = var2.wrapping_sub(var3);
        let var4 = self.global46;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global46;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func43(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
            self.global46 = 2048i32.wrapping_sub(var9).wrapping_mul(4i32);
            let var10 = self.global46;
            let var11 = var0;
            self.global46 = var10.wrapping_sub(var11);
            let var12 = self.global47;
            self.global47 = var12.wrapping_add(1i32) & 255i32;
            let var13 = self.global47;
            if (var13 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global47 = 0i32;
            }
        }
        let var14 = self.global30;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.func44(imports, 1i32); // wasm/sound/registers/isChannelDacEnabled
            var15 = var16;
        } else {
            let var17 = self.global30;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global41;
            var0 = var18;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var19 = self.global47;
        let var20 = self.func47(imports, 1i32, var19); // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
        if (var20 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        let var21 = var1;
        let var22 = var0;
        var21.wrapping_mul(var22).wrapping_add(15i32)
    }
    // wasm/sound/channel2/Channel2.getSample
    fn func49<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global48;
        let var3 = var0;
        self.global48 = var2.wrapping_sub(var3);
        let var4 = self.global48;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global48;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func43(imports, 2i32); // wasm/sound/frequency/getChannelFrequency
            self.global48 = 2048i32.wrapping_sub(var9).wrapping_mul(4i32);
            let var10 = self.global48;
            let var11 = var0;
            self.global48 = var10.wrapping_sub(var11);
            let var12 = self.global49;
            self.global49 = var12.wrapping_add(1i32) & 255i32;
            let var13 = self.global49;
            if (var13 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global49 = 0i32;
            }
        }
        let var14 = self.global32;
        let var15: i32;
        if var14 != 0 {
            let var16 = self.func44(imports, 2i32); // wasm/sound/registers/isChannelDacEnabled
            var15 = var16;
        } else {
            let var17 = self.global32;
            var15 = var17;
        }
        if var15 & 1i32 != 0 {
            let var18 = self.global43;
            var0 = var18;
        } else {
            return 15i32;
        }
        var1 = 1i32;
        let var19 = self.global49;
        let var20 = self.func47(imports, 1i32, var19); // wasm/sound/duty/isDutyCycleClockPositiveOrNegativeForWaveform
        if (var20 == 0) as i32 != 0 {
            var1 = -1i32;
        }
        let var21 = var1;
        let var22 = var0;
        var21.wrapping_mul(var22).wrapping_add(15i32)
    }
    // wasm/sound/channel3/Channel3.getSample
    fn func50<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = self.global50;
        let var4 = var0;
        self.global50 = var3.wrapping_sub(var4);
        let var5 = self.global50;
        if (var5 <= 0i32) as i32 != 0 {
            let var6 = self.global50;
            var1 = var6;
            let var7 = var1;
            let var8 = var1;
            let var9 = var1;
            var1 = if (var9 > 0i32) as i32 != 0 { var7 } else { 0i32.wrapping_sub(var8) };
            let var10 = self.func43(imports, 3i32); // wasm/sound/frequency/getChannelFrequency
            self.global50 = 2048i32.wrapping_sub(var10).wrapping_mul(2i32);
            let var11 = self.global50;
            let var12 = var1;
            self.global50 = var11.wrapping_sub(var12);
            let var13 = self.global51;
            self.global51 = var13.wrapping_add(1i32) & 65535i32;
            let var14 = self.global51;
            if (var14 as u32 >= 32i32 as u32) as i32 != 0 {
                self.global51 = 0i32;
            }
        }
        var1 = 0i32;
        let var15 = self.global34;
        let var16: i32;
        if var15 != 0 {
            let var17 = self.func44(imports, 3i32); // wasm/sound/registers/isChannelDacEnabled
            var16 = var17;
        } else {
            let var18 = self.global34;
            var16 = var18;
        }
        if var16 & 1i32 != 0 {
            let var19 = self.func13(imports, 65308i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            var2 = (var19 as u32).wrapping_shr(5i32 as u32) as i32 & 15i32;
        } else {
            return 15i32;
        }
        let var20 = self.global51;
        let var21 = self.func13(imports, ((var20 as u32 / 2i32 as u32) as i32 & 65535i32).wrapping_add(65328i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var0 = var21;
        let var22 = self.global51;
        let var23: i32;
        if (var22 as u32).wrapping_rem(2i32 as u32) as i32 != 0 {
            let var24 = var0;
            var23 = var24 & 15i32;
        } else {
            let var25 = var0;
            var23 = var25.wrapping_shr(4i32 as u32) & 15i32;
        }
        var0 = var23;
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var26 = var2;
                                match var26 {
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
                        let var27 = var0;
                        var0 = var27.wrapping_shr(4i32 as u32);
                        break 'label0;
                        break;
                    }
                    var1 = 1i32;
                    break 'label0;
                    break;
                }
                let var28 = var0;
                var0 = var28.wrapping_shr(1i32 as u32);
                var1 = 2i32;
                break 'label0;
                break;
            }
            let var29 = var0;
            var0 = var29.wrapping_shr(2i32 as u32);
            var1 = 4i32;
            break;
        }
        let var30 = var1;
        let var31: i32;
        if (var30 > 0i32) as i32 != 0 {
            let var32 = var0;
            let var33 = var1;
            var31 = (var32 / var33).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        } else {
            var31 = 0i32;
        }
        var0 = var31;
        let var34 = var0;
        var34.wrapping_add(15i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32)
    }
    // wasm/sound/channel4/Channel4.getNoiseChannelDivisorFromDivisorCode
    fn func51<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func13(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
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
    fn func52<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        (var0 as u32).wrapping_shr(4i32 as u32) as i32
    }
    // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
    fn func53<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func51(imports); // wasm/sound/channel4/Channel4.getNoiseChannelDivisorFromDivisorCode
        let var1 = self.func52(imports); // wasm/sound/channel4/Channel4.getNoiseChannelClockShift
        var0.wrapping_shl(var1 as u32) & 65535i32
    }
    // wasm/sound/channel4/Channel4.isNoiseChannelWidthModeSet
    fn func54<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65314i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var1 = self.func2(imports, 3i32, var0); // wasm/helpers/index/checkBitOnByte
        var1
    }
    // wasm/sound/channel4/Channel4.getSample
    fn func55<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = self.global52;
        let var3 = var0;
        self.global52 = var2.wrapping_sub(var3);
        let var4 = self.global52;
        if (var4 <= 0i32) as i32 != 0 {
            let var5 = self.global52;
            var0 = var5;
            let var6 = var0;
            let var7 = var0;
            let var8 = var0;
            var0 = if (var8 > 0i32) as i32 != 0 { var6 } else { 0i32.wrapping_sub(var7) };
            let var9 = self.func53(imports); // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
            self.global52 = var9;
            let var10 = self.global52;
            let var11 = var0;
            self.global52 = var10.wrapping_sub(var11);
            let var12 = self.global53;
            let var13 = self.global53;
            var1 = var12 & 1i32 ^ (var13 as u32).wrapping_shr(1i32 as u32) as i32 & 1i32;
            let var14 = self.global53;
            self.global53 = (var14 as u32).wrapping_shr(1i32 as u32) as i32;
            let var15 = self.global53;
            let var16 = var1;
            self.global53 = (var15 | var16.wrapping_shl(14i32 as u32) & 65535i32) & 65535i32;
            let var17 = self.func54(imports); // wasm/sound/channel4/Channel4.isNoiseChannelWidthModeSet
            if var17 != 0 {
                let var18 = self.global53;
                self.global53 = var18 & 65471i32;
                let var19 = self.global53;
                let var20 = var1;
                self.global53 = (var19 | var20.wrapping_shl(6i32 as u32) & 65535i32) & 65535i32;
            }
        }
        let var21 = self.global36;
        let var22: i32;
        if var21 != 0 {
            let var23 = self.func44(imports, 4i32); // wasm/sound/registers/isChannelDacEnabled
            var22 = var23;
        } else {
            let var24 = self.global36;
            var22 = var24;
        }
        if var22 & 1i32 != 0 {
            let var25 = self.global45;
            var1 = var25;
        } else {
            return 15i32;
        }
        let var26 = self.global53;
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
    fn func56<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func13(imports, 65317i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var3 = self.func2(imports, ((var1 & 255i32).wrapping_sub(1i32) & 255i32).wrapping_add(4i32) & 255i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/registers/isChannelEnabledOnRightOutput
    fn func57<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func13(imports, 65317i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var3 = self.func2(imports, (var1 & 255i32).wrapping_sub(1i32) & 255i32, var2); // wasm/helpers/index/checkBitOnByte
        var3
    }
    // wasm/sound/sound/getSampleAsUnsignedByte
    fn func58<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let var3 = var0;
        if (var3 == 60i32) as i32 != 0 {
            return 127i32;
        }
        let var4 = var0;
        var2 = 100000i32;
        let var5 = var2;
        let var6 = var1;
        (var4.wrapping_sub(60i32).wrapping_mul(var5).wrapping_mul(var6) / 8i32 / 100000i32).wrapping_add(60i32).wrapping_mul(100000i32) / 47244i32 & 255i32
    }
    // wasm/memory/memory/setLeftAndRightOutputForAudioQueue
    fn func59<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
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
    // wasm/sound/sound/updateSound
    fn func60<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
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
        let var12 = self.global27;
        let var13 = var0;
        self.global27 = var12.wrapping_add(var13);
        let var14 = self.global27;
        if (var14 >= 8192i32) as i32 != 0 {
            let var15 = self.global27;
            self.global27 = var15.wrapping_sub(8192i32);
            'label0: loop {
                'label1: loop {
                    'label2: loop {
                        'label3: loop {
                            'label4: loop {
                                'label5: loop {
                                    'label6: loop {
                                        let var16 = self.global28;
                                        match var16 {
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
                                self.func24(imports); // wasm/sound/channel1/Channel1.updateLength
                                self.func25(imports); // wasm/sound/channel2/Channel2.updateLength
                                self.func26(imports); // wasm/sound/channel3/Channel3.updateLength
                                self.func27(imports); // wasm/sound/channel4/Channel4.updateLength
                                break 'label0;
                                break;
                            }
                            self.func24(imports); // wasm/sound/channel1/Channel1.updateLength
                            self.func25(imports); // wasm/sound/channel2/Channel2.updateLength
                            self.func26(imports); // wasm/sound/channel3/Channel3.updateLength
                            self.func27(imports); // wasm/sound/channel4/Channel4.updateLength
                            self.func35(imports); // wasm/sound/channel1/Channel1.updateSweep
                            break 'label0;
                            break;
                        }
                        self.func24(imports); // wasm/sound/channel1/Channel1.updateLength
                        self.func25(imports); // wasm/sound/channel2/Channel2.updateLength
                        self.func26(imports); // wasm/sound/channel3/Channel3.updateLength
                        self.func27(imports); // wasm/sound/channel4/Channel4.updateLength
                        break 'label0;
                        break;
                    }
                    self.func24(imports); // wasm/sound/channel1/Channel1.updateLength
                    self.func25(imports); // wasm/sound/channel2/Channel2.updateLength
                    self.func26(imports); // wasm/sound/channel3/Channel3.updateLength
                    self.func27(imports); // wasm/sound/channel4/Channel4.updateLength
                    self.func35(imports); // wasm/sound/channel1/Channel1.updateSweep
                    break 'label0;
                    break;
                }
                self.func39(imports); // wasm/sound/channel1/Channel1.updateEnvelope
                self.func40(imports); // wasm/sound/channel2/Channel2.updateEnvelope
                self.func41(imports); // wasm/sound/channel4/Channel4.updateEnvelope
                break;
            }
            let var17 = self.global28;
            self.global28 = var17.wrapping_add(1i32) & 255i32;
            let var18 = self.global28;
            if (var18 as u32 >= 8i32 as u32) as i32 != 0 {
                self.global28 = 0i32;
            }
        }
        let var19 = var0;
        let var20 = self.func48(imports, var19); // wasm/sound/channel1/Channel1.getSample
        var3 = var20;
        let var21 = var0;
        let var22 = self.func49(imports, var21); // wasm/sound/channel2/Channel2.getSample
        var4 = var22;
        let var23 = var0;
        let var24 = self.func50(imports, var23); // wasm/sound/channel3/Channel3.getSample
        var5 = var24;
        let var25 = var0;
        let var26 = self.func55(imports, var25); // wasm/sound/channel4/Channel4.getSample
        var6 = var26;
        let var27 = self.global54;
        let var28 = var0;
        self.global54 = var27.wrapping_add(var28);
        let var29 = self.global54;
        if (var29 >= 87i32) as i32 != 0 {
            let var30 = self.global54;
            self.global54 = var30.wrapping_sub(87i32);
            let var31 = self.func13(imports, 65316i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            var0 = var31;
            let var32 = var0;
            var7 = (var32 as u32).wrapping_shr(4i32 as u32) as i32 & 7i32;
            let var33 = var0;
            var0 = var33 & 7i32;
            var8 = 1i32;
            let var34 = var8;
            let var35 = self.func56(imports, var34); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            let var36: i32;
            if var35 != 0 {
                let var37 = var3;
                var36 = 0i32.wrapping_add(var37);
            } else {
                var36 = 15i32;
            }
            var1 = var36;
            var9 = 2i32;
            let var38 = var9;
            let var39 = self.func56(imports, var38); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            let var40: i32;
            if var39 != 0 {
                let var41 = var1;
                let var42 = var4;
                var40 = var41.wrapping_add(var42);
            } else {
                let var43 = var1;
                var40 = var43.wrapping_add(15i32);
            }
            var1 = var40;
            var10 = 3i32;
            let var44 = var10;
            let var45 = self.func56(imports, var44); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            let var46: i32;
            if var45 != 0 {
                let var47 = var1;
                let var48 = var5;
                var46 = var47.wrapping_add(var48);
            } else {
                let var49 = var1;
                var46 = var49.wrapping_add(15i32);
            }
            var1 = var46;
            var11 = 4i32;
            let var50 = var11;
            let var51 = self.func56(imports, var50); // wasm/sound/registers/isChannelEnabledOnLeftOutput
            let var52: i32;
            if var51 != 0 {
                let var53 = var1;
                let var54 = var6;
                var52 = var53.wrapping_add(var54);
            } else {
                let var55 = var1;
                var52 = var55.wrapping_add(15i32);
            }
            var1 = var52;
            let var56 = self.func57(imports, 1i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            let var57: i32;
            if var56 != 0 {
                let var58 = var3;
                var57 = 0i32.wrapping_add(var58);
            } else {
                var57 = 15i32;
            }
            var2 = var57;
            let var59 = self.func57(imports, 2i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            let var60: i32;
            if var59 != 0 {
                let var61 = var2;
                let var62 = var4;
                var60 = var61.wrapping_add(var62);
            } else {
                let var63 = var2;
                var60 = var63.wrapping_add(15i32);
            }
            var2 = var60;
            let var64 = self.func57(imports, 3i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            let var65: i32;
            if var64 != 0 {
                let var66 = var2;
                let var67 = var5;
                var65 = var66.wrapping_add(var67);
            } else {
                let var68 = var2;
                var65 = var68.wrapping_add(15i32);
            }
            var2 = var65;
            let var69 = self.func57(imports, 4i32); // wasm/sound/registers/isChannelEnabledOnRightOutput
            let var70: i32;
            if var69 != 0 {
                let var71 = var2;
                let var72 = var6;
                var70 = var71.wrapping_add(var72);
            } else {
                let var73 = var2;
                var70 = var73.wrapping_add(15i32);
            }
            var2 = var70;
            let var74 = var1;
            let var75 = var7;
            let var76 = self.func58(imports, var74, var75.wrapping_add(1i32)); // wasm/sound/sound/getSampleAsUnsignedByte
            let var77 = var2;
            let var78 = var0;
            let var79 = self.func58(imports, var77, var78.wrapping_add(1i32)); // wasm/sound/sound/getSampleAsUnsignedByte
            let var80 = self.global55;
            self.func59(imports, var76.wrapping_add(1i32) & 255i32, var79.wrapping_add(1i32) & 255i32, var80); // wasm/memory/memory/setLeftAndRightOutputForAudioQueue
            let var81 = self.global55;
            self.global55 = var81.wrapping_add(1i32);
            let var82 = self.global55;
            let var83 = self.global56;
            if (var82 >= (var83 / 2i32).wrapping_sub(1i32)) as i32 != 0 {
                let var84 = self.global55;
                self.global55 = var84.wrapping_sub(1i32);
            }
        }
    }
    // wasm/sound/sound/batchProcessAudio
    fn func61<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global25;
        let var1 = self.global26;
        if ((var0) < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global25;
            let var3 = self.global26;
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.global26;
                self.func60(imports, var4); // wasm/sound/sound/updateSound
                let var5 = self.global25;
                let var6 = self.global26;
                self.global25 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/sound/length/setChannelLengthCounter
    fn func62<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3 = var0;
        let var4 = self.func45(imports, var3); // wasm/sound/registers/getRegister1OfChannel
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
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var9 = var0;
                                match var9.wrapping_sub(1i32) {
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
                        let var10 = var1;
                        self.global29 = var10;
                        break 'label0;
                        break;
                    }
                    let var11 = var1;
                    self.global31 = var11;
                    break 'label0;
                    break;
                }
                let var12 = var1;
                self.global33 = var12;
                break 'label0;
                break;
            }
            let var13 = var1;
            self.global35 = var13;
            break;
        }
    }
    // wasm/sound/registers/getChannelStartingVolume
    fn func63<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func36(imports, var1); // wasm/sound/registers/getRegister2OfChannel
        (var2 as u32).wrapping_shr(4i32 as u32) as i32 & 15i32
    }
    // wasm/sound/channel1/Channel1.trigger
    fn func64<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        self.global30 = 1i32;
        let var1 = self.global29;
        if (var1 == 0) as i32 != 0 {
            self.global29 = 64i32;
        }
        let var2 = self.func43(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
        self.global46 = 2048i32.wrapping_sub(var2).wrapping_mul(4i32);
        let var3 = self.func37(imports, 1i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global40 = var3;
        let var4 = self.func63(imports, 1i32); // wasm/sound/registers/getChannelStartingVolume
        self.global41 = var4;
        let var5 = self.func43(imports, 1i32); // wasm/sound/frequency/getChannelFrequency
        self.global39 = var5;
        let var6 = self.func28(imports); // wasm/sound/channel1/getSweepPeriod
        self.global37 = var6;
        let var7 = self.func28(imports); // wasm/sound/channel1/getSweepPeriod
        var0 = (var7 as u32 > 0i32 as u32) as i32;
        let var8 = var0;
        let var9: i32;
        if var8 != 0 {
            let var10 = self.func29(imports); // wasm/sound/channel1/getSweepShift
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
        let var12 = self.func29(imports); // wasm/sound/channel1/getSweepShift
        if (var12 as u32 > 0i32 as u32) as i32 != 0 {
            self.func34(imports); // wasm/sound/channel1/calculateSweepAndCheckOverflow
        }
        let var13 = self.func44(imports, 1i32); // wasm/sound/registers/isChannelDacEnabled
        if (var13 == 0) as i32 != 0 {
            self.global30 = 0i32;
        }
    }
    // wasm/sound/channel2/Channel2.trigger
    fn func65<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global32 = 1i32;
        let var0 = self.global31;
        if (var0 == 0) as i32 != 0 {
            self.global31 = 64i32;
        }
        let var1 = self.func43(imports, 2i32); // wasm/sound/frequency/getChannelFrequency
        self.global48 = 2048i32.wrapping_sub(var1).wrapping_mul(4i32);
        let var2 = self.func37(imports, 2i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global42 = var2;
        let var3 = self.func63(imports, 2i32); // wasm/sound/registers/getChannelStartingVolume
        self.global43 = var3;
        let var4 = self.func44(imports, 2i32); // wasm/sound/registers/isChannelDacEnabled
        if (var4 == 0) as i32 != 0 {
            self.global32 = 0i32;
        }
    }
    // wasm/sound/channel3/Channel3.trigger
    fn func66<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global34 = 1i32;
        let var0 = self.global33;
        if (var0 == 0) as i32 != 0 {
            self.global33 = 256i32;
        }
        let var1 = self.func43(imports, 3i32); // wasm/sound/frequency/getChannelFrequency
        self.global50 = 2048i32.wrapping_sub(var1).wrapping_mul(2i32);
        self.global51 = 0i32;
        let var2 = self.func44(imports, 3i32); // wasm/sound/registers/isChannelDacEnabled
        if (var2 == 0) as i32 != 0 {
            self.global34 = 0i32;
        }
    }
    // wasm/sound/channel4/Channel4.trigger
    fn func67<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global36 = 1i32;
        let var0 = self.global35;
        if (var0 == 0) as i32 != 0 {
            self.global35 = 64i32;
        }
        let var1 = self.func53(imports); // wasm/sound/channel4/Channel4.getNoiseChannelFrequencyPeriod
        self.global52 = var1;
        let var2 = self.func37(imports, 4i32); // wasm/sound/envelope/getChannelEnvelopePeriod
        self.global44 = var2;
        let var3 = self.func63(imports, 4i32); // wasm/sound/registers/getChannelStartingVolume
        self.global45 = var3;
        self.global53 = 32767i32;
        let var4 = self.func44(imports, 4i32); // wasm/sound/registers/isChannelDacEnabled
        if (var4 == 0) as i32 != 0 {
            self.global36 = 0i32;
        }
    }
    // wasm/sound/registers/handledWriteToSoundRegister
    fn func68<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.func13(imports, 65318i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
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
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        let var11 = var0;
                        var2 = var11;
                        let var12 = var2;
                        if (var12 != 65297i32) as i32 != 0 {
                            let var13 = var2;
                            if (var13 == 65302i32) as i32 != 0 {
                                break 'label3;
                            }
                            let var14 = var2;
                            if (var14 == 65307i32) as i32 != 0 {
                                break 'label2;
                            }
                            let var15 = var2;
                            if (var15 == 65312i32) as i32 != 0 {
                                break 'label1;
                            }
                            break 'label0;
                        }
                        let var16 = var0;
                        let var17 = var1;
                        self.func8(imports, var16, var17 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        self.func62(imports, 1i32); // wasm/sound/length/setChannelLengthCounter
                        return 1i32;
                        break;
                    }
                    let var18 = var0;
                    let var19 = var1;
                    self.func8(imports, var18, var19 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func62(imports, 2i32); // wasm/sound/length/setChannelLengthCounter
                    return 1i32;
                    break;
                }
                let var20 = var0;
                let var21 = var1;
                self.func8(imports, var20, var21 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                self.func62(imports, 3i32); // wasm/sound/length/setChannelLengthCounter
                return 1i32;
                break;
            }
            let var22 = var0;
            let var23 = var1;
            self.func8(imports, var22, var23 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            self.func62(imports, 4i32); // wasm/sound/length/setChannelLengthCounter
            return 1i32;
            break;
        }
        let var24 = var0;
        var2 = (var24 == 65300i32) as i32;
        let var25 = var2;
        let var26: i32;
        if var25 != 0 {
            let var27 = var1;
            let var28 = self.func2(imports, 7i32, var27 & 255i32); // wasm/helpers/index/checkBitOnByte
            var26 = var28;
        } else {
            let var29 = var2;
            var26 = var29;
        }
        if var26 & 1i32 != 0 {
            let var30 = var0;
            let var31 = var1;
            self.func8(imports, var30, var31 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            self.func64(imports); // wasm/sound/channel1/Channel1.trigger
            return 1i32;
        } else {
            let var32 = var0;
            var2 = (var32 == 65305i32) as i32;
            let var33 = var2;
            let var34: i32;
            if var33 != 0 {
                let var35 = var1;
                let var36 = self.func2(imports, 7i32, var35 & 255i32); // wasm/helpers/index/checkBitOnByte
                var34 = var36;
            } else {
                let var37 = var2;
                var34 = var37;
            }
            if var34 & 1i32 != 0 {
                let var38 = var0;
                let var39 = var1;
                self.func8(imports, var38, var39 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                self.func65(imports); // wasm/sound/channel2/Channel2.trigger
                return 1i32;
            } else {
                let var40 = var0;
                var2 = (var40 == 65310i32) as i32;
                let var41 = var2;
                let var42: i32;
                if var41 != 0 {
                    let var43 = var1;
                    let var44 = self.func2(imports, 7i32, var43 & 255i32); // wasm/helpers/index/checkBitOnByte
                    var42 = var44;
                } else {
                    let var45 = var2;
                    var42 = var45;
                }
                if var42 & 1i32 != 0 {
                    let var46 = var0;
                    let var47 = var1;
                    self.func8(imports, var46, var47 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    self.func66(imports); // wasm/sound/channel3/Channel3.trigger
                    return 1i32;
                } else {
                    let var48 = var0;
                    var2 = (var48 == 65315i32) as i32;
                    let var49 = var2;
                    let var50: i32;
                    if var49 != 0 {
                        let var51 = var1;
                        let var52 = self.func2(imports, 7i32, var51 & 255i32); // wasm/helpers/index/checkBitOnByte
                        var50 = var52;
                    } else {
                        let var53 = var2;
                        var50 = var53;
                    }
                    if var50 & 1i32 != 0 {
                        let var54 = var0;
                        let var55 = var1;
                        self.func8(imports, var54, var55 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                        self.func67(imports); // wasm/sound/channel4/Channel4.trigger
                        return 1i32;
                    }
                }
            }
        }
        let var56 = var0;
        var2 = (var56 == 65318i32) as i32;
        let var57 = var2;
        let var58: i32;
        if var57 != 0 {
            let var59 = var1;
            let var60 = self.func2(imports, 7i32, var59 & 255i32); // wasm/helpers/index/checkBitOnByte
            var58 = (var60 == 0) as i32;
        } else {
            let var61 = var2;
            var58 = var61;
        }
        if var58 & 1i32 != 0 {
            var2 = 65296i32;
            'label4: loop {
                let var62 = var2;
                if ((var62 as u32) < 65318i32 as u32) as i32 != 0 {
                    let var63 = var2;
                    self.func8(imports, var63, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                    let var64 = var2;
                    var2 = var64.wrapping_add(1i32) & 65535i32;
                    continue 'label4;
                }
                break;
            }
            let var65 = var0;
            let var66 = var1;
            self.func8(imports, var65, var66 & 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return 1i32;
        }
        0i32
    }
    // wasm/memory/writeTraps/_dmaTransfer
    fn func69<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
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
    fn func70<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
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
            return 1i32;
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
            return 1i32;
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
            return 1i32;
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
        var3 = (var36 as u32 >= 65284i32 as u32) as i32;
        let var37 = var3;
        let var38: i32;
        if var37 != 0 {
            let var39 = var0;
            var38 = (var39 as u32 <= 65287i32 as u32) as i32;
        } else {
            let var40 = var3;
            var38 = var40;
        }
        if var38 & 1i32 != 0 {
            self.func21(imports); // wasm/timers/index/batchProcessTimers
            let var41 = var0;
            if (var41 == 65284i32) as i32 != 0 {
                let var42 = var0;
                self.func8(imports, var42, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                return 0i32;
            }
            return 1i32;
        }
        let var43 = var0;
        var3 = (var43 as u32 >= 65296i32 as u32) as i32;
        let var44 = var3;
        let var45: i32;
        if var44 != 0 {
            let var46 = var0;
            var45 = (var46 as u32 <= 65318i32 as u32) as i32;
        } else {
            let var47 = var3;
            var45 = var47;
        }
        if var45 & 1i32 != 0 {
            self.func61(imports); // wasm/sound/sound/batchProcessAudio
            let var48 = var0;
            let var49 = var1;
            let var50 = self.func68(imports, var48, var49); // wasm/sound/registers/handledWriteToSoundRegister
            if var50 != 0 {
                return 0i32;
            }
        }
        let var51 = var0;
        var3 = (var51 as u32 >= 65328i32 as u32) as i32;
        let var52 = var3;
        let var53: i32;
        if var52 != 0 {
            let var54 = var0;
            var53 = (var54 as u32 <= 65343i32 as u32) as i32;
        } else {
            let var55 = var3;
            var53 = var55;
        }
        if var53 & 1i32 != 0 {
            self.func61(imports); // wasm/sound/sound/batchProcessAudio
        }
        let var56 = var0;
        var3 = (var56 as u32 >= 65344i32 as u32) as i32;
        let var57 = var3;
        let var58: i32;
        if var57 != 0 {
            let var59 = var0;
            var58 = (var59 as u32 <= 65355i32 as u32) as i32;
        } else {
            let var60 = var3;
            var58 = var60;
        }
        if var58 & 1i32 != 0 {
            let var61 = var0;
            if (var61 == 65348i32) as i32 != 0 {
                let var62 = var0;
                self.func8(imports, var62, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
                return 0i32;
            }
            let var63 = var0;
            if (var63 == 65350i32) as i32 != 0 {
                let var64 = var1;
                self.func69(imports, var64 & 255i32); // wasm/memory/writeTraps/_dmaTransfer
            }
            return 1i32;
        }
        1i32
    }
    // wasm/memory/store/eightBitStoreIntoGBMemory
    fn func71<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = var1;
        let var4 = self.func70(imports, var2, var3, 1i32); // wasm/memory/writeTraps/checkWriteTraps
        if var4 != 0 {
            let var5 = var0;
            let var6 = var1;
            self.func7(imports, var5, var6); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
    }
    // wasm/helpers/index/resetBitOnByte
    fn func72<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        var2 & (1i32.wrapping_shl(var3 as u32) & 255i32 ^ -1i32) & 255i32
    }
    // wasm/joypad/index/getJoypadState
    fn func73<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
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
                let var7 = self.global61;
                let var8: i32;
                if var7 != 0 {
                    let var9 = var0;
                    let var10 = self.func72(imports, 2i32, var9); // wasm/helpers/index/resetBitOnByte
                    var8 = var10;
                } else {
                    let var11 = var0;
                    let var12 = self.func17(imports, 2i32, var11); // wasm/helpers/index/setBitOnByte
                    var8 = var12;
                }
                var0 = var8;
                let var13 = self.global62;
                let var14: i32;
                if var13 != 0 {
                    let var15 = var0;
                    let var16 = self.func72(imports, 0i32, var15); // wasm/helpers/index/resetBitOnByte
                    var14 = var16;
                } else {
                    let var17 = var0;
                    let var18 = self.func17(imports, 0i32, var17); // wasm/helpers/index/setBitOnByte
                    var14 = var18;
                }
                var0 = var14;
                let var19 = self.global63;
                let var20: i32;
                if var19 != 0 {
                    let var21 = var0;
                    let var22 = self.func72(imports, 3i32, var21); // wasm/helpers/index/resetBitOnByte
                    var20 = var22;
                } else {
                    let var23 = var0;
                    let var24 = self.func17(imports, 3i32, var23); // wasm/helpers/index/setBitOnByte
                    var20 = var24;
                }
                var0 = var20;
                let var25 = self.global64;
                let var26: i32;
                if var25 != 0 {
                    let var27 = var0;
                    let var28 = self.func72(imports, 1i32, var27); // wasm/helpers/index/resetBitOnByte
                    var26 = var28;
                } else {
                    let var29 = var0;
                    let var30 = self.func17(imports, 1i32, var29); // wasm/helpers/index/setBitOnByte
                    var26 = var30;
                }
                var0 = var26;
            }
        } else {
            let var31 = var0;
            var0 = (var31 | 240i32) & 255i32;
            let var32 = self.global57;
            let var33: i32;
            if var32 != 0 {
                let var34 = var0;
                let var35 = self.func72(imports, 0i32, var34); // wasm/helpers/index/resetBitOnByte
                var33 = var35;
            } else {
                let var36 = var0;
                let var37 = self.func17(imports, 0i32, var36); // wasm/helpers/index/setBitOnByte
                var33 = var37;
            }
            var0 = var33;
            let var38 = self.global58;
            let var39: i32;
            if var38 != 0 {
                let var40 = var0;
                let var41 = self.func72(imports, 1i32, var40); // wasm/helpers/index/resetBitOnByte
                var39 = var41;
            } else {
                let var42 = var0;
                let var43 = self.func17(imports, 1i32, var42); // wasm/helpers/index/setBitOnByte
                var39 = var43;
            }
            var0 = var39;
            let var44 = self.global59;
            let var45: i32;
            if var44 != 0 {
                let var46 = var0;
                let var47 = self.func72(imports, 2i32, var46); // wasm/helpers/index/resetBitOnByte
                var45 = var47;
            } else {
                let var48 = var0;
                let var49 = self.func17(imports, 2i32, var48); // wasm/helpers/index/setBitOnByte
                var45 = var49;
            }
            var0 = var45;
            let var50 = self.global60;
            let var51: i32;
            if var50 != 0 {
                let var52 = var0;
                let var53 = self.func72(imports, 3i32, var52); // wasm/helpers/index/resetBitOnByte
                var51 = var53;
            } else {
                let var54 = var0;
                let var55 = self.func17(imports, 3i32, var54); // wasm/helpers/index/setBitOnByte
                var51 = var55;
            }
            var0 = var51;
        }
        let var56 = var0;
        var56
    }
    // wasm/memory/readTraps/checkReadTraps
    fn func74<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
            let var17 = self.func73(imports); // wasm/joypad/index/getJoypadState
            return var17;
        }
        let var18 = var0;
        var1 = (var18 as u32 >= 65296i32 as u32) as i32;
        let var19 = var1;
        let var20: i32;
        if var19 != 0 {
            let var21 = var0;
            var20 = (var21 as u32 <= 65318i32 as u32) as i32;
        } else {
            let var22 = var1;
            var20 = var22;
        }
        if var20 & 1i32 != 0 {
            self.func61(imports); // wasm/sound/sound/batchProcessAudio
        }
        let var23 = var0;
        var1 = (var23 as u32 >= 65328i32 as u32) as i32;
        let var24 = var1;
        let var25: i32;
        if var24 != 0 {
            let var26 = var0;
            var25 = (var26 as u32 <= 65343i32 as u32) as i32;
        } else {
            let var27 = var1;
            var25 = var27;
        }
        if var25 & 1i32 != 0 {
            self.func61(imports); // wasm/sound/sound/batchProcessAudio
        }
        -1i32
    }
    // wasm/memory/load/eightBitLoadFromGBMemory
    fn func75<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        let var3 = self.func74(imports, var2); // wasm/memory/readTraps/checkReadTraps
        var1 = var3;
        let var4 = var1;
        if (var4 == -1i32) as i32 != 0 {
            let var5 = var0;
            let var6 = self.func12(imports, var5); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
            return var6;
        }
        let var7 = var1;
        var7 & 255i32
    }
    // wasm/memory/memory/initializeCartridge
    fn func76<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let var2 = self.func75(imports, 327i32); // wasm/memory/load/eightBitLoadFromGBMemory
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
    fn func77<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func8(imports, 65296i32, 128i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65297i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65298i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65299i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65300i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/sound/channel2/Channel2.initialize
    fn func78<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func8(imports, 65301i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65302i32, 63i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65303i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65304i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65305i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/sound/channel3/Channel3.initialize
    fn func79<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func8(imports, 65306i32, 127i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65307i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65308i32, 159i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65309i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65310i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/sound/channel4/Channel4.initialize
    fn func80<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func8(imports, 65311i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65312i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65313i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65314i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65315i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/sound/sound/initializeSound
    fn func81<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func77(imports); // wasm/sound/channel1/Channel1.initialize
        self.func78(imports); // wasm/sound/channel2/Channel2.initialize
        self.func79(imports); // wasm/sound/channel3/Channel3.initialize
        self.func80(imports); // wasm/sound/channel4/Channel4.initialize
        self.func8(imports, 65316i32, 119i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65317i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        self.func8(imports, 65318i32, 241i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/cpu/cpu/initialize
    fn func82<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.func1(imports, 8861696i32, 1i32, var1, 0i32, 0i32, 0i32, 0i32); // wasm/helpers/index/log
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
            self.func71(imports, 65296i32, 128i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65297i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65298i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65300i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65302i32, 63i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65303i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65305i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65306i32, 127i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65307i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65308i32, 159i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65310i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65312i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65315i32, 191i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65316i32, 119i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65317i32, 243i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65318i32, 241i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65344i32, 145i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65351i32, 252i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65352i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
            self.func71(imports, 65353i32, 255i32); // wasm/memory/store/eightBitStoreIntoGBMemory
        }
        self.func76(imports); // wasm/memory/memory/initializeCartridge
        self.func81(imports); // wasm/sound/sound/initializeSound
    }
    // wasm/config/config
    fn func83<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let var3 = var0;
        if (var3 > 0i32) as i32 != 0 {
            self.global65 = 1i32;
        }
        let var4 = var1;
        if (var4 > 0i32) as i32 != 0 {
            self.global66 = 1i32;
        }
        let var5 = var2;
        if (var5 > 0i32) as i32 != 0 {
            self.global67 = 1i32;
        }
    }
    // wasm/helpers/index/concatenateBytes
    fn func84<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = var1;
        (var2 & 255i32).wrapping_shl(8i32 as u32) | var3 & 255i32
    }
    // wasm/cpu/flags/setFlagBit
    fn func85<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
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
    fn func86<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func85(imports, 5i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
    fn func87<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        if (var3 >= 0i32) as i32 != 0 {
            let var4 = var0;
            let var5 = var1;
            if (var4 & 15i32).wrapping_add(var5 & 15i32) & 16i32 != 0 {
                self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        } else {
            let var6 = var1;
            var2 = var6;
            let var7 = var2;
            let var8 = var2;
            let var9 = var2;
            let var10 = var0;
            if (((if (var9 > 0i32) as i32 != 0 { var7 } else { 0i32.wrapping_sub(var8) }) & 15i32) as u32 > (var10 & 15i32) as u32) as i32 != 0 {
                self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        }
    }
    // wasm/cpu/flags/setZeroFlag
    fn func88<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func85(imports, 7i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/setSubtractFlag
    fn func89<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func85(imports, 6i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/cpu/flags/setCarryFlag
    fn func90<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        let var2 = self.func85(imports, 4i32, var1); // wasm/cpu/flags/setFlagBit
    }
    // wasm/helpers/index/rotateByteLeft
    fn func91<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        (var1.wrapping_shl(1i32 as u32) & 255i32 | (var2 as u32).wrapping_shr(7i32 as u32) as i32) & 255i32
    }
    // wasm/memory/store/sixteenBitStoreIntoGBMemory
    fn func92<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        let var4 = self.func9(imports, var3); // wasm/helpers/index/splitHighByte
        var2 = var4;
        let var5 = var0;
        let var6 = var1;
        let var7 = self.func10(imports, var6); // wasm/helpers/index/splitLowByte
        var1 = var7;
        let var8 = var1;
        let var9 = self.func70(imports, var5, var8, 0i32); // wasm/memory/writeTraps/checkWriteTraps
        if var9 != 0 {
            let var10 = var0;
            let var11 = var1;
            self.func7(imports, var10, var11); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
        let var12 = var0;
        var0 = var12.wrapping_add(1i32) & 65535i32;
        let var13 = var0;
        let var14 = var2;
        let var15 = self.func70(imports, var13, var14, 0i32); // wasm/memory/writeTraps/checkWriteTraps
        if var15 != 0 {
            let var16 = var0;
            let var17 = var2;
            self.func7(imports, var16, var17); // wasm/memory/store/_eightBitStoreIntoWasmBoyMemory
        }
    }
    // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
    fn func93<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let var3 = var2;
        if var3 != 0 {
            let var4 = var0;
            let var5 = var1;
            let var6 = var0;
            let var7 = var1;
            var2 = var4 ^ var5 ^ var6.wrapping_add(var7);
            let var8 = var2;
            if var8 & 16i32 != 0 {
                self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
            let var9 = var2;
            if var9 & 256i32 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        } else {
            let var10 = var0;
            let var11 = var1;
            var2 = var10.wrapping_add(var11 & 65535i32) & 65535i32;
            let var12 = var2;
            let var13 = var0;
            if ((var12 as u32) < var13 as u32) as i32 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
            let var14 = var0;
            let var15 = var1;
            let var16 = var2;
            if (var14 ^ var15 & 65535i32 ^ var16) & 4096i32 != 0 {
                self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            } else {
                self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            }
        }
    }
    // wasm/helpers/index/rotateByteRight
    fn func94<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        ((var1 as u32).wrapping_shr(1i32 as u32) as i32 | var2.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    // wasm/cpu/opcodes/handleOpcode0x
    fn func95<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4 {
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
                                                                    let var5 = var3;
                                                                    let var6 = self.func9(imports, var5); // wasm/helpers/index/splitHighByte
                                                                    self.global3 = var6;
                                                                    let var7 = var3;
                                                                    let var8 = self.func10(imports, var7); // wasm/helpers/index/splitLowByte
                                                                    self.global4 = var8;
                                                                    let var9 = self.global0;
                                                                    self.global0 = var9.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var10 = self.global3;
                                                                let var11 = self.global4;
                                                                let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                                let var13 = self.global1;
                                                                self.func71(imports, var12, var13); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var14 = self.global3;
                                                            let var15 = self.global4;
                                                            let var16 = self.func84(imports, var14, var15); // wasm/helpers/index/concatenateBytes
                                                            var0 = var16.wrapping_add(1i32) & 65535i32;
                                                            let var17 = var0;
                                                            let var18 = self.func9(imports, var17); // wasm/helpers/index/splitHighByte
                                                            self.global3 = var18;
                                                            let var19 = var0;
                                                            let var20 = self.func10(imports, var19); // wasm/helpers/index/splitLowByte
                                                            self.global4 = var20;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var21 = self.global3;
                                                        self.func87(imports, var21, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                        let var22 = self.global3;
                                                        self.global3 = var22.wrapping_add(1i32) & 255i32;
                                                        let var23 = self.global3;
                                                        if var23 != 0 {
                                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                        } else {
                                                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                        }
                                                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var24 = self.global3;
                                                    self.func87(imports, var24, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                    let var25 = self.global3;
                                                    self.global3 = var25.wrapping_sub(1i32) & 255i32;
                                                    let var26 = self.global3;
                                                    if var26 != 0 {
                                                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                    } else {
                                                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                    }
                                                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                    return 4i32;
                                                    break;
                                                }
                                                let var27 = var1;
                                                self.global3 = var27;
                                                let var28 = self.global0;
                                                self.global0 = var28.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            let var29 = self.global1;
                                            if (var29 & 128i32 == 128i32) as i32 != 0 {
                                                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            let var30 = self.global1;
                                            let var31 = self.func91(imports, var30); // wasm/helpers/index/rotateByteLeft
                                            self.global1 = var31;
                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            return 4i32;
                                            break;
                                        }
                                        let var32 = var3;
                                        let var33 = self.global9;
                                        self.func92(imports, var32, var33); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                        let var34 = self.global0;
                                        self.global0 = var34.wrapping_add(2i32) & 65535i32;
                                        return 20i32;
                                        break;
                                    }
                                    let var35 = self.global7;
                                    let var36 = self.global8;
                                    let var37 = self.func84(imports, var35, var36); // wasm/helpers/index/concatenateBytes
                                    var0 = var37;
                                    let var38 = var0;
                                    let var39 = self.global3;
                                    let var40 = self.global4;
                                    let var41 = self.func84(imports, var39, var40); // wasm/helpers/index/concatenateBytes
                                    var1 = var41;
                                    let var42 = var1;
                                    self.func93(imports, var38, var42, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                    let var43 = var0;
                                    let var44 = var1;
                                    var0 = var43.wrapping_add(var44) & 65535i32;
                                    let var45 = var0;
                                    let var46 = self.func9(imports, var45); // wasm/helpers/index/splitHighByte
                                    self.global7 = var46;
                                    let var47 = var0;
                                    let var48 = self.func10(imports, var47); // wasm/helpers/index/splitLowByte
                                    self.global8 = var48;
                                    self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                    return 8i32;
                                    break;
                                }
                                let var49 = self.global3;
                                let var50 = self.global4;
                                let var51 = self.func84(imports, var49, var50); // wasm/helpers/index/concatenateBytes
                                let var52 = self.func75(imports, var51); // wasm/memory/load/eightBitLoadFromGBMemory
                                self.global1 = var52;
                                return 8i32;
                                break;
                            }
                            let var53 = self.global3;
                            let var54 = self.global4;
                            let var55 = self.func84(imports, var53, var54); // wasm/helpers/index/concatenateBytes
                            var0 = var55.wrapping_sub(1i32) & 65535i32;
                            let var56 = var0;
                            let var57 = self.func9(imports, var56); // wasm/helpers/index/splitHighByte
                            self.global3 = var57;
                            let var58 = var0;
                            let var59 = self.func10(imports, var58); // wasm/helpers/index/splitLowByte
                            self.global4 = var59;
                            return 8i32;
                            break;
                        }
                        let var60 = self.global4;
                        self.func87(imports, var60, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                        let var61 = self.global4;
                        self.global4 = var61.wrapping_add(1i32) & 255i32;
                        let var62 = self.global4;
                        if var62 != 0 {
                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                        } else {
                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                        }
                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                        return 4i32;
                        break;
                    }
                    let var63 = self.global4;
                    self.func87(imports, var63, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                    let var64 = self.global4;
                    self.global4 = var64.wrapping_sub(1i32) & 255i32;
                    let var65 = self.global4;
                    if var65 != 0 {
                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                    } else {
                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                    }
                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                    return 4i32;
                    break;
                }
                let var66 = var1;
                self.global4 = var66;
                let var67 = self.global0;
                self.global0 = var67.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var68 = self.global1;
            if ((var68 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
            let var69 = self.global1;
            let var70 = self.func94(imports, var69); // wasm/helpers/index/rotateByteRight
            self.global1 = var70;
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/flags/getCarryFlag
    fn func96<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(4i32 as u32) as i32 & 1i32
    }
    // wasm/helpers/index/rotateByteLeftThroughCarry
    fn func97<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        (var1.wrapping_shl(1i32 as u32) & 255i32 | var2) & 255i32
    }
    // wasm/cpu/cpu/relativeJump
    fn func98<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global0;
        let var2 = var0;
        self.global0 = var1.wrapping_add(var2.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32)) & 65535i32;
        let var3 = self.global0;
        self.global0 = var3.wrapping_add(1i32) & 65535i32;
    }
    // wasm/helpers/index/rotateByteRightThroughCarry
    fn func99<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        ((var1 as u32).wrapping_shr(1i32 as u32) as i32 | var2.wrapping_shl(7i32 as u32) & 255i32) & 255i32
    }
    // wasm/cpu/opcodes/handleOpcode1x
    fn func100<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(16i32) {
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
                                                                        let var5 = self.global0;
                                                                        self.global0 = var5.wrapping_add(1i32) & 65535i32;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = var3;
                                                                    let var7 = self.func9(imports, var6); // wasm/helpers/index/splitHighByte
                                                                    self.global5 = var7;
                                                                    let var8 = var3;
                                                                    let var9 = self.func10(imports, var8); // wasm/helpers/index/splitLowByte
                                                                    self.global6 = var9;
                                                                    let var10 = self.global0;
                                                                    self.global0 = var10.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var11 = self.global5;
                                                                let var12 = self.global6;
                                                                let var13 = self.func84(imports, var11, var12); // wasm/helpers/index/concatenateBytes
                                                                let var14 = self.global1;
                                                                self.func71(imports, var13, var14); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var15 = self.global5;
                                                            let var16 = self.global6;
                                                            let var17 = self.func84(imports, var15, var16); // wasm/helpers/index/concatenateBytes
                                                            var0 = var17.wrapping_add(1i32) & 65535i32;
                                                            let var18 = var0;
                                                            let var19 = self.func9(imports, var18); // wasm/helpers/index/splitHighByte
                                                            self.global5 = var19;
                                                            let var20 = var0;
                                                            let var21 = self.func10(imports, var20); // wasm/helpers/index/splitLowByte
                                                            self.global6 = var21;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var22 = self.global5;
                                                        self.func87(imports, var22, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                        let var23 = self.global5;
                                                        self.global5 = var23.wrapping_add(1i32) & 255i32;
                                                        let var24 = self.global5;
                                                        if var24 != 0 {
                                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                        } else {
                                                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                        }
                                                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var25 = self.global5;
                                                    self.func87(imports, var25, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                    let var26 = self.global5;
                                                    self.global5 = var26.wrapping_sub(1i32) & 255i32;
                                                    let var27 = self.global5;
                                                    if var27 != 0 {
                                                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                    } else {
                                                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                    }
                                                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                    return 4i32;
                                                    break;
                                                }
                                                let var28 = var1;
                                                self.global5 = var28;
                                                let var29 = self.global0;
                                                self.global0 = var29.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            var0 = 0i32;
                                            let var30 = self.global1;
                                            if (var30 & 128i32 == 128i32) as i32 != 0 {
                                                var0 = 1i32;
                                            }
                                            let var31 = self.global1;
                                            let var32 = self.func97(imports, var31); // wasm/helpers/index/rotateByteLeftThroughCarry
                                            self.global1 = var32;
                                            let var33 = var0;
                                            if var33 != 0 {
                                                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            return 4i32;
                                            break;
                                        }
                                        let var34 = var1;
                                        self.func98(imports, var34); // wasm/cpu/cpu/relativeJump
                                        return 12i32;
                                        break;
                                    }
                                    let var35 = self.global7;
                                    let var36 = self.global8;
                                    let var37 = self.func84(imports, var35, var36); // wasm/helpers/index/concatenateBytes
                                    var0 = var37;
                                    let var38 = var0;
                                    let var39 = self.global5;
                                    let var40 = self.global6;
                                    let var41 = self.func84(imports, var39, var40); // wasm/helpers/index/concatenateBytes
                                    var1 = var41;
                                    let var42 = var1;
                                    self.func93(imports, var38, var42, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                    let var43 = var0;
                                    let var44 = var1;
                                    var0 = var43.wrapping_add(var44) & 65535i32;
                                    let var45 = var0;
                                    let var46 = self.func9(imports, var45); // wasm/helpers/index/splitHighByte
                                    self.global7 = var46;
                                    let var47 = var0;
                                    let var48 = self.func10(imports, var47); // wasm/helpers/index/splitLowByte
                                    self.global8 = var48;
                                    self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                    return 8i32;
                                    break;
                                }
                                let var49 = self.global5;
                                let var50 = self.global6;
                                let var51 = self.func84(imports, var49, var50); // wasm/helpers/index/concatenateBytes
                                let var52 = self.func75(imports, var51); // wasm/memory/load/eightBitLoadFromGBMemory
                                self.global1 = var52;
                                return 8i32;
                                break;
                            }
                            let var53 = self.global5;
                            let var54 = self.global6;
                            let var55 = self.func84(imports, var53, var54); // wasm/helpers/index/concatenateBytes
                            var0 = var55.wrapping_sub(1i32) & 65535i32;
                            let var56 = var0;
                            let var57 = self.func9(imports, var56); // wasm/helpers/index/splitHighByte
                            self.global5 = var57;
                            let var58 = var0;
                            let var59 = self.func10(imports, var58); // wasm/helpers/index/splitLowByte
                            self.global6 = var59;
                            return 8i32;
                            break;
                        }
                        let var60 = self.global6;
                        self.func87(imports, var60, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                        let var61 = self.global6;
                        self.global6 = var61.wrapping_add(1i32) & 255i32;
                        let var62 = self.global6;
                        if var62 != 0 {
                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                        } else {
                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                        }
                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                        return 4i32;
                        break;
                    }
                    let var63 = self.global6;
                    self.func87(imports, var63, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                    let var64 = self.global6;
                    self.global6 = var64.wrapping_sub(1i32) & 255i32;
                    let var65 = self.global6;
                    if var65 != 0 {
                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                    } else {
                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                    }
                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                    return 4i32;
                    break;
                }
                let var66 = var1;
                self.global6 = var66;
                let var67 = self.global0;
                self.global0 = var67.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            var0 = 0i32;
            let var68 = self.global1;
            if (var68 & 1i32 == 1i32) as i32 != 0 {
                var0 = 1i32;
            }
            let var69 = self.global1;
            let var70 = self.func99(imports, var69); // wasm/helpers/index/rotateByteRightThroughCarry
            self.global1 = var70;
            let var71 = var0;
            if var71 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/flags/getZeroFlag
    fn func101<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(7i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/flags/getHalfCarryFlag
    fn func102<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(5i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/flags/getSubtractFlag
    fn func103<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        (var0 as u32).wrapping_shr(6i32 as u32) as i32 & 1i32
    }
    // wasm/cpu/opcodes/handleOpcode2x
    fn func104<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(32i32) {
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
                                                                        let var5 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                                                        if var5 != 0 {
                                                                            let var6 = self.global0;
                                                                            self.global0 = var6.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            let var7 = var1;
                                                                            self.func98(imports, var7); // wasm/cpu/cpu/relativeJump
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var8 = var3;
                                                                    var0 = var8;
                                                                    let var9 = var0;
                                                                    let var10 = self.func9(imports, var9); // wasm/helpers/index/splitHighByte
                                                                    self.global7 = var10;
                                                                    let var11 = var0;
                                                                    let var12 = self.func10(imports, var11); // wasm/helpers/index/splitLowByte
                                                                    self.global8 = var12;
                                                                    let var13 = self.global0;
                                                                    self.global0 = var13.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var14 = self.global7;
                                                                let var15 = self.global8;
                                                                let var16 = self.func84(imports, var14, var15); // wasm/helpers/index/concatenateBytes
                                                                var0 = var16;
                                                                let var17 = var0;
                                                                let var18 = self.global1;
                                                                self.func71(imports, var17, var18); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                let var19 = var0;
                                                                var0 = var19.wrapping_add(1i32) & 65535i32;
                                                                let var20 = var0;
                                                                let var21 = self.func9(imports, var20); // wasm/helpers/index/splitHighByte
                                                                self.global7 = var21;
                                                                let var22 = var0;
                                                                let var23 = self.func10(imports, var22); // wasm/helpers/index/splitLowByte
                                                                self.global8 = var23;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var24 = self.global7;
                                                            let var25 = self.global8;
                                                            let var26 = self.func84(imports, var24, var25); // wasm/helpers/index/concatenateBytes
                                                            var0 = var26.wrapping_add(1i32) & 65535i32;
                                                            let var27 = var0;
                                                            let var28 = self.func9(imports, var27); // wasm/helpers/index/splitHighByte
                                                            self.global7 = var28;
                                                            let var29 = var0;
                                                            let var30 = self.func10(imports, var29); // wasm/helpers/index/splitLowByte
                                                            self.global8 = var30;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var31 = self.global7;
                                                        self.func87(imports, var31, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                        let var32 = self.global7;
                                                        self.global7 = var32.wrapping_add(1i32) & 255i32;
                                                        let var33 = self.global7;
                                                        if var33 != 0 {
                                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                        } else {
                                                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                        }
                                                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var34 = self.global7;
                                                    self.func87(imports, var34, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                    let var35 = self.global7;
                                                    self.global7 = var35.wrapping_sub(1i32) & 255i32;
                                                    let var36 = self.global7;
                                                    if var36 != 0 {
                                                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                    } else {
                                                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                    }
                                                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                    return 4i32;
                                                    break;
                                                }
                                                let var37 = var1;
                                                self.global7 = var37;
                                                let var38 = self.global0;
                                                self.global0 = var38.wrapping_add(1i32) & 65535i32;
                                                return 8i32;
                                                break;
                                            }
                                            var1 = 0i32;
                                            let var39 = self.func102(imports); // wasm/cpu/flags/getHalfCarryFlag
                                            if (var39 as u32 > 0i32 as u32) as i32 != 0 {
                                                var1 = 6i32;
                                            }
                                            let var40 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                            if (var40 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var41 = var1;
                                                var1 = (var41 | 96i32) & 255i32;
                                            }
                                            let var42 = self.func103(imports); // wasm/cpu/flags/getSubtractFlag
                                            let var43: i32;
                                            if (var42 as u32 > 0i32 as u32) as i32 != 0 {
                                                let var44 = self.global1;
                                                let var45 = var1;
                                                var43 = var44.wrapping_sub(var45) & 255i32;
                                            } else {
                                                let var46 = self.global1;
                                                if ((var46 & 15i32) as u32 > 9i32 as u32) as i32 != 0 {
                                                    let var47 = var1;
                                                    var1 = (var47 | 6i32) & 255i32;
                                                }
                                                let var48 = self.global1;
                                                if (var48 as u32 > 153i32 as u32) as i32 != 0 {
                                                    let var49 = var1;
                                                    var1 = (var49 | 96i32) & 255i32;
                                                }
                                                let var50 = self.global1;
                                                let var51 = var1;
                                                var43 = var50.wrapping_add(var51) & 255i32;
                                            }
                                            var0 = var43;
                                            let var52 = var0;
                                            if var52 != 0 {
                                                self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                            } else {
                                                self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                            }
                                            let var53 = var1;
                                            if var53 & 96i32 != 0 {
                                                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            } else {
                                                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
                                            }
                                            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            let var54 = var0;
                                            self.global1 = var54;
                                            return 4i32;
                                            break;
                                        }
                                        let var55 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                        if (var55 as u32 > 0i32 as u32) as i32 != 0 {
                                            let var56 = var1;
                                            self.func98(imports, var56); // wasm/cpu/cpu/relativeJump
                                            return 12i32;
                                        } else {
                                            let var57 = self.global0;
                                            self.global0 = var57.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var58 = self.global7;
                                    let var59 = self.global8;
                                    let var60 = self.func84(imports, var58, var59); // wasm/helpers/index/concatenateBytes
                                    var1 = var60;
                                    let var61 = var1;
                                    let var62 = var1;
                                    self.func93(imports, var61, var62, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                    let var63 = var1;
                                    var1 = var63.wrapping_mul(2i32) & 65535i32;
                                    let var64 = var1;
                                    let var65 = self.func9(imports, var64); // wasm/helpers/index/splitHighByte
                                    self.global7 = var65;
                                    let var66 = var1;
                                    let var67 = self.func10(imports, var66); // wasm/helpers/index/splitLowByte
                                    self.global8 = var67;
                                    self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                    return 8i32;
                                    break;
                                }
                                let var68 = self.global7;
                                let var69 = self.global8;
                                let var70 = self.func84(imports, var68, var69); // wasm/helpers/index/concatenateBytes
                                var1 = var70;
                                let var71 = var1;
                                let var72 = self.func75(imports, var71); // wasm/memory/load/eightBitLoadFromGBMemory
                                self.global1 = var72;
                                let var73 = var1;
                                var1 = var73.wrapping_add(1i32) & 65535i32;
                                let var74 = var1;
                                let var75 = self.func9(imports, var74); // wasm/helpers/index/splitHighByte
                                self.global7 = var75;
                                let var76 = var1;
                                let var77 = self.func10(imports, var76); // wasm/helpers/index/splitLowByte
                                self.global8 = var77;
                                return 8i32;
                                break;
                            }
                            let var78 = self.global7;
                            let var79 = self.global8;
                            let var80 = self.func84(imports, var78, var79); // wasm/helpers/index/concatenateBytes
                            var1 = var80.wrapping_sub(1i32) & 65535i32;
                            let var81 = var1;
                            let var82 = self.func9(imports, var81); // wasm/helpers/index/splitHighByte
                            self.global7 = var82;
                            let var83 = var1;
                            let var84 = self.func10(imports, var83); // wasm/helpers/index/splitLowByte
                            self.global8 = var84;
                            return 8i32;
                            break;
                        }
                        let var85 = self.global8;
                        self.func87(imports, var85, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                        let var86 = self.global8;
                        self.global8 = var86.wrapping_add(1i32) & 255i32;
                        let var87 = self.global8;
                        if var87 != 0 {
                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                        } else {
                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                        }
                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                        return 4i32;
                        break;
                    }
                    let var88 = self.global8;
                    self.func87(imports, var88, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                    let var89 = self.global8;
                    self.global8 = var89.wrapping_sub(1i32) & 255i32;
                    let var90 = self.global8;
                    if var90 != 0 {
                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                    } else {
                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                    }
                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                    return 4i32;
                    break;
                }
                let var91 = var1;
                self.global8 = var91;
                let var92 = self.global0;
                self.global0 = var92.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var93 = self.global1;
            self.global1 = (var93 ^ -1i32) & 255i32;
            self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
            self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcode3x
    fn func105<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(48i32) {
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
                                                                        let var5 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                                                        if var5 != 0 {
                                                                            let var6 = self.global0;
                                                                            self.global0 = var6.wrapping_add(1i32) & 65535i32;
                                                                            return 8i32;
                                                                        } else {
                                                                            let var7 = var1;
                                                                            self.func98(imports, var7); // wasm/cpu/cpu/relativeJump
                                                                            return 12i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var8 = var3;
                                                                    self.global9 = var8;
                                                                    let var9 = self.global0;
                                                                    self.global0 = var9.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var10 = self.global7;
                                                                let var11 = self.global8;
                                                                let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                                var0 = var12;
                                                                let var13 = var0;
                                                                let var14 = self.global1;
                                                                self.func71(imports, var13, var14); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                let var15 = var0;
                                                                var0 = var15.wrapping_sub(1i32) & 65535i32;
                                                                let var16 = var0;
                                                                let var17 = self.func9(imports, var16); // wasm/helpers/index/splitHighByte
                                                                self.global7 = var17;
                                                                let var18 = var0;
                                                                let var19 = self.func10(imports, var18); // wasm/helpers/index/splitLowByte
                                                                self.global8 = var19;
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var20 = self.global9;
                                                            self.global9 = var20.wrapping_add(1i32) & 65535i32;
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var21 = self.global7;
                                                        let var22 = self.global8;
                                                        let var23 = self.func84(imports, var21, var22); // wasm/helpers/index/concatenateBytes
                                                        var0 = var23;
                                                        let var24 = var0;
                                                        let var25 = self.func75(imports, var24); // wasm/memory/load/eightBitLoadFromGBMemory
                                                        var1 = var25;
                                                        let var26 = var1;
                                                        var2 = 1i32;
                                                        let var27 = var2;
                                                        self.func87(imports, var26, var27); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                        let var28 = var1;
                                                        var1 = var28.wrapping_add(1i32) & 255i32;
                                                        let var29 = var1;
                                                        if var29 != 0 {
                                                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                        } else {
                                                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                        }
                                                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                                        let var30 = var0;
                                                        let var31 = var1;
                                                        self.func71(imports, var30, var31); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var32 = self.global7;
                                                    let var33 = self.global8;
                                                    let var34 = self.func84(imports, var32, var33); // wasm/helpers/index/concatenateBytes
                                                    var2 = var34;
                                                    let var35 = var2;
                                                    let var36 = self.func75(imports, var35); // wasm/memory/load/eightBitLoadFromGBMemory
                                                    var1 = var36;
                                                    let var37 = var1;
                                                    self.func87(imports, var37, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                                                    let var38 = var1;
                                                    var1 = var38.wrapping_sub(1i32) & 255i32;
                                                    let var39 = var1;
                                                    if var39 != 0 {
                                                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                                    } else {
                                                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                                                    }
                                                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                                                    let var40 = var2;
                                                    let var41 = var1;
                                                    self.func71(imports, var40, var41); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                    return 12i32;
                                                    break;
                                                }
                                                let var42 = self.global7;
                                                let var43 = self.global8;
                                                let var44 = self.func84(imports, var42, var43); // wasm/helpers/index/concatenateBytes
                                                let var45 = var1;
                                                self.func71(imports, var44, var45); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                let var46 = self.global0;
                                                self.global0 = var46.wrapping_add(1i32) & 65535i32;
                                                return 12i32;
                                                break;
                                            }
                                            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
                                            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
                                            return 4i32;
                                            break;
                                        }
                                        let var47 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                        if (var47 == 1i32) as i32 != 0 {
                                            let var48 = var1;
                                            self.func98(imports, var48); // wasm/cpu/cpu/relativeJump
                                            return 12i32;
                                        } else {
                                            let var49 = self.global0;
                                            self.global0 = var49.wrapping_add(1i32) & 65535i32;
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var50 = self.global7;
                                    let var51 = self.global8;
                                    let var52 = self.func84(imports, var50, var51); // wasm/helpers/index/concatenateBytes
                                    var1 = var52;
                                    let var53 = var1;
                                    let var54 = self.global9;
                                    self.func93(imports, var53, var54, 0i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                    let var55 = var1;
                                    let var56 = self.global9;
                                    var2 = var55.wrapping_add(var56) & 65535i32;
                                    let var57 = var2;
                                    let var58 = self.func9(imports, var57); // wasm/helpers/index/splitHighByte
                                    self.global7 = var58;
                                    let var59 = var2;
                                    let var60 = self.func10(imports, var59); // wasm/helpers/index/splitLowByte
                                    self.global8 = var60;
                                    self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                    return 8i32;
                                    break;
                                }
                                let var61 = self.global7;
                                let var62 = self.global8;
                                let var63 = self.func84(imports, var61, var62); // wasm/helpers/index/concatenateBytes
                                var2 = var63;
                                let var64 = var2;
                                let var65 = self.func75(imports, var64); // wasm/memory/load/eightBitLoadFromGBMemory
                                self.global1 = var65;
                                let var66 = var2;
                                var2 = var66.wrapping_sub(1i32) & 65535i32;
                                let var67 = var2;
                                let var68 = self.func9(imports, var67); // wasm/helpers/index/splitHighByte
                                self.global7 = var68;
                                let var69 = var2;
                                let var70 = self.func10(imports, var69); // wasm/helpers/index/splitLowByte
                                self.global8 = var70;
                                return 8i32;
                                break;
                            }
                            let var71 = self.global9;
                            self.global9 = var71.wrapping_sub(1i32) & 65535i32;
                            return 8i32;
                            break;
                        }
                        let var72 = self.global1;
                        self.func87(imports, var72, 1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                        let var73 = self.global1;
                        self.global1 = var73.wrapping_add(1i32) & 255i32;
                        let var74 = self.global1;
                        if var74 != 0 {
                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                        } else {
                            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                        }
                        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                        return 4i32;
                        break;
                    }
                    let var75 = self.global1;
                    self.func87(imports, var75, -1i32); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
                    let var76 = self.global1;
                    self.global1 = var76.wrapping_sub(1i32) & 255i32;
                    let var77 = self.global1;
                    if var77 != 0 {
                        self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                    } else {
                        self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
                    }
                    self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
                    return 4i32;
                    break;
                }
                let var78 = var1;
                self.global1 = var78;
                let var79 = self.global0;
                self.global0 = var79.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
            let var80 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
            if (var80 as u32 > 0i32 as u32) as i32 != 0 {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            }
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcode4x
    fn func106<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(64i32) {
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
                                                                    let var5 = self.global4;
                                                                    self.global3 = var5;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var6 = self.global5;
                                                                self.global3 = var6;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global6;
                                                            self.global3 = var7;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global7;
                                                        self.global3 = var8;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global8;
                                                    self.global3 = var9;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global7;
                                                let var11 = self.global8;
                                                let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                let var13 = self.func75(imports, var12); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.global3 = var13;
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global1;
                                            self.global3 = var14;
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global3;
                                        self.global4 = var15;
                                        return 4i32;
                                        break;
                                    }
                                    return 4i32;
                                    break;
                                }
                                let var16 = self.global5;
                                self.global4 = var16;
                                return 4i32;
                                break;
                            }
                            let var17 = self.global6;
                            self.global4 = var17;
                            return 4i32;
                            break;
                        }
                        let var18 = self.global7;
                        self.global4 = var18;
                        return 4i32;
                        break;
                    }
                    let var19 = self.global8;
                    self.global4 = var19;
                    return 4i32;
                    break;
                }
                let var20 = self.global7;
                let var21 = self.global8;
                let var22 = self.func84(imports, var20, var21); // wasm/helpers/index/concatenateBytes
                let var23 = self.func75(imports, var22); // wasm/memory/load/eightBitLoadFromGBMemory
                self.global4 = var23;
                return 8i32;
                break;
            }
            let var24 = self.global1;
            self.global4 = var24;
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcode5x
    fn func107<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(80i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.global5 = var5;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.global5 = var6;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var7 = self.global6;
                                                            self.global5 = var7;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var8 = self.global7;
                                                        self.global5 = var8;
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global8;
                                                    self.global5 = var9;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global7;
                                                let var11 = self.global8;
                                                let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                let var13 = self.func75(imports, var12); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.global5 = var13;
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global1;
                                            self.global5 = var14;
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global3;
                                        self.global6 = var15;
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global4;
                                    self.global6 = var16;
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global5;
                                self.global6 = var17;
                                return 4i32;
                                break;
                            }
                            return 4i32;
                            break;
                        }
                        let var18 = self.global7;
                        self.global6 = var18;
                        return 4i32;
                        break;
                    }
                    let var19 = self.global8;
                    self.global6 = var19;
                    return 4i32;
                    break;
                }
                let var20 = self.global7;
                let var21 = self.global8;
                let var22 = self.func84(imports, var20, var21); // wasm/helpers/index/concatenateBytes
                let var23 = self.func75(imports, var22); // wasm/memory/load/eightBitLoadFromGBMemory
                self.global6 = var23;
                return 4i32;
                break;
            }
            let var24 = self.global1;
            self.global6 = var24;
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcode6x
    fn func108<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(96i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.global7 = var5;
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.global7 = var6;
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global5;
                                                                self.global7 = var7;
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var8 = self.global6;
                                                            self.global7 = var8;
                                                            return 4i32;
                                                            break;
                                                        }
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var9 = self.global8;
                                                    self.global7 = var9;
                                                    return 4i32;
                                                    break;
                                                }
                                                let var10 = self.global7;
                                                let var11 = self.global8;
                                                let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                let var13 = self.func75(imports, var12); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.global7 = var13;
                                                return 8i32;
                                                break;
                                            }
                                            let var14 = self.global1;
                                            self.global7 = var14;
                                            return 4i32;
                                            break;
                                        }
                                        let var15 = self.global3;
                                        self.global8 = var15;
                                        return 4i32;
                                        break;
                                    }
                                    let var16 = self.global4;
                                    self.global8 = var16;
                                    return 4i32;
                                    break;
                                }
                                let var17 = self.global5;
                                self.global8 = var17;
                                return 4i32;
                                break;
                            }
                            let var18 = self.global6;
                            self.global8 = var18;
                            return 4i32;
                            break;
                        }
                        let var19 = self.global7;
                        self.global8 = var19;
                        return 4i32;
                        break;
                    }
                    return 4i32;
                    break;
                }
                let var20 = self.global7;
                let var21 = self.global8;
                let var22 = self.func84(imports, var20, var21); // wasm/helpers/index/concatenateBytes
                let var23 = self.func75(imports, var22); // wasm/memory/load/eightBitLoadFromGBMemory
                self.global8 = var23;
                return 8i32;
                break;
            }
            let var24 = self.global1;
            self.global8 = var24;
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcode7x
    fn func109<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(112i32) {
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
                                                                        let var5 = self.global7;
                                                                        let var6 = self.global8;
                                                                        let var7 = self.func84(imports, var5, var6); // wasm/helpers/index/concatenateBytes
                                                                        let var8 = self.global3;
                                                                        self.func71(imports, var7, var8); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                        return 8i32;
                                                                        break;
                                                                    }
                                                                    let var9 = self.global7;
                                                                    let var10 = self.global8;
                                                                    let var11 = self.func84(imports, var9, var10); // wasm/helpers/index/concatenateBytes
                                                                    let var12 = self.global4;
                                                                    self.func71(imports, var11, var12); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                    return 8i32;
                                                                    break;
                                                                }
                                                                let var13 = self.global7;
                                                                let var14 = self.global8;
                                                                let var15 = self.func84(imports, var13, var14); // wasm/helpers/index/concatenateBytes
                                                                let var16 = self.global5;
                                                                self.func71(imports, var15, var16); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                                return 8i32;
                                                                break;
                                                            }
                                                            let var17 = self.global7;
                                                            let var18 = self.global8;
                                                            let var19 = self.func84(imports, var17, var18); // wasm/helpers/index/concatenateBytes
                                                            let var20 = self.global6;
                                                            self.func71(imports, var19, var20); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                            return 8i32;
                                                            break;
                                                        }
                                                        let var21 = self.global7;
                                                        let var22 = self.global8;
                                                        let var23 = self.func84(imports, var21, var22); // wasm/helpers/index/concatenateBytes
                                                        let var24 = self.global7;
                                                        self.func71(imports, var23, var24); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                        return 8i32;
                                                        break;
                                                    }
                                                    let var25 = self.global7;
                                                    let var26 = self.global8;
                                                    let var27 = self.func84(imports, var25, var26); // wasm/helpers/index/concatenateBytes
                                                    let var28 = self.global8;
                                                    self.func71(imports, var27, var28); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                    return 8i32;
                                                    break;
                                                }
                                                self.global69 = 1i32;
                                                return 4i32;
                                                break;
                                            }
                                            let var29 = self.global7;
                                            let var30 = self.global8;
                                            let var31 = self.func84(imports, var29, var30); // wasm/helpers/index/concatenateBytes
                                            let var32 = self.global1;
                                            self.func71(imports, var31, var32); // wasm/memory/store/eightBitStoreIntoGBMemory
                                            return 8i32;
                                            break;
                                        }
                                        let var33 = self.global3;
                                        self.global1 = var33;
                                        return 4i32;
                                        break;
                                    }
                                    let var34 = self.global4;
                                    self.global1 = var34;
                                    return 4i32;
                                    break;
                                }
                                let var35 = self.global5;
                                self.global1 = var35;
                                return 4i32;
                                break;
                            }
                            let var36 = self.global6;
                            self.global1 = var36;
                            return 4i32;
                            break;
                        }
                        let var37 = self.global7;
                        self.global1 = var37;
                        return 4i32;
                        break;
                    }
                    let var38 = self.global8;
                    self.global1 = var38;
                    return 4i32;
                    break;
                }
                let var39 = self.global7;
                let var40 = self.global8;
                let var41 = self.func84(imports, var39, var40); // wasm/helpers/index/concatenateBytes
                let var42 = self.func75(imports, var41); // wasm/memory/load/eightBitLoadFromGBMemory
                self.global1 = var42;
                return 4i32;
                break;
            }
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/flags/checkAndSetEightBitCarryFlag
    fn func110<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let mut var2: i32 = 0;
        let var3 = var1;
        if (var3 >= 0i32) as i32 != 0 {
            let var4 = var0;
            let var5 = var0;
            let var6 = var1;
            if (var4 as u32 > (var5.wrapping_add(var6 & 255i32) & 255i32) as u32) as i32 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        } else {
            let var7 = var1;
            var2 = var7;
            let var8 = var2;
            let var9 = var2;
            let var10 = var2;
            let var11 = var0;
            if ((if (var10 > 0i32) as i32 != 0 { var8 } else { 0i32.wrapping_sub(var9) }) > var11) as i32 != 0 {
                self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
            } else {
                self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
            }
        }
    }
    // wasm/cpu/instructions/addARegister
    fn func111<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.func87(imports, var1, var2); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var3 = self.global1;
        let var4 = var0;
        self.func110(imports, var3, var4); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var5 = self.global1;
        let var6 = var0;
        self.global1 = var5.wrapping_add(var6) & 255i32;
        let var7 = self.global1;
        if var7 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/addAThroughCarryRegister
    fn func112<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        let var4 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        var1 = var2.wrapping_add(var3).wrapping_add(var4) & 255i32;
        let var5 = self.global1;
        let var6 = var0;
        let var7 = var1;
        if (var5 ^ var6 ^ var7) & 16i32 != 0 {
            self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        } else {
            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        }
        let var8 = self.global1;
        let var9 = var0;
        let var10 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        if ((var8.wrapping_add(var9).wrapping_add(var10) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var11 = var1;
        self.global1 = var11;
        let var12 = self.global1;
        if var12 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/opcodes/handleOpcode8x
    fn func113<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(128i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.func111(imports, var5); // wasm/cpu/instructions/addARegister
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.func111(imports, var6); // wasm/cpu/instructions/addARegister
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global5;
                                                                self.func111(imports, var7); // wasm/cpu/instructions/addARegister
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var8 = self.global6;
                                                            self.func111(imports, var8); // wasm/cpu/instructions/addARegister
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var9 = self.global7;
                                                        self.func111(imports, var9); // wasm/cpu/instructions/addARegister
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var10 = self.global8;
                                                    self.func111(imports, var10); // wasm/cpu/instructions/addARegister
                                                    return 4i32;
                                                    break;
                                                }
                                                let var11 = self.global7;
                                                let var12 = self.global8;
                                                let var13 = self.func84(imports, var11, var12); // wasm/helpers/index/concatenateBytes
                                                let var14 = self.func75(imports, var13); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.func111(imports, var14); // wasm/cpu/instructions/addARegister
                                                return 8i32;
                                                break;
                                            }
                                            let var15 = self.global1;
                                            self.func111(imports, var15); // wasm/cpu/instructions/addARegister
                                            return 4i32;
                                            break;
                                        }
                                        let var16 = self.global3;
                                        self.func112(imports, var16); // wasm/cpu/instructions/addAThroughCarryRegister
                                        return 4i32;
                                        break;
                                    }
                                    let var17 = self.global4;
                                    self.func112(imports, var17); // wasm/cpu/instructions/addAThroughCarryRegister
                                    return 4i32;
                                    break;
                                }
                                let var18 = self.global5;
                                self.func112(imports, var18); // wasm/cpu/instructions/addAThroughCarryRegister
                                return 4i32;
                                break;
                            }
                            let var19 = self.global6;
                            self.func112(imports, var19); // wasm/cpu/instructions/addAThroughCarryRegister
                            return 4i32;
                            break;
                        }
                        let var20 = self.global7;
                        self.func112(imports, var20); // wasm/cpu/instructions/addAThroughCarryRegister
                        return 4i32;
                        break;
                    }
                    let var21 = self.global8;
                    self.func112(imports, var21); // wasm/cpu/instructions/addAThroughCarryRegister
                    return 4i32;
                    break;
                }
                let var22 = self.global7;
                let var23 = self.global8;
                let var24 = self.func84(imports, var22, var23); // wasm/helpers/index/concatenateBytes
                let var25 = self.func75(imports, var24); // wasm/memory/load/eightBitLoadFromGBMemory
                self.func112(imports, var25); // wasm/cpu/instructions/addAThroughCarryRegister
                return 8i32;
                break;
            }
            let var26 = self.global1;
            self.func112(imports, var26); // wasm/cpu/instructions/addAThroughCarryRegister
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/instructions/subARegister
    fn func114<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        var1 = var3.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        let var4 = var1;
        self.func87(imports, var2, var4); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var5 = self.global1;
        let var6 = var1;
        self.func110(imports, var5, var6); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var7 = self.global1;
        let var8 = var0;
        self.global1 = var7.wrapping_sub(var8) & 255i32;
        let var9 = self.global1;
        if var9 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/instructions/subAThroughCarryRegister
    fn func115<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        let var4 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        var1 = var2.wrapping_sub(var3).wrapping_sub(var4) & 255i32;
        let var5 = self.global1;
        let var6 = var0;
        let var7 = var1;
        if (var5 ^ var6 ^ var7) & 16i32 != 0 {
            self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        } else {
            self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        }
        let var8 = self.global1;
        let var9 = var0;
        let var10 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
        if ((var8.wrapping_sub(var9).wrapping_sub(var10) & 256i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var11 = var1;
        self.global1 = var11;
        let var12 = self.global1;
        if var12 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/opcodes/handleOpcode9x
    fn func116<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(144i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.func114(imports, var5); // wasm/cpu/instructions/subARegister
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.func114(imports, var6); // wasm/cpu/instructions/subARegister
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global5;
                                                                self.func114(imports, var7); // wasm/cpu/instructions/subARegister
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var8 = self.global6;
                                                            self.func114(imports, var8); // wasm/cpu/instructions/subARegister
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var9 = self.global7;
                                                        self.func114(imports, var9); // wasm/cpu/instructions/subARegister
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var10 = self.global8;
                                                    self.func114(imports, var10); // wasm/cpu/instructions/subARegister
                                                    return 4i32;
                                                    break;
                                                }
                                                let var11 = self.global7;
                                                let var12 = self.global8;
                                                let var13 = self.func84(imports, var11, var12); // wasm/helpers/index/concatenateBytes
                                                let var14 = self.func75(imports, var13); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.func114(imports, var14); // wasm/cpu/instructions/subARegister
                                                return 8i32;
                                                break;
                                            }
                                            let var15 = self.global1;
                                            self.func114(imports, var15); // wasm/cpu/instructions/subARegister
                                            return 4i32;
                                            break;
                                        }
                                        let var16 = self.global3;
                                        self.func115(imports, var16); // wasm/cpu/instructions/subAThroughCarryRegister
                                        return 4i32;
                                        break;
                                    }
                                    let var17 = self.global4;
                                    self.func115(imports, var17); // wasm/cpu/instructions/subAThroughCarryRegister
                                    return 4i32;
                                    break;
                                }
                                let var18 = self.global5;
                                self.func115(imports, var18); // wasm/cpu/instructions/subAThroughCarryRegister
                                return 4i32;
                                break;
                            }
                            let var19 = self.global6;
                            self.func115(imports, var19); // wasm/cpu/instructions/subAThroughCarryRegister
                            return 4i32;
                            break;
                        }
                        let var20 = self.global7;
                        self.func115(imports, var20); // wasm/cpu/instructions/subAThroughCarryRegister
                        return 4i32;
                        break;
                    }
                    let var21 = self.global8;
                    self.func115(imports, var21); // wasm/cpu/instructions/subAThroughCarryRegister
                    return 4i32;
                    break;
                }
                let var22 = self.global7;
                let var23 = self.global8;
                let var24 = self.func84(imports, var22, var23); // wasm/helpers/index/concatenateBytes
                let var25 = self.func75(imports, var24); // wasm/memory/load/eightBitLoadFromGBMemory
                self.func115(imports, var25); // wasm/cpu/instructions/subAThroughCarryRegister
                return 8i32;
                break;
            }
            let var26 = self.global1;
            self.func115(imports, var26); // wasm/cpu/instructions/subAThroughCarryRegister
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/instructions/andARegister
    fn func117<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = var1 & var2 & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/instructions/xorARegister
    fn func118<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = (var1 ^ var2) & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/opcodes/handleOpcodeAx
    fn func119<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(160i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.func117(imports, var5); // wasm/cpu/instructions/andARegister
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.func117(imports, var6); // wasm/cpu/instructions/andARegister
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global5;
                                                                self.func117(imports, var7); // wasm/cpu/instructions/andARegister
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var8 = self.global6;
                                                            self.func117(imports, var8); // wasm/cpu/instructions/andARegister
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var9 = self.global7;
                                                        self.func117(imports, var9); // wasm/cpu/instructions/andARegister
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var10 = self.global8;
                                                    self.func117(imports, var10); // wasm/cpu/instructions/andARegister
                                                    return 4i32;
                                                    break;
                                                }
                                                let var11 = self.global7;
                                                let var12 = self.global8;
                                                let var13 = self.func84(imports, var11, var12); // wasm/helpers/index/concatenateBytes
                                                let var14 = self.func75(imports, var13); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.func117(imports, var14); // wasm/cpu/instructions/andARegister
                                                return 8i32;
                                                break;
                                            }
                                            let var15 = self.global1;
                                            self.func117(imports, var15); // wasm/cpu/instructions/andARegister
                                            return 4i32;
                                            break;
                                        }
                                        let var16 = self.global3;
                                        self.func118(imports, var16); // wasm/cpu/instructions/xorARegister
                                        return 4i32;
                                        break;
                                    }
                                    let var17 = self.global4;
                                    self.func118(imports, var17); // wasm/cpu/instructions/xorARegister
                                    return 4i32;
                                    break;
                                }
                                let var18 = self.global5;
                                self.func118(imports, var18); // wasm/cpu/instructions/xorARegister
                                return 4i32;
                                break;
                            }
                            let var19 = self.global6;
                            self.func118(imports, var19); // wasm/cpu/instructions/xorARegister
                            return 4i32;
                            break;
                        }
                        let var20 = self.global7;
                        self.func118(imports, var20); // wasm/cpu/instructions/xorARegister
                        return 4i32;
                        break;
                    }
                    let var21 = self.global8;
                    self.func118(imports, var21); // wasm/cpu/instructions/xorARegister
                    return 4i32;
                    break;
                }
                let var22 = self.global7;
                let var23 = self.global8;
                let var24 = self.func84(imports, var22, var23); // wasm/helpers/index/concatenateBytes
                let var25 = self.func75(imports, var24); // wasm/memory/load/eightBitLoadFromGBMemory
                self.func118(imports, var25); // wasm/cpu/instructions/xorARegister
                return 8i32;
                break;
            }
            let var26 = self.global1;
            self.func118(imports, var26); // wasm/cpu/instructions/xorARegister
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/instructions/orARegister
    fn func120<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = self.global1;
        let var2 = var0;
        self.global1 = (var1 | var2) & 255i32;
        let var3 = self.global1;
        if var3 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
    }
    // wasm/cpu/instructions/cpARegister
    fn func121<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.global1;
        let var3 = var0;
        var1 = var3.wrapping_mul(-1i32).wrapping_shl(16i32 as u32).wrapping_shr(16i32 as u32);
        let var4 = var1;
        self.func87(imports, var2, var4); // wasm/cpu/flags/checkAndSetEightBitHalfCarryFlag
        let var5 = self.global1;
        let var6 = var1;
        self.func110(imports, var5, var6); // wasm/cpu/flags/checkAndSetEightBitCarryFlag
        let var7 = self.global1;
        let var8 = var1;
        if var7.wrapping_add(var8) & 65535i32 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 1i32); // wasm/cpu/flags/setSubtractFlag
    }
    // wasm/cpu/opcodes/handleOpcodeBx
    fn func122<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var4 = var0;
                                                                                match var4.wrapping_sub(176i32) {
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
                                                                        let var5 = self.global3;
                                                                        self.func120(imports, var5); // wasm/cpu/instructions/orARegister
                                                                        return 4i32;
                                                                        break;
                                                                    }
                                                                    let var6 = self.global4;
                                                                    self.func120(imports, var6); // wasm/cpu/instructions/orARegister
                                                                    return 4i32;
                                                                    break;
                                                                }
                                                                let var7 = self.global5;
                                                                self.func120(imports, var7); // wasm/cpu/instructions/orARegister
                                                                return 4i32;
                                                                break;
                                                            }
                                                            let var8 = self.global6;
                                                            self.func120(imports, var8); // wasm/cpu/instructions/orARegister
                                                            return 4i32;
                                                            break;
                                                        }
                                                        let var9 = self.global7;
                                                        self.func120(imports, var9); // wasm/cpu/instructions/orARegister
                                                        return 4i32;
                                                        break;
                                                    }
                                                    let var10 = self.global8;
                                                    self.func120(imports, var10); // wasm/cpu/instructions/orARegister
                                                    return 4i32;
                                                    break;
                                                }
                                                let var11 = self.global7;
                                                let var12 = self.global8;
                                                let var13 = self.func84(imports, var11, var12); // wasm/helpers/index/concatenateBytes
                                                let var14 = self.func75(imports, var13); // wasm/memory/load/eightBitLoadFromGBMemory
                                                self.func120(imports, var14); // wasm/cpu/instructions/orARegister
                                                return 8i32;
                                                break;
                                            }
                                            let var15 = self.global1;
                                            self.func120(imports, var15); // wasm/cpu/instructions/orARegister
                                            return 4i32;
                                            break;
                                        }
                                        let var16 = self.global3;
                                        self.func121(imports, var16); // wasm/cpu/instructions/cpARegister
                                        return 4i32;
                                        break;
                                    }
                                    let var17 = self.global4;
                                    self.func121(imports, var17); // wasm/cpu/instructions/cpARegister
                                    return 4i32;
                                    break;
                                }
                                let var18 = self.global5;
                                self.func121(imports, var18); // wasm/cpu/instructions/cpARegister
                                return 4i32;
                                break;
                            }
                            let var19 = self.global6;
                            self.func121(imports, var19); // wasm/cpu/instructions/cpARegister
                            return 4i32;
                            break;
                        }
                        let var20 = self.global7;
                        self.func121(imports, var20); // wasm/cpu/instructions/cpARegister
                        return 4i32;
                        break;
                    }
                    let var21 = self.global8;
                    self.func121(imports, var21); // wasm/cpu/instructions/cpARegister
                    return 4i32;
                    break;
                }
                let var22 = self.global7;
                let var23 = self.global8;
                let var24 = self.func84(imports, var22, var23); // wasm/helpers/index/concatenateBytes
                let var25 = self.func75(imports, var24); // wasm/memory/load/eightBitLoadFromGBMemory
                self.func121(imports, var25); // wasm/cpu/instructions/cpARegister
                return 8i32;
                break;
            }
            let var26 = self.global1;
            self.func121(imports, var26); // wasm/cpu/instructions/cpARegister
            return 4i32;
            break;
        }
        -1i32
    }
    // wasm/memory/load/sixteenBitLoadFromGBMemory
    fn func123<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let var3: i32;
        'label0: loop {
            let var4 = var0;
            let var5 = self.func74(imports, var4); // wasm/memory/readTraps/checkReadTraps
            var1 = var5;
            let var6 = var1;
            if (var6 == -1i32) as i32 != 0 {
                let var7 = var0;
                let var8 = self.func12(imports, var7); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
                var3 = var8;
                break 'label0;
            }
            let var9 = var1;
            var3 = var9 & 255i32;
            break;
        }
        var1 = var3;
        let var10: i32;
        'label1: loop {
            let var11 = var0;
            var0 = var11.wrapping_add(1i32) & 65535i32;
            let var12 = var0;
            let var13 = self.func74(imports, var12); // wasm/memory/readTraps/checkReadTraps
            var2 = var13;
            let var14 = var2;
            if (var14 == -1i32) as i32 != 0 {
                let var15 = var0;
                let var16 = self.func12(imports, var15); // wasm/memory/load/_eightBitLoadFromWasmBoyMemory
                var10 = var16;
                break 'label1;
            }
            let var17 = var2;
            var10 = var17 & 255i32;
            break;
        }
        var0 = var10;
        let var18 = var0;
        let var19 = var1;
        let var20 = self.func84(imports, var18, var19); // wasm/helpers/index/concatenateBytes
        var20
    }
    // wasm/cpu/instructions/rotateRegisterLeft
    fn func124<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        if (var1 & 128i32 == 128i32) as i32 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var2 = var0;
        let var3 = self.func91(imports, var2); // wasm/helpers/index/rotateByteLeft
        var0 = var3;
        let var4 = var0;
        if var4 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var0;
        var5
    }
    // wasm/cpu/instructions/rotateRegisterRight
    fn func125<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        if ((var1 & 1i32) as u32 > 0i32 as u32) as i32 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var2 = var0;
        let var3 = self.func94(imports, var2); // wasm/helpers/index/rotateByteRight
        var0 = var3;
        let var4 = var0;
        if var4 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var0;
        var5
    }
    // wasm/cpu/instructions/rotateRegisterLeftThroughCarry
    fn func126<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        let var4 = self.func97(imports, var3); // wasm/helpers/index/rotateByteLeftThroughCarry
        var0 = var4;
        let var5 = var1;
        if var5 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        if var6 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var7 = var0;
        var7
    }
    // wasm/cpu/instructions/rotateRegisterRightThroughCarry
    fn func127<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        let var4 = self.func99(imports, var3); // wasm/helpers/index/rotateByteRightThroughCarry
        var0 = var4;
        let var5 = var1;
        if var5 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        if var6 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var7 = var0;
        var7
    }
    // wasm/cpu/instructions/shiftLeftRegister
    fn func128<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 128i32 == 128i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        var0 = var3.wrapping_shl(1i32 as u32) & 255i32;
        let var4 = var1;
        if var4 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var5 = var0;
        if var5 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var6 = var0;
        var6
    }
    // wasm/cpu/instructions/shiftRightArithmeticRegister
    fn func129<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var9 = var2;
        if var9 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var10 = var0;
        var10
    }
    // wasm/cpu/instructions/swapNibblesOnRegister
    fn func130<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = var0;
        var0 = (var1 & 15i32).wrapping_shl(4i32 as u32) | ((var2 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32;
        let var3 = var0;
        if var3 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        let var4 = var0;
        var4
    }
    // wasm/cpu/instructions/shiftRightLogicalRegister
    fn func131<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let mut var1: i32 = 0;
        let var2 = var0;
        if (var2 & 1i32 == 1i32) as i32 != 0 {
            var1 = 1i32;
        }
        let var3 = var0;
        var0 = (var3 as u32).wrapping_shr(1i32 as u32) as i32;
        let var4 = var0;
        if var4 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 0i32); // wasm/cpu/flags/setHalfCarryFlag
        let var5 = var1;
        if var5 != 0 {
            self.func90(imports, 1i32); // wasm/cpu/flags/setCarryFlag
        } else {
            self.func90(imports, 0i32); // wasm/cpu/flags/setCarryFlag
        }
        let var6 = var0;
        var6
    }
    // wasm/cpu/instructions/testBitOnRegister
    fn func132<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        if var2 & 1i32.wrapping_shl(var3 as u32) & 255i32 != 0 {
            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
        } else {
            self.func88(imports, 1i32); // wasm/cpu/flags/setZeroFlag
        }
        self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
        self.func86(imports, 1i32); // wasm/cpu/flags/setHalfCarryFlag
        let var4 = var1;
        var4
    }
    // wasm/cpu/instructions/setBitOnRegister
    fn func133<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
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
    fn func134<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                                                let var6 = var0;
                                                var5 = (var6 as u32).wrapping_rem(8i32 as u32) as i32;
                                                let var7 = var5;
                                                match var7 {
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
                                        let var8 = self.global3;
                                        var1 = var8;
                                        break 'label0;
                                        break;
                                    }
                                    let var9 = self.global4;
                                    var1 = var9;
                                    break 'label0;
                                    break;
                                }
                                let var10 = self.global5;
                                var1 = var10;
                                break 'label0;
                                break;
                            }
                            let var11 = self.global6;
                            var1 = var11;
                            break 'label0;
                            break;
                        }
                        let var12 = self.global7;
                        var1 = var12;
                        break 'label0;
                        break;
                    }
                    let var13 = self.global8;
                    var1 = var13;
                    break 'label0;
                    break;
                }
                let var14 = self.global7;
                let var15 = self.global8;
                let var16 = self.func84(imports, var14, var15); // wasm/helpers/index/concatenateBytes
                let var17 = self.func75(imports, var16); // wasm/memory/load/eightBitLoadFromGBMemory
                var1 = var17;
                break 'label0;
                break;
            }
            let var18 = self.global1;
            var1 = var18;
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
                                                                                let var19 = var0;
                                                                                match ((var19 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32 {
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
                                                                        let var20 = var0;
                                                                        if (var20 as u32 <= 7i32 as u32) as i32 != 0 {
                                                                            let var21 = var1;
                                                                            let var22 = self.func124(imports, var21); // wasm/cpu/instructions/rotateRegisterLeft
                                                                            var2 = var22;
                                                                            var3 = 1i32;
                                                                        } else {
                                                                            let var23 = var0;
                                                                            if (var23 as u32 <= 15i32 as u32) as i32 != 0 {
                                                                                let var24 = var1;
                                                                                let var25 = self.func125(imports, var24); // wasm/cpu/instructions/rotateRegisterRight
                                                                                var2 = var25;
                                                                                var3 = 1i32;
                                                                            }
                                                                        }
                                                                        break 'label10;
                                                                        break;
                                                                    }
                                                                    let var26 = var0;
                                                                    if (var26 as u32 <= 23i32 as u32) as i32 != 0 {
                                                                        let var27 = var1;
                                                                        let var28 = self.func126(imports, var27); // wasm/cpu/instructions/rotateRegisterLeftThroughCarry
                                                                        var2 = var28;
                                                                        var3 = 1i32;
                                                                    } else {
                                                                        let var29 = var0;
                                                                        if (var29 as u32 <= 31i32 as u32) as i32 != 0 {
                                                                            let var30 = var1;
                                                                            let var31 = self.func127(imports, var30); // wasm/cpu/instructions/rotateRegisterRightThroughCarry
                                                                            var2 = var31;
                                                                            var3 = 1i32;
                                                                        }
                                                                    }
                                                                    break 'label10;
                                                                    break;
                                                                }
                                                                let var32 = var0;
                                                                if (var32 as u32 <= 39i32 as u32) as i32 != 0 {
                                                                    let var33 = var1;
                                                                    let var34 = self.func128(imports, var33); // wasm/cpu/instructions/shiftLeftRegister
                                                                    var2 = var34;
                                                                    var3 = 1i32;
                                                                } else {
                                                                    let var35 = var0;
                                                                    if (var35 as u32 <= 47i32 as u32) as i32 != 0 {
                                                                        let var36 = var1;
                                                                        let var37 = self.func129(imports, var36); // wasm/cpu/instructions/shiftRightArithmeticRegister
                                                                        var2 = var37;
                                                                        var3 = 1i32;
                                                                    }
                                                                }
                                                                break 'label10;
                                                                break;
                                                            }
                                                            let var38 = var0;
                                                            if (var38 as u32 <= 55i32 as u32) as i32 != 0 {
                                                                let var39 = var1;
                                                                let var40 = self.func130(imports, var39); // wasm/cpu/instructions/swapNibblesOnRegister
                                                                var2 = var40;
                                                                var3 = 1i32;
                                                            } else {
                                                                let var41 = var0;
                                                                if (var41 as u32 <= 63i32 as u32) as i32 != 0 {
                                                                    let var42 = var1;
                                                                    let var43 = self.func131(imports, var42); // wasm/cpu/instructions/shiftRightLogicalRegister
                                                                    var2 = var43;
                                                                    var3 = 1i32;
                                                                }
                                                            }
                                                            break 'label10;
                                                            break;
                                                        }
                                                        let var44 = var0;
                                                        if (var44 as u32 <= 71i32 as u32) as i32 != 0 {
                                                            let var45 = var1;
                                                            let var46 = self.func132(imports, 0i32, var45); // wasm/cpu/instructions/testBitOnRegister
                                                            var2 = var46;
                                                            var3 = 1i32;
                                                        } else {
                                                            let var47 = var0;
                                                            if (var47 as u32 <= 79i32 as u32) as i32 != 0 {
                                                                let var48 = var1;
                                                                let var49 = self.func132(imports, 1i32, var48); // wasm/cpu/instructions/testBitOnRegister
                                                                var2 = var49;
                                                                var3 = 1i32;
                                                            }
                                                        }
                                                        break 'label10;
                                                        break;
                                                    }
                                                    let var50 = var0;
                                                    if (var50 as u32 <= 87i32 as u32) as i32 != 0 {
                                                        let var51 = var1;
                                                        let var52 = self.func132(imports, 2i32, var51); // wasm/cpu/instructions/testBitOnRegister
                                                        var2 = var52;
                                                        var3 = 1i32;
                                                    } else {
                                                        let var53 = var0;
                                                        if (var53 as u32 <= 95i32 as u32) as i32 != 0 {
                                                            let var54 = var1;
                                                            let var55 = self.func132(imports, 3i32, var54); // wasm/cpu/instructions/testBitOnRegister
                                                            var2 = var55;
                                                            var3 = 1i32;
                                                        }
                                                    }
                                                    break 'label10;
                                                    break;
                                                }
                                                let var56 = var0;
                                                if (var56 as u32 <= 103i32 as u32) as i32 != 0 {
                                                    let var57 = var1;
                                                    let var58 = self.func132(imports, 4i32, var57); // wasm/cpu/instructions/testBitOnRegister
                                                    var2 = var58;
                                                    var3 = 1i32;
                                                } else {
                                                    let var59 = var0;
                                                    if (var59 as u32 <= 111i32 as u32) as i32 != 0 {
                                                        let var60 = var1;
                                                        let var61 = self.func132(imports, 5i32, var60); // wasm/cpu/instructions/testBitOnRegister
                                                        var2 = var61;
                                                        var3 = 1i32;
                                                    }
                                                }
                                                break 'label10;
                                                break;
                                            }
                                            let var62 = var0;
                                            if (var62 as u32 <= 119i32 as u32) as i32 != 0 {
                                                let var63 = var1;
                                                let var64 = self.func132(imports, 6i32, var63); // wasm/cpu/instructions/testBitOnRegister
                                                var2 = var64;
                                                var3 = 1i32;
                                            } else {
                                                let var65 = var0;
                                                if (var65 as u32 <= 127i32 as u32) as i32 != 0 {
                                                    let var66 = var1;
                                                    let var67 = self.func132(imports, 7i32, var66); // wasm/cpu/instructions/testBitOnRegister
                                                    var2 = var67;
                                                    var3 = 1i32;
                                                }
                                            }
                                            break 'label10;
                                            break;
                                        }
                                        let var68 = var0;
                                        if (var68 as u32 <= 135i32 as u32) as i32 != 0 {
                                            let var69 = var1;
                                            let var70 = self.func133(imports, 0i32, 0i32, var69); // wasm/cpu/instructions/setBitOnRegister
                                            var2 = var70;
                                            var3 = 1i32;
                                        } else {
                                            let var71 = var0;
                                            if (var71 as u32 <= 143i32 as u32) as i32 != 0 {
                                                let var72 = var1;
                                                let var73 = self.func133(imports, 1i32, 0i32, var72); // wasm/cpu/instructions/setBitOnRegister
                                                var2 = var73;
                                                var3 = 1i32;
                                            }
                                        }
                                        break 'label10;
                                        break;
                                    }
                                    let var74 = var0;
                                    if (var74 as u32 <= 151i32 as u32) as i32 != 0 {
                                        let var75 = var1;
                                        let var76 = self.func133(imports, 2i32, 0i32, var75); // wasm/cpu/instructions/setBitOnRegister
                                        var2 = var76;
                                        var3 = 1i32;
                                    } else {
                                        let var77 = var0;
                                        if (var77 as u32 <= 159i32 as u32) as i32 != 0 {
                                            let var78 = var1;
                                            let var79 = self.func133(imports, 3i32, 0i32, var78); // wasm/cpu/instructions/setBitOnRegister
                                            var2 = var79;
                                            var3 = 1i32;
                                        }
                                    }
                                    break 'label10;
                                    break;
                                }
                                let var80 = var0;
                                if (var80 as u32 <= 167i32 as u32) as i32 != 0 {
                                    let var81 = var1;
                                    let var82 = self.func133(imports, 4i32, 0i32, var81); // wasm/cpu/instructions/setBitOnRegister
                                    var2 = var82;
                                    var3 = 1i32;
                                } else {
                                    let var83 = var0;
                                    if (var83 as u32 <= 175i32 as u32) as i32 != 0 {
                                        let var84 = var1;
                                        let var85 = self.func133(imports, 5i32, 0i32, var84); // wasm/cpu/instructions/setBitOnRegister
                                        var2 = var85;
                                        var3 = 1i32;
                                    }
                                }
                                break 'label10;
                                break;
                            }
                            let var86 = var0;
                            if (var86 as u32 <= 183i32 as u32) as i32 != 0 {
                                let var87 = var1;
                                let var88 = self.func133(imports, 6i32, 0i32, var87); // wasm/cpu/instructions/setBitOnRegister
                                var2 = var88;
                                var3 = 1i32;
                            } else {
                                let var89 = var0;
                                if (var89 as u32 <= 191i32 as u32) as i32 != 0 {
                                    let var90 = var1;
                                    let var91 = self.func133(imports, 7i32, 0i32, var90); // wasm/cpu/instructions/setBitOnRegister
                                    var2 = var91;
                                    var3 = 1i32;
                                }
                            }
                            break 'label10;
                            break;
                        }
                        let var92 = var0;
                        if (var92 as u32 <= 199i32 as u32) as i32 != 0 {
                            let var93 = var1;
                            let var94 = self.func133(imports, 0i32, 1i32, var93); // wasm/cpu/instructions/setBitOnRegister
                            var2 = var94;
                            var3 = 1i32;
                        } else {
                            let var95 = var0;
                            if (var95 as u32 <= 207i32 as u32) as i32 != 0 {
                                let var96 = var1;
                                let var97 = self.func133(imports, 1i32, 1i32, var96); // wasm/cpu/instructions/setBitOnRegister
                                var2 = var97;
                                var3 = 1i32;
                            }
                        }
                        break 'label10;
                        break;
                    }
                    let var98 = var0;
                    if (var98 as u32 <= 215i32 as u32) as i32 != 0 {
                        let var99 = var1;
                        let var100 = self.func133(imports, 2i32, 1i32, var99); // wasm/cpu/instructions/setBitOnRegister
                        var2 = var100;
                        var3 = 1i32;
                    } else {
                        let var101 = var0;
                        if (var101 as u32 <= 223i32 as u32) as i32 != 0 {
                            let var102 = var1;
                            let var103 = self.func133(imports, 3i32, 1i32, var102); // wasm/cpu/instructions/setBitOnRegister
                            var2 = var103;
                            var3 = 1i32;
                        }
                    }
                    break 'label10;
                    break;
                }
                let var104 = var0;
                if (var104 as u32 <= 231i32 as u32) as i32 != 0 {
                    let var105 = var1;
                    let var106 = self.func133(imports, 4i32, 1i32, var105); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var106;
                    var3 = 1i32;
                } else {
                    let var107 = var0;
                    if (var107 as u32 <= 239i32 as u32) as i32 != 0 {
                        let var108 = var1;
                        let var109 = self.func133(imports, 5i32, 1i32, var108); // wasm/cpu/instructions/setBitOnRegister
                        var2 = var109;
                        var3 = 1i32;
                    }
                }
                break 'label10;
                break;
            }
            let var110 = var0;
            if (var110 as u32 <= 247i32 as u32) as i32 != 0 {
                let var111 = var1;
                let var112 = self.func133(imports, 6i32, 1i32, var111); // wasm/cpu/instructions/setBitOnRegister
                var2 = var112;
                var3 = 1i32;
            } else {
                let var113 = var0;
                if (var113 as u32 <= 255i32 as u32) as i32 != 0 {
                    let var114 = var1;
                    let var115 = self.func133(imports, 7i32, 1i32, var114); // wasm/cpu/instructions/setBitOnRegister
                    var2 = var115;
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
                                                let var116 = var5;
                                                match var116 {
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
                                        let var117 = var2;
                                        self.global3 = var117;
                                        break 'label28;
                                        break;
                                    }
                                    let var118 = var2;
                                    self.global4 = var118;
                                    break 'label28;
                                    break;
                                }
                                let var119 = var2;
                                self.global5 = var119;
                                break 'label28;
                                break;
                            }
                            let var120 = var2;
                            self.global6 = var120;
                            break 'label28;
                            break;
                        }
                        let var121 = var2;
                        self.global7 = var121;
                        break 'label28;
                        break;
                    }
                    let var122 = var2;
                    self.global8 = var122;
                    break 'label28;
                    break;
                }
                let var123 = self.global7;
                let var124 = self.global8;
                let var125 = self.func84(imports, var123, var124); // wasm/helpers/index/concatenateBytes
                let var126 = var2;
                self.func71(imports, var125, var126); // wasm/memory/store/eightBitStoreIntoGBMemory
                break 'label28;
                break;
            }
            let var127 = var2;
            self.global1 = var127;
            break;
        }
        let var128 = self.global0;
        self.global0 = var128.wrapping_add(1i32) & 65535i32;
        let var129 = var3;
        if var129 != 0 {
            var4 = 8i32;
            let var130 = var5;
            if (var130 == 6i32) as i32 != 0 {
                var4 = 16i32;
            }
        }
        let var131 = var4;
        var131
    }
    // wasm/cpu/opcodes/handleOpcodeCx
    fn func135<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                                let var5 = var0;
                                                                                match var5.wrapping_sub(192i32) {
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
                                                                        let var6 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                                                        if var6 != 0 {
                                                                            return 8i32;
                                                                        } else {
                                                                            let var7 = self.global9;
                                                                            let var8 = self.func123(imports, var7); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                                            self.global0 = var8;
                                                                            let var9 = self.global9;
                                                                            self.global9 = var9.wrapping_add(2i32) & 65535i32;
                                                                            return 20i32;
                                                                        }
                                                                        unreachable!();
                                                                        break;
                                                                    }
                                                                    let var10 = self.global3;
                                                                    let var11 = self.global4;
                                                                    let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                                    let var13 = self.global9;
                                                                    let var14 = self.func123(imports, var13); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                                    var4 = var14;
                                                                    let var15 = self.global9;
                                                                    self.global9 = var15.wrapping_add(2i32) & 65535i32;
                                                                    let var16 = var4;
                                                                    let var17 = self.func9(imports, var16); // wasm/helpers/index/splitHighByte
                                                                    self.global3 = var17;
                                                                    let var18 = var4;
                                                                    let var19 = self.func10(imports, var18); // wasm/helpers/index/splitLowByte
                                                                    self.global4 = var19;
                                                                    return 12i32;
                                                                    break;
                                                                }
                                                                let var20 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                                                if var20 != 0 {
                                                                    let var21 = self.global0;
                                                                    self.global0 = var21.wrapping_add(2i32) & 65535i32;
                                                                    return 12i32;
                                                                } else {
                                                                    let var22 = var3;
                                                                    self.global0 = var22;
                                                                    return 16i32;
                                                                }
                                                                unreachable!();
                                                                break;
                                                            }
                                                            let var23 = var3;
                                                            self.global0 = var23;
                                                            return 16i32;
                                                            break;
                                                        }
                                                        let var24 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                                        if var24 != 0 {
                                                            let var25 = self.global0;
                                                            self.global0 = var25.wrapping_add(2i32) & 65535i32;
                                                            return 12i32;
                                                        } else {
                                                            let var26 = self.global9;
                                                            self.global9 = var26.wrapping_sub(2i32) & 65535i32;
                                                            let var27 = self.global9;
                                                            let var28 = self.global0;
                                                            self.func92(imports, var27, var28.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                            let var29 = self.global0;
                                                            let var30 = self.func123(imports, var29); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                            self.global0 = var30;
                                                            return 24i32;
                                                        }
                                                        unreachable!();
                                                        break;
                                                    }
                                                    let var31 = self.global3;
                                                    let var32 = self.global4;
                                                    let var33 = self.func84(imports, var31, var32); // wasm/helpers/index/concatenateBytes
                                                    var4 = var33;
                                                    let var34 = self.global9;
                                                    self.global9 = var34.wrapping_sub(2i32) & 65535i32;
                                                    let var35 = self.global9;
                                                    let var36 = var4;
                                                    self.func92(imports, var35, var36); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                    return 16i32;
                                                    break;
                                                }
                                                let var37 = var1;
                                                self.func111(imports, var37); // wasm/cpu/instructions/addARegister
                                                let var38 = self.global0;
                                                self.global0 = var38.wrapping_add(1i32) & 65535i32;
                                                return 4i32;
                                                break;
                                            }
                                            let var39 = self.global9;
                                            self.global9 = var39.wrapping_sub(2i32) & 65535i32;
                                            let var40 = self.global9;
                                            let var41 = self.global0;
                                            self.func92(imports, var40, var41); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                            self.global0 = 0i32;
                                            return 16i32;
                                            break;
                                        }
                                        let var42 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                        if (var42 == 1i32) as i32 != 0 {
                                            let var43 = self.global9;
                                            let var44 = self.func123(imports, var43); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                            self.global0 = var44;
                                            let var45 = self.global9;
                                            self.global9 = var45.wrapping_add(2i32) & 65535i32;
                                            return 20i32;
                                        } else {
                                            return 8i32;
                                        }
                                        unreachable!();
                                        break;
                                    }
                                    let var46 = self.global9;
                                    let var47 = self.func123(imports, var46); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                    self.global0 = var47;
                                    let var48 = self.global9;
                                    self.global9 = var48.wrapping_add(2i32) & 65535i32;
                                    return 16i32;
                                    break;
                                }
                                let var49 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                                if (var49 == 1i32) as i32 != 0 {
                                    let var50 = var3;
                                    self.global0 = var50;
                                    return 16i32;
                                } else {
                                    let var51 = self.global0;
                                    self.global0 = var51.wrapping_add(2i32) & 65535i32;
                                    return 12i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var52 = var1;
                            let var53 = self.func134(imports, var52); // wasm/cpu/cbOpcodes/handleCbOpcode
                            var4 = var53;
                            let var54 = var4;
                            if (var54 > 0i32) as i32 != 0 {
                                let var55 = var4;
                                var4 = var55.wrapping_add(4i32).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            }
                            let var56 = var4;
                            return var56;
                            break;
                        }
                        let var57 = self.func101(imports); // wasm/cpu/flags/getZeroFlag
                        if (var57 == 1i32) as i32 != 0 {
                            let var58 = self.global9;
                            self.global9 = var58.wrapping_sub(2i32) & 65535i32;
                            let var59 = self.global9;
                            let var60 = self.global0;
                            self.func92(imports, var59, var60.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                            let var61 = self.global0;
                            let var62 = self.func123(imports, var61); // wasm/memory/load/sixteenBitLoadFromGBMemory
                            self.global0 = var62;
                            return 24i32;
                        } else {
                            let var63 = self.global0;
                            self.global0 = var63.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var64 = self.global9;
                    self.global9 = var64.wrapping_sub(2i32) & 65535i32;
                    let var65 = self.global9;
                    let var66 = self.global0;
                    self.func92(imports, var65, var66.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                    let var67 = self.global0;
                    let var68 = self.func123(imports, var67); // wasm/memory/load/sixteenBitLoadFromGBMemory
                    self.global0 = var68;
                    return 24i32;
                    break;
                }
                let var69 = var1;
                self.func112(imports, var69); // wasm/cpu/instructions/addAThroughCarryRegister
                let var70 = self.global0;
                self.global0 = var70.wrapping_add(1i32) & 65535i32;
                return 4i32;
                break;
            }
            let var71 = self.global9;
            self.global9 = var71.wrapping_sub(2i32) & 65535i32;
            let var72 = self.global9;
            let var73 = self.global0;
            self.func92(imports, var72, var73); // wasm/memory/store/sixteenBitStoreIntoGBMemory
            self.global0 = 8i32;
            return 16i32;
            break;
        }
        -1i32
    }
    // wasm/interrupts/index/setInterrupts
    fn func136<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.global71 = var1;
    }
    // wasm/cpu/opcodes/handleOpcodeDx
    fn func137<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                    let var5 = var0;
                                                                    match var5.wrapping_sub(208i32) {
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
                                                            let var6 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                                            if var6 != 0 {
                                                                return 8i32;
                                                            } else {
                                                                let var7 = self.global9;
                                                                let var8 = self.func123(imports, var7); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                                self.global0 = var8;
                                                                let var9 = self.global9;
                                                                self.global9 = var9.wrapping_add(2i32) & 65535i32;
                                                                return 20i32;
                                                            }
                                                            unreachable!();
                                                            break;
                                                        }
                                                        let var10 = self.global5;
                                                        let var11 = self.global6;
                                                        let var12 = self.func84(imports, var10, var11); // wasm/helpers/index/concatenateBytes
                                                        let var13 = self.global9;
                                                        let var14 = self.func123(imports, var13); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                        var4 = var14;
                                                        let var15 = self.global9;
                                                        self.global9 = var15.wrapping_add(2i32) & 65535i32;
                                                        let var16 = var4;
                                                        let var17 = self.func9(imports, var16); // wasm/helpers/index/splitHighByte
                                                        self.global5 = var17;
                                                        let var18 = var4;
                                                        let var19 = self.func10(imports, var18); // wasm/helpers/index/splitLowByte
                                                        self.global6 = var19;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var20 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                                    if var20 != 0 {
                                                        let var21 = self.global0;
                                                        self.global0 = var21.wrapping_add(2i32) & 65535i32;
                                                        return 12i32;
                                                    } else {
                                                        let var22 = var3;
                                                        self.global0 = var22;
                                                        return 16i32;
                                                    }
                                                    unreachable!();
                                                    break;
                                                }
                                                let var23 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                                if var23 != 0 {
                                                    let var24 = self.global0;
                                                    self.global0 = var24.wrapping_add(2i32) & 65535i32;
                                                    return 12i32;
                                                } else {
                                                    let var25 = self.global9;
                                                    self.global9 = var25.wrapping_sub(2i32) & 65535i32;
                                                    let var26 = self.global9;
                                                    let var27 = self.global0;
                                                    self.func92(imports, var26, var27.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                                    let var28 = self.global0;
                                                    let var29 = self.func123(imports, var28); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                    self.global0 = var29;
                                                    return 24i32;
                                                }
                                                unreachable!();
                                                break;
                                            }
                                            let var30 = self.global5;
                                            let var31 = self.global6;
                                            let var32 = self.func84(imports, var30, var31); // wasm/helpers/index/concatenateBytes
                                            var4 = var32;
                                            let var33 = self.global9;
                                            self.global9 = var33.wrapping_sub(2i32) & 65535i32;
                                            let var34 = self.global9;
                                            let var35 = var4;
                                            self.func92(imports, var34, var35); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                            return 16i32;
                                            break;
                                        }
                                        let var36 = var1;
                                        self.func114(imports, var36); // wasm/cpu/instructions/subARegister
                                        let var37 = self.global0;
                                        self.global0 = var37.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var38 = self.global9;
                                    self.global9 = var38.wrapping_sub(2i32) & 65535i32;
                                    let var39 = self.global9;
                                    let var40 = self.global0;
                                    self.func92(imports, var39, var40); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                    self.global0 = 16i32;
                                    return 16i32;
                                    break;
                                }
                                let var41 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                                if (var41 == 1i32) as i32 != 0 {
                                    let var42 = self.global9;
                                    let var43 = self.func123(imports, var42); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                    self.global0 = var43;
                                    let var44 = self.global9;
                                    self.global9 = var44.wrapping_add(2i32) & 65535i32;
                                    return 20i32;
                                } else {
                                    return 8i32;
                                }
                                unreachable!();
                                break;
                            }
                            let var45 = self.global9;
                            let var46 = self.func123(imports, var45); // wasm/memory/load/sixteenBitLoadFromGBMemory
                            self.global0 = var46;
                            self.func136(imports, 1i32); // wasm/interrupts/index/setInterrupts
                            let var47 = self.global9;
                            self.global9 = var47.wrapping_add(2i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var48 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                        if (var48 == 1i32) as i32 != 0 {
                            let var49 = var3;
                            self.global0 = var49;
                            return 16i32;
                        } else {
                            let var50 = self.global0;
                            self.global0 = var50.wrapping_add(2i32) & 65535i32;
                            return 12i32;
                        }
                        unreachable!();
                        break;
                    }
                    let var51 = self.func96(imports); // wasm/cpu/flags/getCarryFlag
                    if (var51 == 1i32) as i32 != 0 {
                        let var52 = self.global9;
                        self.global9 = var52.wrapping_sub(2i32) & 65535i32;
                        let var53 = self.global9;
                        let var54 = self.global0;
                        self.func92(imports, var53, var54.wrapping_add(2i32) & 65535i32); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                        let var55 = self.global0;
                        let var56 = self.func123(imports, var55); // wasm/memory/load/sixteenBitLoadFromGBMemory
                        self.global0 = var56;
                        return 24i32;
                    } else {
                        let var57 = self.global0;
                        self.global0 = var57.wrapping_add(2i32) & 65535i32;
                        return 12i32;
                    }
                    unreachable!();
                    break;
                }
                let var58 = var1;
                self.func115(imports, var58); // wasm/cpu/instructions/subAThroughCarryRegister
                let var59 = self.global0;
                self.global0 = var59.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var60 = self.global9;
            self.global9 = var60.wrapping_sub(2i32) & 65535i32;
            let var61 = self.global9;
            let var62 = self.global0;
            self.func92(imports, var61, var62); // wasm/memory/store/sixteenBitStoreIntoGBMemory
            self.global0 = 24i32;
            return 16i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcodeEx
    fn func138<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                            let var5 = var0;
                                                            match var5.wrapping_sub(224i32) {
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
                                                    let var6 = var1;
                                                    let var7 = self.global1;
                                                    self.func71(imports, var6.wrapping_add(65280i32) & 65535i32, var7); // wasm/memory/store/eightBitStoreIntoGBMemory
                                                    let var8 = self.global0;
                                                    self.global0 = var8.wrapping_add(1i32) & 65535i32;
                                                    return 12i32;
                                                    break;
                                                }
                                                let var9 = self.global7;
                                                let var10 = self.global8;
                                                let var11 = self.func84(imports, var9, var10); // wasm/helpers/index/concatenateBytes
                                                let var12 = self.global9;
                                                let var13 = self.func123(imports, var12); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                var4 = var13;
                                                let var14 = self.global9;
                                                self.global9 = var14.wrapping_add(2i32) & 65535i32;
                                                let var15 = var4;
                                                let var16 = self.func9(imports, var15); // wasm/helpers/index/splitHighByte
                                                self.global7 = var16;
                                                let var17 = var4;
                                                let var18 = self.func10(imports, var17); // wasm/helpers/index/splitLowByte
                                                self.global8 = var18;
                                                return 12i32;
                                                break;
                                            }
                                            let var19 = self.global4;
                                            let var20 = self.global1;
                                            self.func71(imports, var19.wrapping_add(65280i32) & 65535i32, var20); // wasm/memory/store/eightBitStoreIntoGBMemory
                                            return 8i32;
                                            break;
                                        }
                                        let var21 = self.global7;
                                        let var22 = self.global8;
                                        let var23 = self.func84(imports, var21, var22); // wasm/helpers/index/concatenateBytes
                                        var4 = var23;
                                        let var24 = self.global9;
                                        self.global9 = var24.wrapping_sub(2i32) & 65535i32;
                                        let var25 = self.global9;
                                        let var26 = var4;
                                        self.func92(imports, var25, var26); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                        return 16i32;
                                        break;
                                    }
                                    let var27 = var1;
                                    self.func117(imports, var27); // wasm/cpu/instructions/andARegister
                                    let var28 = self.global0;
                                    self.global0 = var28.wrapping_add(1i32) & 65535i32;
                                    return 8i32;
                                    break;
                                }
                                let var29 = self.global9;
                                self.global9 = var29.wrapping_sub(2i32) & 65535i32;
                                let var30 = self.global9;
                                let var31 = self.global0;
                                self.func92(imports, var30, var31); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                self.global0 = 32i32;
                                return 16i32;
                                break;
                            }
                            let var32 = self.global9;
                            let var33 = var1;
                            var4 = var33.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                            let var34 = var4;
                            self.func93(imports, var32, var34, 1i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                            let var35 = self.global9;
                            let var36 = var4;
                            self.global9 = var35.wrapping_add(var36) & 65535i32;
                            self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                            self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                            let var37 = self.global0;
                            self.global0 = var37.wrapping_add(1i32) & 65535i32;
                            return 16i32;
                            break;
                        }
                        let var38 = self.global7;
                        let var39 = self.global8;
                        let var40 = self.func84(imports, var38, var39); // wasm/helpers/index/concatenateBytes
                        self.global0 = var40;
                        return 4i32;
                        break;
                    }
                    let var41 = var3;
                    let var42 = self.global1;
                    self.func71(imports, var41, var42); // wasm/memory/store/eightBitStoreIntoGBMemory
                    let var43 = self.global0;
                    self.global0 = var43.wrapping_add(2i32) & 65535i32;
                    return 16i32;
                    break;
                }
                let var44 = var1;
                self.func118(imports, var44); // wasm/cpu/instructions/xorARegister
                let var45 = self.global0;
                self.global0 = var45.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var46 = self.global9;
            self.global9 = var46.wrapping_sub(2i32) & 65535i32;
            let var47 = self.global9;
            let var48 = self.global0;
            self.func92(imports, var47, var48); // wasm/memory/store/sixteenBitStoreIntoGBMemory
            self.global0 = 40i32;
            return 16i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/handleOpcodeFx
    fn func139<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32) -> i32 {
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
                                                                    let var4 = var0;
                                                                    match var4.wrapping_sub(240i32) {
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
                                                            let var5 = var1;
                                                            let var6 = self.func75(imports, var5.wrapping_add(65280i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                                                            self.global1 = var6;
                                                            let var7 = self.global0;
                                                            self.global0 = var7.wrapping_add(1i32) & 65535i32;
                                                            return 12i32;
                                                            break;
                                                        }
                                                        let var8 = self.global1;
                                                        let var9 = self.global2;
                                                        let var10 = self.func84(imports, var8, var9); // wasm/helpers/index/concatenateBytes
                                                        let var11 = self.global9;
                                                        let var12 = self.func123(imports, var11); // wasm/memory/load/sixteenBitLoadFromGBMemory
                                                        var0 = var12;
                                                        let var13 = self.global9;
                                                        self.global9 = var13.wrapping_add(2i32) & 65535i32;
                                                        let var14 = var0;
                                                        let var15 = self.func9(imports, var14); // wasm/helpers/index/splitHighByte
                                                        self.global1 = var15;
                                                        let var16 = var0;
                                                        let var17 = self.func10(imports, var16); // wasm/helpers/index/splitLowByte
                                                        self.global2 = var17;
                                                        return 12i32;
                                                        break;
                                                    }
                                                    let var18 = self.global4;
                                                    let var19 = self.func75(imports, var18.wrapping_add(65280i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                                                    self.global1 = var19;
                                                    let var20 = self.global0;
                                                    self.global0 = var20.wrapping_add(1i32) & 65535i32;
                                                    return 8i32;
                                                    break;
                                                }
                                                self.func136(imports, 0i32); // wasm/interrupts/index/setInterrupts
                                                return 4i32;
                                                break;
                                            }
                                            let var21 = self.global1;
                                            let var22 = self.global2;
                                            let var23 = self.func84(imports, var21, var22); // wasm/helpers/index/concatenateBytes
                                            var0 = var23;
                                            let var24 = self.global9;
                                            self.global9 = var24.wrapping_sub(2i32) & 65535i32;
                                            let var25 = self.global9;
                                            let var26 = var0;
                                            self.func92(imports, var25, var26); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                            return 16i32;
                                            break;
                                        }
                                        let var27 = var1;
                                        self.func120(imports, var27); // wasm/cpu/instructions/orARegister
                                        let var28 = self.global0;
                                        self.global0 = var28.wrapping_add(1i32) & 65535i32;
                                        return 8i32;
                                        break;
                                    }
                                    let var29 = self.global9;
                                    self.global9 = var29.wrapping_sub(2i32) & 65535i32;
                                    let var30 = self.global9;
                                    let var31 = self.global0;
                                    self.func92(imports, var30, var31); // wasm/memory/store/sixteenBitStoreIntoGBMemory
                                    self.global0 = 48i32;
                                    return 16i32;
                                    break;
                                }
                                self.func88(imports, 0i32); // wasm/cpu/flags/setZeroFlag
                                self.func89(imports, 0i32); // wasm/cpu/flags/setSubtractFlag
                                let var32 = self.global9;
                                let var33 = var1;
                                var0 = var33.wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
                                let var34 = var0;
                                self.func93(imports, var32, var34, 1i32); // wasm/cpu/flags/checkAndSetSixteenBitFlagsAddOverflow
                                let var35 = self.global9;
                                let var36 = var0;
                                var0 = var35.wrapping_add(var36) & 65535i32;
                                let var37 = var0;
                                let var38 = self.func9(imports, var37); // wasm/helpers/index/splitHighByte
                                self.global7 = var38;
                                let var39 = var0;
                                let var40 = self.func10(imports, var39); // wasm/helpers/index/splitLowByte
                                self.global8 = var40;
                                let var41 = self.global0;
                                self.global0 = var41.wrapping_add(1i32) & 65535i32;
                                return 12i32;
                                break;
                            }
                            let var42 = self.global7;
                            let var43 = self.global8;
                            let var44 = self.func84(imports, var42, var43); // wasm/helpers/index/concatenateBytes
                            self.global9 = var44;
                            return 8i32;
                            break;
                        }
                        let var45 = var3;
                        let var46 = self.func75(imports, var45); // wasm/memory/load/eightBitLoadFromGBMemory
                        self.global1 = var46;
                        let var47 = self.global0;
                        self.global0 = var47.wrapping_add(2i32) & 65535i32;
                        return 16i32;
                        break;
                    }
                    self.func136(imports, 1i32); // wasm/interrupts/index/setInterrupts
                    return 4i32;
                    break;
                }
                let var48 = var1;
                self.func121(imports, var48); // wasm/cpu/instructions/cpARegister
                let var49 = self.global0;
                self.global0 = var49.wrapping_add(1i32) & 65535i32;
                return 8i32;
                break;
            }
            let var50 = self.global9;
            self.global9 = var50.wrapping_sub(2i32) & 65535i32;
            let var51 = self.global9;
            let var52 = self.global0;
            self.func92(imports, var51, var52); // wasm/memory/store/sixteenBitStoreIntoGBMemory
            self.global0 = 56i32;
            return 16i32;
            break;
        }
        -1i32
    }
    // wasm/cpu/opcodes/executeOpcode
    fn func140<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let var4 = self.global0;
        self.global0 = var4.wrapping_add(1i32) & 65535i32;
        let var5 = var2;
        let var6 = var1;
        let var7 = self.func84(imports, var5, var6); // wasm/helpers/index/concatenateBytes
        var3 = var7;
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
                                                                            let var8 = var0;
                                                                            match ((var8 & 240i32) as u32).wrapping_shr(4i32 as u32) as i32 {
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
                                                                    let var9 = var0;
                                                                    let var10 = var1;
                                                                    let var11 = var2;
                                                                    let var12 = var3;
                                                                    let var13 = self.func95(imports, var9, var10, var11, var12); // wasm/cpu/opcodes/handleOpcode0x
                                                                    return var13;
                                                                    break;
                                                                }
                                                                let var14 = var0;
                                                                let var15 = var1;
                                                                let var16 = var2;
                                                                let var17 = var3;
                                                                let var18 = self.func100(imports, var14, var15, var16, var17); // wasm/cpu/opcodes/handleOpcode1x
                                                                return var18;
                                                                break;
                                                            }
                                                            let var19 = var0;
                                                            let var20 = var1;
                                                            let var21 = var2;
                                                            let var22 = var3;
                                                            let var23 = self.func104(imports, var19, var20, var21, var22); // wasm/cpu/opcodes/handleOpcode2x
                                                            return var23;
                                                            break;
                                                        }
                                                        let var24 = var0;
                                                        let var25 = var1;
                                                        let var26 = var2;
                                                        let var27 = var3;
                                                        let var28 = self.func105(imports, var24, var25, var26, var27); // wasm/cpu/opcodes/handleOpcode3x
                                                        return var28;
                                                        break;
                                                    }
                                                    let var29 = var0;
                                                    let var30 = var1;
                                                    let var31 = var2;
                                                    let var32 = var3;
                                                    let var33 = self.func106(imports, var29, var30, var31, var32); // wasm/cpu/opcodes/handleOpcode4x
                                                    return var33;
                                                    break;
                                                }
                                                let var34 = var0;
                                                let var35 = var1;
                                                let var36 = var2;
                                                let var37 = var3;
                                                let var38 = self.func107(imports, var34, var35, var36, var37); // wasm/cpu/opcodes/handleOpcode5x
                                                return var38;
                                                break;
                                            }
                                            let var39 = var0;
                                            let var40 = var1;
                                            let var41 = var2;
                                            let var42 = var3;
                                            let var43 = self.func108(imports, var39, var40, var41, var42); // wasm/cpu/opcodes/handleOpcode6x
                                            return var43;
                                            break;
                                        }
                                        let var44 = var0;
                                        let var45 = var1;
                                        let var46 = var2;
                                        let var47 = var3;
                                        let var48 = self.func109(imports, var44, var45, var46, var47); // wasm/cpu/opcodes/handleOpcode7x
                                        return var48;
                                        break;
                                    }
                                    let var49 = var0;
                                    let var50 = var1;
                                    let var51 = var2;
                                    let var52 = var3;
                                    let var53 = self.func113(imports, var49, var50, var51, var52); // wasm/cpu/opcodes/handleOpcode8x
                                    return var53;
                                    break;
                                }
                                let var54 = var0;
                                let var55 = var1;
                                let var56 = var2;
                                let var57 = var3;
                                let var58 = self.func116(imports, var54, var55, var56, var57); // wasm/cpu/opcodes/handleOpcode9x
                                return var58;
                                break;
                            }
                            let var59 = var0;
                            let var60 = var1;
                            let var61 = var2;
                            let var62 = var3;
                            let var63 = self.func119(imports, var59, var60, var61, var62); // wasm/cpu/opcodes/handleOpcodeAx
                            return var63;
                            break;
                        }
                        let var64 = var0;
                        let var65 = var1;
                        let var66 = var2;
                        let var67 = var3;
                        let var68 = self.func122(imports, var64, var65, var66, var67); // wasm/cpu/opcodes/handleOpcodeBx
                        return var68;
                        break;
                    }
                    let var69 = var0;
                    let var70 = var1;
                    let var71 = var2;
                    let var72 = var3;
                    let var73 = self.func135(imports, var69, var70, var71, var72); // wasm/cpu/opcodes/handleOpcodeCx
                    return var73;
                    break;
                }
                let var74 = var0;
                let var75 = var1;
                let var76 = var2;
                let var77 = var3;
                let var78 = self.func137(imports, var74, var75, var76, var77); // wasm/cpu/opcodes/handleOpcodeDx
                return var78;
                break;
            }
            let var79 = var0;
            let var80 = var1;
            let var81 = var2;
            let var82 = var3;
            let var83 = self.func138(imports, var79, var80, var81, var82); // wasm/cpu/opcodes/handleOpcodeEx
            return var83;
            break;
        }
        let var84 = var0;
        let var85 = var1;
        let var86 = var2;
        let var87 = var3;
        let var88 = self.func139(imports, var84, var85, var86, var87); // wasm/cpu/opcodes/handleOpcodeFx
        var88
    }
    // wasm/interrupts/index/areInterruptsEnabled
    fn func141<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global71;
        var0
    }
    // wasm/interrupts/index/areInterruptsPending
    fn func142<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var1 = self.func13(imports, 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var2: i32;
        if var0 & var1 & 255i32 != 0 {
            var2 = 1i32;
        } else {
            var2 = 0i32;
        }
        var2
    }
    // wasm/graphics/lcd/isLcdEnabled
    fn func143<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.func13(imports, 65344i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var1 = self.func2(imports, 7i32, var0); // wasm/helpers/index/checkBitOnByte
        var1
    }
    // wasm/interrupts/index/requestLcdInterrupt
    fn func144<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func18(imports, 1i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/graphics/lcd/setLcdStatus
    fn func145<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.func13(imports, 65345i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var1 = var4;
        let var5 = var0;
        if (var5 == 0) as i32 != 0 {
            self.global72 = 0i32;
            self.func8(imports, 65348i32, 0i32); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            let var6 = var1;
            let var7 = self.func72(imports, 1i32, var6); // wasm/helpers/index/resetBitOnByte
            let var8 = self.func72(imports, 0i32, var7); // wasm/helpers/index/resetBitOnByte
            var1 = var8;
            self.global19 = 0i32;
            let var9 = var1;
            self.func8(imports, 65345i32, var9); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
            return;
        }
        let var10 = var1;
        let var11 = self.func13(imports, 65348i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        var3 = var11;
        let var12 = var3;
        let var13: i32;
        if (var12 as u32 >= 144i32 as u32) as i32 != 0 {
            let var14 = var1;
            let var15 = self.func72(imports, 1i32, var14); // wasm/helpers/index/resetBitOnByte
            let var16 = self.func17(imports, 0i32, var15); // wasm/helpers/index/setBitOnByte
            var1 = var16;
            let var17 = var1;
            let var18 = self.func2(imports, 4i32, var17); // wasm/helpers/index/checkBitOnByte
            var2 = var18;
            var13 = 1i32;
        } else {
            let var19 = self.global72;
            let var20: i32;
            if (var19 >= 376i32) as i32 != 0 {
                let var21 = var1;
                let var22 = self.func72(imports, 0i32, var21); // wasm/helpers/index/resetBitOnByte
                let var23 = self.func17(imports, 1i32, var22); // wasm/helpers/index/setBitOnByte
                var1 = var23;
                let var24 = var1;
                let var25 = self.func2(imports, 5i32, var24); // wasm/helpers/index/checkBitOnByte
                var2 = var25;
                var20 = 2i32;
            } else {
                let var26 = self.global72;
                let var27: i32;
                if (var26 >= 249i32) as i32 != 0 {
                    let var28 = var1;
                    let var29 = self.func17(imports, 0i32, var28); // wasm/helpers/index/setBitOnByte
                    let var30 = self.func17(imports, 1i32, var29); // wasm/helpers/index/setBitOnByte
                    var1 = var30;
                    var27 = 3i32;
                } else {
                    let var31 = var1;
                    let var32 = self.func72(imports, 0i32, var31); // wasm/helpers/index/resetBitOnByte
                    let var33 = self.func72(imports, 1i32, var32); // wasm/helpers/index/resetBitOnByte
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
                self.func144(imports); // wasm/interrupts/index/requestLcdInterrupt
            }
            let var38 = var0;
            var2 = (var38 == 0) as i32;
            let var39 = var2;
            let var40: i32;
            if var39 != 0 {
                let var41 = var3;
                let var42 = self.func13(imports, 65349i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var40 = (var41 == var42) as i32;
            } else {
                let var43 = var2;
                var40 = var43;
            }
            if var40 & 1i32 != 0 {
                let var44 = var1;
                let var45 = self.func17(imports, 2i32, var44); // wasm/helpers/index/setBitOnByte
                var1 = var45;
                let var46 = var1;
                let var47 = self.func2(imports, 6i32, var46); // wasm/helpers/index/checkBitOnByte
                if var47 != 0 {
                    self.func144(imports); // wasm/interrupts/index/requestLcdInterrupt
                }
            } else {
                let var48 = var1;
                let var49 = self.func72(imports, 2i32, var48); // wasm/helpers/index/resetBitOnByte
                var1 = var49;
            }
        }
        let var50 = var0;
        self.global19 = var50;
        let var51 = var1;
        self.func8(imports, 65345i32, var51); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
    }
    // wasm/graphics/renderUtils/getTileDataAddress
    fn func146<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
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
    fn func147<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = self.func13(imports, var2); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var4 = var0;
        (var3 as u32).wrapping_shr((var4.wrapping_mul(2i32) & 255i32) as u32) as i32 & 3i32
    }
    // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
    fn func148<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var0;
        let var3 = var1;
        self.memory.store8(var2 as usize, var3.wrapping_add(1i32) as u8);
    }
    // wasm/graphics/background/renderBackground
    fn func149<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
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
                let var26 = self.func146(imports, var21, var25); // wasm/graphics/renderUtils/getTileDataAddress
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
                let var45 = self.func147(imports, var44, 65351i32); // wasm/graphics/renderUtils/getColorFromPalette
                self.func148(imports, var42.wrapping_add(var43), var45); // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
                let var46 = var4;
                var4 = var46.wrapping_add(1i32);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/graphics/window/renderWindow
    fn func150<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
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
                let var29 = self.func146(imports, var24, var28); // wasm/graphics/renderUtils/getTileDataAddress
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
                let var48 = self.func147(imports, var47, 65351i32); // wasm/graphics/renderUtils/getColorFromPalette
                self.func148(imports, var45.wrapping_add(var46), var48); // wasm/memory/memory/setPixelOnFrameDirectlyToWasmMemory
                let var49 = var3;
                var3 = var49.wrapping_add(1i32) & 65535i32;
                continue 'label0;
            }
            break;
        }
    }
    // wasm/memory/memory/getPixelOnFrame
    fn func151<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var1;
        let var3 = var0;
        let var4 = self.memory.load8(var2.wrapping_mul(160i32).wrapping_add(187904i32).wrapping_add(var3) as usize) as i32;
        var4
    }
    // wasm/memory/memory/setPixelOnFrame
    fn func152<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) {
        let var3 = var1;
        let var4 = var0;
        let var5 = var2;
        self.memory.store8(var3.wrapping_mul(160i32).wrapping_add(187904i32).wrapping_add(var4) as usize, var5.wrapping_add(1i32) as u8);
    }
    // wasm/graphics/sprites/renderSprites
    fn func153<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
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
                    let var46 = self.func146(imports, 32768i32, var45); // wasm/graphics/renderUtils/getTileDataAddress
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
                                let var66 = self.func147(imports, var64, var65); // wasm/graphics/renderUtils/getColorFromPalette
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
                                    let var75 = self.func151(imports, var73, var74); // wasm/memory/memory/getPixelOnFrame
                                    var71 = (var75 as u32 <= 1i32 as u32) as i32;
                                }
                                if var71 & 1i32 != 0 {
                                    let var76 = var4;
                                    let var77 = var0;
                                    let var78 = var3;
                                    self.func152(imports, var76, var77, var78); // wasm/memory/memory/setPixelOnFrame
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
    fn func154<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        var3 = 34816i32;
        let var4 = self.func13(imports, 65344i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
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
            self.func149(imports, var11, var12, var13); // wasm/graphics/background/renderBackground
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
            self.func150(imports, var18, var19, var20); // wasm/graphics/window/renderWindow
        }
        let var21 = var1;
        let var22 = self.func2(imports, 1i32, var21); // wasm/helpers/index/checkBitOnByte
        if var22 != 0 {
            let var23 = var0;
            let var24 = var1;
            let var25 = self.func2(imports, 2i32, var24); // wasm/helpers/index/checkBitOnByte
            self.func153(imports, var23, var25); // wasm/graphics/sprites/renderSprites
        }
    }
    // wasm/memory/memory/storeFrameToBeRendered
    fn func155<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
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
    fn func156<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func18(imports, 0i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/graphics/graphics/updateGraphics
    fn func157<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let var2 = self.func143(imports); // wasm/graphics/lcd/isLcdEnabled
        var1 = var2;
        let var3 = var1;
        self.func145(imports, var3); // wasm/graphics/lcd/setLcdStatus
        let var4 = var1;
        if var4 != 0 {
            let var5 = self.global72;
            let var6 = var0;
            self.global72 = var5.wrapping_add(var6);
            let var7 = self.global72;
            if (var7 >= 456i32) as i32 != 0 {
                let var8 = self.global72;
                self.global72 = var8.wrapping_sub(456i32);
                let var9 = self.func13(imports, 65348i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
                var0 = var9;
                let var10 = var0;
                if (var10 == 144i32) as i32 != 0 {
                    let var11 = var0;
                    self.func154(imports, var11); // wasm/graphics/graphics/_drawScanline
                    self.func155(imports); // wasm/memory/memory/storeFrameToBeRendered
                    self.func156(imports); // wasm/interrupts/index/requestVBlankInterrupt
                } else {
                    let var12 = var0;
                    if ((var12 as u32) < 144i32 as u32) as i32 != 0 {
                        let var13 = var0;
                        self.func154(imports, var13); // wasm/graphics/graphics/_drawScanline
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
    fn func158<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        self.func136(imports, 0i32); // wasm/interrupts/index/setInterrupts
        let var1 = var0;
        let var2 = self.func13(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
        let var3 = self.func72(imports, var1, var2); // wasm/helpers/index/resetBitOnByte
        self.func8(imports, 65295i32, var3); // wasm/memory/store/eightBitStoreIntoGBMemorySkipTraps
        let var4 = self.global9;
        self.global9 = var4.wrapping_sub(2i32) & 65535i32;
        let var5 = self.global9;
        let var6 = self.global0;
        self.func11(imports, var5, var6); // wasm/memory/store/sixteenBitStoreIntoGBMemorySkipTraps
        'label0: loop {
            'label1: loop {
                'label2: loop {
                    'label3: loop {
                        'label4: loop {
                            'label5: loop {
                                let var7 = var0;
                                match var7 {
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
                        self.global0 = 64i32;
                        break 'label0;
                        break;
                    }
                    self.global0 = 72i32;
                    break 'label0;
                    break;
                }
                self.global0 = 80i32;
                break 'label0;
                break;
            }
            self.global0 = 96i32;
            break;
        }
        self.global69 = 0i32;
    }
    // wasm/interrupts/index/checkInterrupts
    fn func159<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let var4 = self.global71;
        if var4 != 0 {
            let var5 = self.func13(imports, 65295i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
            var2 = var5;
            let var6 = self.func13(imports, 65535i32); // wasm/memory/load/eightBitLoadFromGBMemorySkipTraps
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
                    self.func158(imports, 0i32); // wasm/interrupts/index/_handleInterrupt
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
                        self.func158(imports, 1i32); // wasm/interrupts/index/_handleInterrupt
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
                            self.func158(imports, 2i32); // wasm/interrupts/index/_handleInterrupt
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
                                self.func158(imports, 4i32); // wasm/interrupts/index/_handleInterrupt
                                var1 = 1i32;
                            }
                        }
                    }
                }
            }
            let var36 = var1;
            if var36 != 0 {
                return 20i32;
            }
        }
        0i32
    }
    // wasm/cpu/opcodes/emulationStep
    fn func160<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32) -> i32 {
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        var3 = 4i32;
        let var6 = self.global69;
        var5 = (var6 == 0) as i32;
        let var7 = var5;
        let var8: i32;
        if var7 != 0 {
            let var9 = self.global70;
            var8 = (var9 == 0) as i32;
        } else {
            let var10 = var5;
            var8 = var10;
        }
        if var8 & 1i32 != 0 {
            let var11 = self.global0;
            let var12 = self.func75(imports, var11); // wasm/memory/load/eightBitLoadFromGBMemory
            var4 = var12;
            let var13 = var4;
            let var14 = self.global0;
            let var15 = self.func75(imports, var14.wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
            let var16 = self.global0;
            let var17 = self.func75(imports, var16.wrapping_add(2i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
            let var18 = self.func140(imports, var13, var15, var17); // wasm/cpu/opcodes/executeOpcode
            var3 = var18;
        } else {
            let var19 = self.global69;
            let var20: i32;
            if var19 != 0 {
                let var21 = self.func141(imports); // wasm/interrupts/index/areInterruptsEnabled
                var20 = (var21 == 0) as i32;
            } else {
                let var22 = self.global69;
                var20 = var22;
            }
            var5 = var20 & 1i32;
            let var23 = var5;
            let var24: i32;
            if var23 != 0 {
                let var25 = self.func142(imports); // wasm/interrupts/index/areInterruptsPending
                var24 = var25;
            } else {
                let var26 = var5;
                var24 = var26;
            }
            if var24 & 1i32 != 0 {
                self.global69 = 0i32;
                self.global70 = 0i32;
                let var27 = self.global0;
                let var28 = self.func75(imports, var27); // wasm/memory/load/eightBitLoadFromGBMemory
                var4 = var28;
                let var29 = var4;
                let var30 = self.global0;
                let var31 = self.func75(imports, var30); // wasm/memory/load/eightBitLoadFromGBMemory
                let var32 = self.global0;
                let var33 = self.func75(imports, var32.wrapping_add(1i32) & 65535i32); // wasm/memory/load/eightBitLoadFromGBMemory
                let var34 = self.func140(imports, var29, var31, var33); // wasm/cpu/opcodes/executeOpcode
                var3 = var34;
                let var35 = self.global0;
                self.global0 = var35.wrapping_sub(1i32) & 65535i32;
            }
        }
        let var36 = self.global2;
        self.global2 = var36 & 240i32;
        let var37 = var2;
        if (var37 == 0) as i32 != 0 {
            let var38 = var3;
            self.func20(imports, var38); // wasm/timers/index/updateTimers
        }
        let var39 = self.global70;
        if (var39 == 0) as i32 != 0 {
            let var40 = var1;
            if (var40 == 0) as i32 != 0 {
                let var41 = var3;
                self.func157(imports, var41); // wasm/graphics/graphics/updateGraphics
            }
            let var42 = var0;
            if (var42 == 0) as i32 != 0 {
                let var43 = var3;
                self.func60(imports, var43); // wasm/sound/sound/updateSound
            }
        }
        let var44 = var3;
        let var45 = self.func159(imports); // wasm/interrupts/index/checkInterrupts
        var3 = var44.wrapping_add(var45).wrapping_shl(24i32 as u32).wrapping_shr(24i32 as u32);
        let var46 = var3;
        if (var46 <= 0i32) as i32 != 0 {
            let var47 = var4;
            self.func1(imports, 8861764i32, 1i32, var47, 0i32, 0i32, 0i32, 0i32); // wasm/helpers/index/log
        }
        let var48 = var3;
        var48
    }
    // wasm/graphics/graphics/batchProcessGraphics
    fn func161<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.global73;
        let var1 = self.global74;
        if ((var0) < var1) as i32 != 0 {
            return;
        }
        'label0: loop {
            let var2 = self.global73;
            let var3 = self.global74;
            if (var2 >= var3) as i32 != 0 {
                let var4 = self.global74;
                self.func157(imports, var4); // wasm/graphics/graphics/updateGraphics
                let var5 = self.global73;
                let var6 = self.global74;
                self.global73 = var5.wrapping_sub(var6);
                continue 'label0;
            }
            break;
        }
    }
    // wasm/cpu/opcodes/update
    fn func162<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let mut var0: i32 = 0;
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        let mut var4: i32 = 0;
        let var5 = self.global65;
        var1 = var5;
        let var6 = self.global66;
        var2 = var6;
        let var7 = self.global67;
        var3 = var7;
        'label0: loop {
            let var8 = var4;
            var0 = (var8 == 0) as i32;
            let var9 = var0;
            let var10: i32;
            if var9 != 0 {
                let var11 = self.global68;
                var10 = ((var11) < 69905i32) as i32;
            } else {
                let var12 = var0;
                var10 = var12;
            }
            if var10 & 1i32 != 0 {
                let var13 = var1;
                let var14 = var2;
                let var15 = var3;
                let var16 = self.func160(imports, var13, var14, var15); // wasm/cpu/opcodes/emulationStep
                var0 = var16;
                let var17 = var0;
                if (var17 >= 0i32) as i32 != 0 {
                    let var18 = self.global68;
                    let var19 = var0;
                    self.global68 = var18.wrapping_add(var19);
                    let var20 = var1;
                    if var20 != 0 {
                        let var21 = self.global25;
                        let var22 = var0;
                        self.global25 = var21.wrapping_add(var22);
                    }
                    let var23 = var2;
                    if var23 != 0 {
                        let var24 = self.global73;
                        let var25 = var0;
                        self.global73 = var24.wrapping_add(var25);
                        self.func161(imports); // wasm/graphics/graphics/batchProcessGraphics
                    }
                    let var26 = var3;
                    if var26 != 0 {
                        let var27 = self.global22;
                        let var28 = var0;
                        self.global22 = var27.wrapping_add(var28);
                        self.func21(imports); // wasm/timers/index/batchProcessTimers
                    }
                    let var29 = self.global22;
                    let var30 = var0;
                    self.global22 = var29.wrapping_add(var30);
                } else {
                    var4 = 1i32;
                }
                continue 'label0;
            }
            break;
        }
        let var31 = self.global68;
        if (var31 >= 69905i32) as i32 != 0 {
            self.global68 = 0i32;
            return 1i32;
        }
        let var32 = self.global0;
        self.global0 = var32.wrapping_sub(1i32) & 65535i32;
        -1i32
    }
    // wasm/joypad/index/_getJoypadButtonStateFromButtonId
    fn func163<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
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
                                                let var1 = var0;
                                                match var1 {
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
                                        let var2 = self.global61;
                                        return var2;
                                        break;
                                    }
                                    let var3 = self.global62;
                                    return var3;
                                    break;
                                }
                                let var4 = self.global63;
                                return var4;
                                break;
                            }
                            let var5 = self.global64;
                            return var5;
                            break;
                        }
                        let var6 = self.global57;
                        return var6;
                        break;
                    }
                    let var7 = self.global58;
                    return var7;
                    break;
                }
                let var8 = self.global59;
                return var8;
                break;
            }
            let var9 = self.global60;
            return var9;
            break;
        }
        0i32
    }
    // wasm/joypad/index/_setJoypadButtonStateFromButtonId
    fn func164<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
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
                                                let var2 = var0;
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
                                        let var3 = var1;
                                        self.global61 = var3;
                                        break 'label0;
                                        break;
                                    }
                                    let var4 = var1;
                                    self.global62 = var4;
                                    break 'label0;
                                    break;
                                }
                                let var5 = var1;
                                self.global63 = var5;
                                break 'label0;
                                break;
                            }
                            let var6 = var1;
                            self.global64 = var6;
                            break 'label0;
                            break;
                        }
                        let var7 = var1;
                        self.global57 = var7;
                        break 'label0;
                        break;
                    }
                    let var8 = var1;
                    self.global58 = var8;
                    break 'label0;
                    break;
                }
                let var9 = var1;
                self.global59 = var9;
                break 'label0;
                break;
            }
            let var10 = var1;
            self.global60 = var10;
            break;
        }
    }
    // wasm/interrupts/index/requestJoypadInterrupt
    fn func165<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func18(imports, 4i32); // wasm/interrupts/index/_requestInterrupt
    }
    // wasm/joypad/index/_pressJoypadButton
    fn func166<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let mut var1: i32 = 0;
        let mut var2: i32 = 0;
        let mut var3: i32 = 0;
        self.global70 = 0i32;
        let var4 = var0;
        let var5 = self.func163(imports, var4); // wasm/joypad/index/_getJoypadButtonStateFromButtonId
        if (var5 == 0) as i32 != 0 {
            var1 = 1i32;
        }
        let var6 = var0;
        self.func164(imports, var6, 1i32); // wasm/joypad/index/_setJoypadButtonStateFromButtonId
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
                self.func165(imports); // wasm/interrupts/index/requestJoypadInterrupt
            }
        }
    }
    // wasm/joypad/index/_releaseJoypadButton
    fn func167<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) {
        let var1 = var0;
        self.func164(imports, var1, 0i32); // wasm/joypad/index/_setJoypadButtonStateFromButtonId
    }
    // wasm/joypad/index/setJoypadState
    fn func168<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32, mut var2: i32, mut var3: i32, mut var4: i32, mut var5: i32, mut var6: i32, mut var7: i32) {
        let var8 = var0;
        if (var8 > 0i32) as i32 != 0 {
            self.func166(imports, 0i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 0i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var9 = var1;
        if (var9 > 0i32) as i32 != 0 {
            self.func166(imports, 1i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 1i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var10 = var2;
        if (var10 > 0i32) as i32 != 0 {
            self.func166(imports, 2i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 2i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var11 = var3;
        if (var11 > 0i32) as i32 != 0 {
            self.func166(imports, 3i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 3i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var12 = var4;
        if (var12 > 0i32) as i32 != 0 {
            self.func166(imports, 4i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 4i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var13 = var5;
        if (var13 > 0i32) as i32 != 0 {
            self.func166(imports, 5i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 5i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var14 = var6;
        if (var14 > 0i32) as i32 != 0 {
            self.func166(imports, 6i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 6i32); // wasm/joypad/index/_releaseJoypadButton
        }
        let var15 = var7;
        if (var15 > 0i32) as i32 != 0 {
            self.func166(imports, 7i32); // wasm/joypad/index/_pressJoypadButton
        } else {
            self.func167(imports, 7i32); // wasm/joypad/index/_releaseJoypadButton
        }
    }
    // wasm/sound/sound/getAudioQueueIndex
    fn func169<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global55;
        var0
    }
    // wasm/sound/sound/resetAudioQueue
    fn func170<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.global55 = 0i32;
    }
    // wasm/memory/memory/getSaveStateMemoryOffset
    fn func171<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) -> i32 {
        let var2 = var0;
        let var3 = var1;
        var2.wrapping_add(var3.wrapping_mul(50i32) & 65535i32) & 65535i32
    }
    // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    fn func172<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32, mut var1: i32) {
        let var2 = var1;
        if var2 != 0 {
            let var3 = var0;
            self.memory.store8(var3 as usize, 1i32 as u8);
        } else {
            let var4 = var0;
            self.memory.store8(var4 as usize, 0i32 as u8);
        }
    }
    // wasm/cpu/cpu/Cpu.saveState
    fn func173<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global1;
        self.memory.store8(var0 as usize, var1 as u8);
        let var2 = self.func171(imports, 1i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global3;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.func171(imports, 2i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global4;
        self.memory.store8(var4 as usize, var5 as u8);
        let var6 = self.func171(imports, 3i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global5;
        self.memory.store8(var6 as usize, var7 as u8);
        let var8 = self.func171(imports, 4i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global6;
        self.memory.store8(var8 as usize, var9 as u8);
        let var10 = self.func171(imports, 5i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global7;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func171(imports, 6i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global8;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.func171(imports, 7i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global2;
        self.memory.store8(var14 as usize, var15 as u8);
        let var16 = self.func171(imports, 8i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global9;
        self.memory.store16(var16 as usize, var17 as u16);
        let var18 = self.func171(imports, 10i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.global0;
        self.memory.store16(var18 as usize, var19 as u16);
        let var20 = self.func171(imports, 12i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var21 = self.global68;
        self.memory.store32(var20 as usize, var21 as u32);
        let var22 = self.func171(imports, 17i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var23 = self.global69;
        self.func172(imports, var22, var23); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var24 = self.func171(imports, 18i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var25 = self.global70;
        self.func172(imports, var24, var25); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/graphics/graphics/Graphics.saveState
    fn func174<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global72;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func171(imports, 4i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global19;
        self.memory.store8(var2 as usize, var3 as u8);
    }
    // wasm/interrupts/index/Interrupts.saveState
    fn func175<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global71;
        self.func172(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func171(imports, 1i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global75;
        self.func172(imports, var2, var3); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/joypad/index/Joypad.saveState
    fn func176<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
    }
    // wasm/memory/memory/Memory.saveState
    fn func177<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global14;
        self.memory.store16(var0 as usize, var1 as u16);
        let var2 = self.func171(imports, 2i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global18;
        self.memory.store16(var2 as usize, var3 as u16);
        let var4 = self.func171(imports, 4i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global12;
        self.func172(imports, var4, var5); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var6 = self.func171(imports, 5i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global17;
        self.func172(imports, var6, var7); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var8 = self.func171(imports, 6i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global10;
        self.func172(imports, var8, var9); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var10 = self.func171(imports, 7i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global15;
        self.func172(imports, var10, var11); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var12 = self.func171(imports, 8i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global11;
        self.func172(imports, var12, var13); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var14 = self.func171(imports, 9i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global16;
        self.func172(imports, var14, var15); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var16 = self.func171(imports, 10i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global13;
        self.func172(imports, var16, var17); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
    }
    // wasm/timers/index/Timers.saveState
    fn func178<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global24;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func171(imports, 4i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global21;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func171(imports, 8i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global23;
        self.memory.store32(var4 as usize, var5 as u32);
    }
    // wasm/sound/sound/Sound.saveState
    fn func179<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global27;
        self.memory.store32(var0 as usize, var1 as u32);
        let var2 = self.func171(imports, 4i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global54;
        self.memory.store8(var2 as usize, var3 as u8);
        let var4 = self.func171(imports, 5i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global28;
        self.memory.store8(var4 as usize, var5 as u8);
    }
    // wasm/sound/channel1/Channel1.saveState
    fn func180<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global30;
        self.func172(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func171(imports, 1i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global46;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func171(imports, 5i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global40;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func171(imports, 9i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global29;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func171(imports, 14i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global41;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func171(imports, 19i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global76;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func171(imports, 20i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global47;
        self.memory.store8(var12 as usize, var13 as u8);
        let var14 = self.func171(imports, 25i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.global38;
        self.func172(imports, var14, var15); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var16 = self.func171(imports, 26i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.global37;
        self.memory.store32(var16 as usize, var17 as u32);
        let var18 = self.func171(imports, 31i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.global39;
        self.memory.store16(var18 as usize, var19 as u16);
    }
    // wasm/sound/channel2/Channel2.saveState
    fn func181<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global32;
        self.func172(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func171(imports, 1i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global48;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func171(imports, 5i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global42;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func171(imports, 9i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global31;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func171(imports, 14i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global43;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func171(imports, 19i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global77;
        self.memory.store8(var10 as usize, var11 as u8);
        let var12 = self.func171(imports, 20i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.global49;
        self.memory.store8(var12 as usize, var13 as u8);
    }
    // wasm/sound/channel3/Channel3.saveState
    fn func182<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global34;
        self.func172(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func171(imports, 1i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global50;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func171(imports, 5i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global33;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func171(imports, 9i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global51;
        self.memory.store16(var6 as usize, var7 as u16);
    }
    // wasm/sound/channel4/Channel4.saveState
    fn func183<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.global36;
        self.func172(imports, var0, var1); // wasm/memory/store/storeBooleanDirectlyToWasmMemory
        let var2 = self.func171(imports, 1i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.global52;
        self.memory.store32(var2 as usize, var3 as u32);
        let var4 = self.func171(imports, 5i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.global44;
        self.memory.store32(var4 as usize, var5 as u32);
        let var6 = self.func171(imports, 9i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.global35;
        self.memory.store32(var6 as usize, var7 as u32);
        let var8 = self.func171(imports, 14i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.global45;
        self.memory.store32(var8 as usize, var9 as u32);
        let var10 = self.func171(imports, 19i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.global53;
        self.memory.store16(var10 as usize, var11 as u16);
    }
    // wasm/index/saveState
    fn func184<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func173(imports); // wasm/cpu/cpu/Cpu.saveState
        self.func174(imports); // wasm/graphics/graphics/Graphics.saveState
        self.func175(imports); // wasm/interrupts/index/Interrupts.saveState
        self.func176(imports); // wasm/joypad/index/Joypad.saveState
        self.func177(imports); // wasm/memory/memory/Memory.saveState
        self.func178(imports); // wasm/timers/index/Timers.saveState
        self.func179(imports); // wasm/sound/sound/Sound.saveState
        self.func180(imports); // wasm/sound/channel1/Channel1.saveState
        self.func181(imports); // wasm/sound/channel2/Channel2.saveState
        self.func182(imports); // wasm/sound/channel3/Channel3.saveState
        self.func183(imports); // wasm/sound/channel4/Channel4.saveState
    }
    // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
    fn func185<I: Imports<Memory = M>>(&mut self, imports: &mut I, mut var0: i32) -> i32 {
        let var1 = var0;
        let var2 = self.memory.load8(var1 as usize) as i32;
        if (var2 as u32 > 0i32 as u32) as i32 != 0 {
            return 1i32;
        }
        0i32
    }
    // wasm/cpu/cpu/Cpu.loadState
    fn func186<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load8(var0 as usize) as i32;
        self.global1 = var1;
        let var2 = self.func171(imports, 1i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global3 = var3;
        let var4 = self.func171(imports, 2i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global4 = var5;
        let var6 = self.func171(imports, 3i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load8(var6 as usize) as i32;
        self.global5 = var7;
        let var8 = self.func171(imports, 4i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load8(var8 as usize) as i32;
        self.global6 = var9;
        let var10 = self.func171(imports, 5i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global7 = var11;
        let var12 = self.func171(imports, 6i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global8 = var13;
        let var14 = self.func171(imports, 7i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.memory.load8(var14 as usize) as i32;
        self.global2 = var15;
        let var16 = self.func171(imports, 8i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.memory.load16(var16 as usize) as i32;
        self.global9 = var17;
        let var18 = self.func171(imports, 10i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global0 = var19;
        let var20 = self.func171(imports, 12i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var21 = self.memory.load32(var20 as usize) as i32;
        self.global68 = var21;
        let var22 = self.func171(imports, 17i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var23 = self.func185(imports, var22); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global69 = var23;
        let var24 = self.func171(imports, 18i32, 0i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var25 = self.func185(imports, var24); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global70 = var25;
    }
    // wasm/graphics/graphics/Graphics.loadState
    fn func187<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global72 = var1;
        let var2 = self.func171(imports, 4i32, 1i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global19 = var3;
    }
    // wasm/interrupts/index/Interrupts.loadState
    fn func188<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func185(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global71 = var1;
        let var2 = self.func171(imports, 1i32, 2i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.func185(imports, var2); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global75 = var3;
    }
    // wasm/memory/memory/Memory.loadState
    fn func189<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load16(var0 as usize) as i32;
        self.global14 = var1;
        let var2 = self.func171(imports, 2i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load16(var2 as usize) as i32;
        self.global18 = var3;
        let var4 = self.func171(imports, 4i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.func185(imports, var4); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global12 = var5;
        let var6 = self.func171(imports, 5i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.func185(imports, var6); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global17 = var7;
        let var8 = self.func171(imports, 6i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.func185(imports, var8); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global10 = var9;
        let var10 = self.func171(imports, 7i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.func185(imports, var10); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global15 = var11;
        let var12 = self.func171(imports, 8i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.func185(imports, var12); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global11 = var13;
        let var14 = self.func171(imports, 9i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.func185(imports, var14); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global16 = var15;
        let var16 = self.func171(imports, 10i32, 4i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.func185(imports, var16); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global13 = var17;
    }
    // wasm/timers/index/Timers.loadState
    fn func190<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global24 = var1;
        let var2 = self.func171(imports, 4i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global21 = var3;
        let var4 = self.func171(imports, 8i32, 5i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global23 = var5;
    }
    // wasm/sound/sound/Sound.loadState
    fn func191<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.memory.load32(var0 as usize) as i32;
        self.global27 = var1;
        let var2 = self.func171(imports, 4i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load8(var2 as usize) as i32;
        self.global54 = var3;
        let var4 = self.func171(imports, 5i32, 6i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load8(var4 as usize) as i32;
        self.global28 = var5;
        self.func170(imports); // wasm/sound/sound/resetAudioQueue
    }
    // wasm/sound/channel1/Channel1.loadState
    fn func192<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func185(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global30 = var1;
        let var2 = self.func171(imports, 1i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global46 = var3;
        let var4 = self.func171(imports, 5i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global40 = var5;
        let var6 = self.func171(imports, 9i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global29 = var7;
        let var8 = self.func171(imports, 14i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global41 = var9;
        let var10 = self.func171(imports, 19i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global76 = var11;
        let var12 = self.func171(imports, 20i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global47 = var13;
        let var14 = self.func171(imports, 25i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var15 = self.func185(imports, var14); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global38 = var15;
        let var16 = self.func171(imports, 26i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var17 = self.memory.load32(var16 as usize) as i32;
        self.global37 = var17;
        let var18 = self.func171(imports, 31i32, 7i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var19 = self.memory.load16(var18 as usize) as i32;
        self.global39 = var19;
    }
    // wasm/sound/channel2/Channel2.loadState
    fn func193<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func185(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global32 = var1;
        let var2 = self.func171(imports, 1i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global48 = var3;
        let var4 = self.func171(imports, 5i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global42 = var5;
        let var6 = self.func171(imports, 9i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global31 = var7;
        let var8 = self.func171(imports, 14i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global43 = var9;
        let var10 = self.func171(imports, 19i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load8(var10 as usize) as i32;
        self.global77 = var11;
        let var12 = self.func171(imports, 20i32, 8i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var13 = self.memory.load8(var12 as usize) as i32;
        self.global49 = var13;
    }
    // wasm/sound/channel3/Channel3.loadState
    fn func194<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func185(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global34 = var1;
        let var2 = self.func171(imports, 1i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global50 = var3;
        let var4 = self.func171(imports, 5i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global33 = var5;
        let var6 = self.func171(imports, 9i32, 9i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load16(var6 as usize) as i32;
        self.global51 = var7;
    }
    // wasm/sound/channel4/Channel4.loadState
    fn func195<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        let var0 = self.func171(imports, 0i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var1 = self.func185(imports, var0); // wasm/memory/load/loadBooleanDirectlyFromWasmMemory
        self.global36 = var1;
        let var2 = self.func171(imports, 1i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var3 = self.memory.load32(var2 as usize) as i32;
        self.global52 = var3;
        let var4 = self.func171(imports, 5i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var5 = self.memory.load32(var4 as usize) as i32;
        self.global44 = var5;
        let var6 = self.func171(imports, 9i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var7 = self.memory.load32(var6 as usize) as i32;
        self.global35 = var7;
        let var8 = self.func171(imports, 14i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var9 = self.memory.load32(var8 as usize) as i32;
        self.global45 = var9;
        let var10 = self.func171(imports, 19i32, 10i32); // wasm/memory/memory/getSaveStateMemoryOffset
        let var11 = self.memory.load16(var10 as usize) as i32;
        self.global53 = var11;
    }
    // wasm/index/loadState
    fn func196<I: Imports<Memory = M>>(&mut self, imports: &mut I) {
        self.func186(imports); // wasm/cpu/cpu/Cpu.loadState
        self.func187(imports); // wasm/graphics/graphics/Graphics.loadState
        self.func188(imports); // wasm/interrupts/index/Interrupts.loadState
        self.func176(imports); // wasm/joypad/index/Joypad.saveState
        self.func189(imports); // wasm/memory/memory/Memory.loadState
        self.func190(imports); // wasm/timers/index/Timers.loadState
        self.func191(imports); // wasm/sound/sound/Sound.loadState
        self.func192(imports); // wasm/sound/channel1/Channel1.loadState
        self.func193(imports); // wasm/sound/channel2/Channel2.loadState
        self.func194(imports); // wasm/sound/channel3/Channel3.loadState
        self.func195(imports); // wasm/sound/channel4/Channel4.loadState
    }
    // wasm/index/getRegisterA
    fn func197<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global1;
        var0
    }
    // wasm/index/getRegisterB
    fn func198<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global3;
        var0
    }
    // wasm/index/getRegisterC
    fn func199<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global4;
        var0
    }
    // wasm/index/getRegisterD
    fn func200<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global5;
        var0
    }
    // wasm/index/getRegisterE
    fn func201<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global6;
        var0
    }
    // wasm/index/getRegisterH
    fn func202<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global7;
        var0
    }
    // wasm/index/getRegisterL
    fn func203<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global8;
        var0
    }
    // wasm/index/getRegisterF
    fn func204<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global2;
        var0
    }
    // wasm/index/getProgramCounter
    fn func205<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global0;
        var0
    }
    // wasm/index/getStackPointer
    fn func206<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global9;
        var0
    }
    // wasm/index/getPreviousOpcode
    fn func207<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global78;
        var0
    }
    // wasm/index/getOpcodeAtProgramCounter
    fn func208<I: Imports<Memory = M>>(&mut self, imports: &mut I) -> i32 {
        let var0 = self.global0;
        let var1 = self.func75(imports, var0); // wasm/memory/load/eightBitLoadFromGBMemory
        var1
    }
}

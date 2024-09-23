use crate::{
    cpu::{go, step, Cpu},
    operand::{Reg16, IO16, IO8},
    peripherals::Peripherals,
};

use std::{result, sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed}};

impl Cpu {
    // 何もしないという命令
    // fetch/execute overlapにしたがい次の命令をfetchする
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
    // ldはsから読み込んでdに書き込む
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                VAL8.store(v, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, dst, VAL8.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(1);
                self.fetch(bus);
            },
        });
    }
    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v, Relaxed);
                go!(1);
            },
            1: if self.write16(bus, dst, VAL16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }
    // Aレジスタとsの値を比較する
    // Zフラグ：演算結果が0の場合1にする
    // N 無条件で1にする
    // H 4bit目からの繰り下がりが発生した場合は1にする
    // C 8bit目からの繰り下がりが発生した場合は1にする
    pub fn cp<S: Copy>(&mut self, bus: &Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);
            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            // 0xfは00001111
            // & 0xfで下位4ビットを抽出できる
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);
            self.fetch(bus);
        }
    }
    // sをインクリメントする
    // Z 結果が0のとき1
    // N 無条件で0
    // 3bit目で繰り上がりが発生した場合は1にする
    // C変更しない
    pub fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_add(1);
                self.regs.set_zf(result==0);
                self.regs.set_nf(false);
                self.regs.set_hf(v & 0xf == 0xf);
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }
    pub fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
              VAL16.store(v.wrapping_add(1), Relaxed);  
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });

    }
    // dec　sをデクリメントする
    // 
    pub fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = v.wrapping_sub(1);
                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf(v & 0xf == 0);
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }
    pub fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where Self: IO16<S> {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v.wrapping_sub(1), Relaxed);
                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                return go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
         });
    }
    // RL sの値とCフラッグを合わせた9ビットの値を左に回転する
    // Z演算結果が0の場合は1にする
    // N　無条件で0にする
    // H　無条件で0にする
    // C 演算するまえのsの7bit目が1出会った場合は1にする
    pub fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src:S)
    where Self: IO8<S> {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                let result = (v << 1) | self.regs.cf() as u8;
                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(v & 0x80 > 0); // 0x80 10000000
                VAL8.store(result, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }  
    pub fn bit<S: Copy>(&mut self, bus: &Peripherals, bit: usize, src: S)
    where Self: IO8<S> {
        if let Some(mut v) = self.read8(bus, src) {
            v &= 1 << bit;
            self.regs.set_zf(v == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);
            self.fetch(bus);
        }
    }
    pub fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        step!(None, {
            0: {
                go!(1);
                return None;
            },
            1: {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, hi);
                VAL8.store(lo, Relaxed);
                go!(2);
                return None;
            },
            2: {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, VAL8.load(Relaxed));
                go!(3);
                return None;
            },
            3: {
                return Some(go!(0));
            },
        });
    }
    pub fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        step!((), {
            0: {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);
                go!(1);
            },
            1: if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                go!(0);
                self.fetch(bus);
            },
        });
    }
}

use crate::{
    cpu::{go, step, Cpu},
    operand::{IO16, IO8},
    peripherals::Peripherals,
};

use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

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
}

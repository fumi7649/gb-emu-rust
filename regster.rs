#[derive(Clone, Copy, Debug, Default)]

pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}
impl Registers {
    // 8bitレジスタをけつごうして16ビットレジスタとして扱うこともある
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16) // aを8ビット左シフトして、ORで結合する
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.b as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn zf(&self) -> bool {
        (self.f & 0b_1000_0000) > 0
    }
    pub fn set_zf(&mut self, zf: bool) {
        if zf {
            self.f |= 0b_1000_0000;
        } else {
            self.f &= 0b_0111_1111;
        }
    }
}
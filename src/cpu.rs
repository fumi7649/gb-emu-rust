#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}
pub struct Cpu {
    regs: Register,
    ctx: Ctx,
}
impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc); // プログラムカウンタのポインタにreadしに行く
        self.regs.pc = self.regs.pc.wrapping_add(1); // プログラムカウンタを進める
        self.ctx.cb = false;
    }
    pub fn decode(&mut self, bus: &mut &Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _    => panic!("Not implemented: {:02x}", self.opcode)
        }
    }
}

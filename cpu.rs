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
    // 何もしないという命令
    // fetch/execute overlapにしたがい次の命令をfetchする
    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}

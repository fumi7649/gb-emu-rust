macro_rules! step {
    ($d:expr, {$($c:tt : $e:expr,)*}) => {
        static STEP: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL8: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL16: AtomicU16 = AtomicU16::new(0);
        $(if STEP.load(Relaxed) == $c { $e })* else { return $d; }
    };
}
pub(crate) use step;
macro_rules! go {
    ($e:expr) => {
        STEP.store($e, Relaxed)
    };
}
pub(crate) use go;

use crate::{peripherals::Peripherals, register::Registers};

#[derive(Default, Clone)]
struct Ctx {
    opcode: u8,
    cb: bool,
}
#[derive(Clone)]
pub struct Cpu {
    pub regs: Registers,
    ctx: Ctx,
}
impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc); // プログラムカウンタのポインタにreadしに行く
        self.regs.pc = self.regs.pc.wrapping_add(1); // プログラムカウンタを進める
        self.ctx.cb = false;
    }
    pub fn decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02x}", self.ctx.opcode),
        }
    }
}

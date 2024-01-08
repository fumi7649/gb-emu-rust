pub struct HRam(Box<[u8; 0x80]>);
impl HRam {
  pub fn new() -> Self {
    Self(Box::new([0; 0x80]))
  }
  pub fn read(&self, addr: u16) -> u8 {
    self.0
  }
}
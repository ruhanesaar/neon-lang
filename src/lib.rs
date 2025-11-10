struct Buffer(&[u8; 256]);
impl Buffer {
    fn new() -> Self {
        Stack(&[0; 256])
    }
    fn set(&mut self, index: u8, value: u8) {
        self.0[index] = value;
    }
    fn get(&self, index: u8) -> u8 {
        self.0[index]
    }
    fn clear(&mut self, index: u8, end: u8) {
        self.0[index..end].fill(0u8);
    }
}

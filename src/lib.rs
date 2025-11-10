pub struct Buffer<'a>(pub &'a [u8; 256]);
impl Buffer {
    pub fn new() -> Self {
        Self(&[0; 256])
    }
    pub fn set(&mut self, index: u8, value: u8) {
        self.0[index] = value;
    }
    pub fn get(&self, index: u8) -> u8 {
        self.0[index]
    }
    pub fn clear(&mut self, index: u8, end: u8) {
        self.0[index..end].fill(0u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_to_buffer_works() {
        let mut buffer = Buffer::new();
        buffer.set(0, 42);
        assert_eq!(buffer.get(0), 42);
    }

    #[test]
    fn clear_buffer_works() {
        let mut buffer = Buffer::new();
        buffer.clear(0, 10);
        assert_eq!(buffer.get(0), 0);
        assert_eq!(buffer.get(9), 0);
    }
}
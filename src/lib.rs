/// A Tuple Struct `Buffer`
/// This is (Basically) Equivalent to `[u8; 256]` 
pub struct Buffer(pub [u8; 256]);
impl Buffer {
    /// Creates a New `Buffer` with Value `Buffer([0u8; 256])`
    pub fn new() -> Self {
        Self([0; 256])
    }
    /// Changes the value in the given index
    /// # Examples
    /// ```
    /// use neon_lang::Buffer;
    /// let mut buf = Buffer::new();
    /// buf.set(0, 64);
    /// // The zeroeth value is now 64. 
    /// println!("{}", (buf.0)[0]); // 64
    /// ```
    pub fn set(&mut self, index: u8, value: u8) {
        (self.0)[index as usize] = value;
    }
    /// Gets the value in the index
    /// # Examples
    /// ```
    /// use neon_lang::Buffer;
    /// let mut buf = Buffer::new();
    /// (buf.0)[0] = 64;
    /// println!("{}", buf.get(0)); // 64
    /// // Note: This is the same as the example of Buffer::set()
    /// ```
    pub fn get(&self, index: u8) -> u8 {
        (self.0)[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_to_buffer_works() {
        let mut buffer = Buffer::new();
        buffer.set(0, 42);
        assert_eq!((buffer.0)[0], 42);
    }

    #[test]
    fn get_from_buffer_works() {
        let mut buffer = Buffer::new();
        (buffer.0)[0] = 42;
        assert_eq!(buffer.get(0), 42);
    }
}
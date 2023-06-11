use adler32::RollingAdler32;

pub trait RollingHasher {
    /// Creates an empty hasher context.
    fn new() -> Self;

    /// Returns the current hash.
    fn hash(&self) -> u32;

    /// Resets the state.
    fn reset(&mut self);

    /// Removes the given `byte` that was fed to the algorithm `size` bytes ago.
    fn remove(&mut self, size: usize, byte: u8);

    /// Feeds a new `byte` to the algorithm to update the hash.
    fn update(&mut self, byte: u8);

    /// Feeds a vector of bytes to the algorithm to update the hash.
    fn update_buffer(&mut self, buffer: &[u8]);
}

impl RollingHasher for RollingAdler32 {
    fn new() -> Self {
        RollingAdler32::new()
    }

    fn hash(&self) -> u32 {
        RollingAdler32::hash(self)
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn remove(&mut self, size: usize, byte: u8) {
        RollingAdler32::remove(self, size, byte)
    }

    fn update(&mut self, byte: u8) {
        RollingAdler32::update(self, byte)
    }

    fn update_buffer(&mut self, buffer: &[u8]) {
        RollingAdler32::update_buffer(self, buffer)
    }
}

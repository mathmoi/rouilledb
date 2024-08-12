use rand::Rng;
use std::default::Default;

/// Represents a random collections of bytes.
///
/// This can be used during testing when non-specific data is needed.
pub struct RandomBlob {
    data: Vec<u8>,
}

impl RandomBlob {
    const DEFAULT_LENGTH: usize = 128;

    /// Create a new random blob with a specified size.
    pub fn new(length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<u8> = { (0..length).map(|_| rng.gen()).collect() };
        RandomBlob { data }
    }

    /// Returns a reference to the internal data.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns the size of the [RandomBlob].
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Default for RandomBlob {
    // Create a [RandomBlob] with an unspecified, non-zero, size.
    fn default() -> Self {
        RandomBlob::new(RandomBlob::DEFAULT_LENGTH)
    }
}

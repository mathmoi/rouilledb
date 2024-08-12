use rand::Rng;

/// Represents a random collections of bytes.
///
/// This can be used during testing when non-specific data is needed.
pub struct RandomBlob {
    data: Vec<u8>,
}

impl RandomBlob {
    const DEFAULT_LENGTH: usize = 512;

    /// Create a new random blob with a specified size.
    pub fn new(length: Option<usize>) -> Self {
        let length = length.unwrap_or(Self::DEFAULT_LENGTH);

        let mut rng = rand::thread_rng();
        let data: Vec<u8> = { (0..length).map(|_| rng.gen()).collect() };
        RandomBlob { data }
    }

    /// Create a new [RandomBlob] with an unspecified size.
    pub fn new_default() -> Self {
        Self::new(None)
    }

    /// Returns a reference to the [RandomBlob] data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the size of the [RandomBlob].
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

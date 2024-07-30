use crate::fs::file::*;

/// Represents a file in memory.
/// 
/// This struct that implements the [File] trait does not represents a real file. Instead it 
/// represents a filed stored in memory. This can be used during testing to create a fast ephemeral
/// file that does not depends on the operating system or the file system.
pub struct MemoryFile {
    is_opened: bool,
}

impl MemoryFile {
    /// Creates a new [MemoryFile].
    pub fn new() -> Self {
        MemoryFile { is_opened: false }
    }
}

impl File for MemoryFile {
    /// Creates and open the file
    /// 
    /// # Errors
    /// 
    /// This method will return an error if:
    /// - The file is already opened.
    /// - The file already exists
    fn create(&mut self) -> Result<(), FileError> {
        if self.is_opened {
            return Err(FileError::FileAlreadyOpened(String::from("MemoryFile")));
        }
        self.is_opened = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that when called once, the create method succeeds.
    #[test]
    fn create_called_once_succeeds() {
        let mut file = MemoryFile::new();

        let result = file.create();

        assert!(result.is_ok());
    }

    /// Tests that subsequents calls to create will fail withe a [File:Error::FileAlreadyOpened] 
    /// error.
    #[test]
    fn create_called_twice_fails() {
        let mut file = MemoryFile::new();

        file.create().expect("this should not fail");
        let result = file.create();

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileAlreadyOpened(_))));
    }
}

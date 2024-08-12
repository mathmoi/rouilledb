use std::u8;

use crate::fs::file::*;

/// Represents a file in memory.
///
/// This struct that implements the [File] trait does not represents a real file. Instead it
/// represents a filed stored in memory. This can be used during testing to create a fast ephemeral
/// file that does not depends on the operating system or the file system.
pub struct MemoryFile {
    is_opened: bool,
    data: Vec<u8>,
}

impl MemoryFile {
    /// Creates a new [MemoryFile].
    pub fn new() -> Self {
        MemoryFile {
            is_opened: false,
            data: Vec::new(),
        }
    }

    /// Create a new [MemoryFile] with an specified initial content.
    pub fn new_with_data(data: &[u8]) -> Self {
        MemoryFile {
            is_opened: false,
            data: data.to_vec(),
        }
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
    ///
    /// # Example
    ///
    /// ```
    /// use rouilledb::fs::{File, MemoryFile};
    /// let mut file = MemoryFile::new();
    ///
    /// let result = file.create();
    ///
    /// assert!(result.is_ok());
    /// ```
    fn create(&mut self) -> Result<(), FileError> {
        if self.is_opened {
            return Err(FileError::FileOpened(String::from("MemoryFile")));
        }
        self.is_opened = true;
        Ok(())
    }

    /// Closes the file.
    ///
    /// # Errors
    ///
    /// This method will returned an error if the file is not opened
    ///
    /// # Example
    ///
    /// ```
    /// use rouilledb::fs::{File, MemoryFile};
    /// let mut file = MemoryFile::new();
    /// file.create().expect("this should not fail");
    ///
    /// let result = file.close();
    ///
    /// assert!(result.is_ok());
    /// ```
    fn close(&mut self) -> Result<(), FileError> {
        if !self.is_opened {
            return Err(FileError::FileNotOpened(String::from("MemoryFile")));
        }

        self.is_opened = false;
        Ok(())
    }

    /// Opens the file.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - the file is already opened
    /// - the file does not exists
    /// - an unexpected error occurs while opening the file
    ///
    /// # Example
    ///
    /// ```
    /// use rouilledb::fs::{File, MemoryFile};
    /// let mut file = MemoryFile::new();
    ///
    /// let result = file.open();
    ///
    /// assert!(result.is_ok());
    /// ```
    fn open(&mut self) -> Result<(), FileError> {
        if self.is_opened {
            return Err(FileError::FileOpened(String::from("MemoryFile")));
        }

        self.is_opened = true;
        Ok(())
    }

    /// Deletes the file.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - the file is opened
    /// - the file does not exists
    /// - an unexpected error occurs while deleting the file
    ///
    /// # Example
    ///
    /// ```
    /// use rouilledb::fs::{File, MemoryFile};
    /// let mut file = MemoryFile::new();
    ///
    /// let result = file.delete();
    ///
    /// assert!(result.is_ok());
    /// ```
    fn delete(&mut self) -> Result<(), FileError> {
        if self.is_opened {
            return Err(FileError::FileOpened(String::from("MemoryFile")));
        }
        Ok(())
    }

    /// Write a block of data in the file at a specified offset.
    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), FileError> {
        if !self.is_opened {
            return Err(FileError::FileNotOpened(String::from("MemoryFile")));
        }

        let end_offset: usize = offset + data.len();
        if self.data.len() < end_offset {
            self.data.resize(end_offset, 0);
        }

        self.data[offset..end_offset].copy_from_slice(data);

        Ok(())
    }

    /// Read a block of data in the file at a specified offset into a buffer. The size of the data
    /// read is based on the size of the buffer.
    fn read(self, offset: usize, buffer: &mut [u8]) -> Result<(), FileError> {
        if !self.is_opened {
            return Err(FileError::FileNotOpened(String::from("MemoryFile")));
        }

        let end_offset: usize = offset + buffer.len();
        if self.data.len() < end_offset {
            return Err(FileError::EndOfFileRead {
                filename: String::from("MemoryFile"),
                file_size: self.data.len(),
                offset,
                read_size: buffer.len(),
            });
        }

        buffer.copy_from_slice(&self.data[offset..end_offset]);

        Ok(())
    }

    /// Flush all changes to the disk so it will not be lost in case of a crash or power failure.
    ///
    /// # Example
    ///
    /// ```
    /// use rouilledb::common::RandomBlob;
    /// use rouilledb::fs::{File, MemoryFile};
    ///
    /// let blob : RandomBlob = RandomBlob::default();
    /// let mut file = MemoryFile::new();
    /// file.create().expect("create should not fail");
    /// file.write(0, blob.data()).expect("write should not fail");
    ///
    /// let result = file.sync();
    ///
    /// assert!(result.is_ok());
    /// ```
    fn sync(self) -> Result<(), FileError> {
        Ok(())
    }

    /// Get the size of the file.
    ///
    /// # Errors
    ///
    /// This method will return an error if the file is not opened.
    fn size(self) -> Result<usize, FileError> {
        if !self.is_opened {
            return Err(FileError::FileNotOpened(String::from("MemoryFile")));
        }

        Ok(self.data.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::RandomBlob;

    use super::*;

    /// Tests that when called once, the create method succeeds.
    #[test]
    fn create_called_once_succeeds() {
        let mut file = MemoryFile::new();
        let result = file.create();

        assert!(result.is_ok());
    }

    /// Tests that subsequents calls to create will fail with a [File:Error::FileAlreadyOpened]
    /// error.
    #[test]
    fn create_called_twice_fails() {
        let mut file = MemoryFile::new();

        file.create()
            .expect("create should not fail when called once");
        let result = file.create();

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileOpened(_))));
    }

    /// Tests that close will fail if the file is not opened.
    #[test]
    fn close_file_is_no_opened_fails() {
        let mut file = MemoryFile::new();

        let result = file.close();

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileNotOpened(_))));
    }

    /// Test that the file can be create, closed, opened then closed again.
    #[test]
    fn create_close_open_close_succeed() {
        let mut file = MemoryFile::new();

        file.create().expect("create should not fail");
        file.close().expect("close should not fail");
        file.open().expect("open should not fail");
        file.close().expect("close should not fail");
    }

    /// Test that deleting an opened file will return an error.
    #[test]
    fn delete_an_opened_file_fails() {
        let mut file = MemoryFile::new();
        file.create().expect("create should not fail");

        let result = file.delete();

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileOpened(_))));
    }

    /// Writing a block of data to the file succeeds.
    #[test]
    fn write_an_non_zero_blob_succeed() {
        let blob = RandomBlob::default();

        let mut file = MemoryFile::new();
        file.create().expect("create should not fail");

        let result = file.write(0, blob.data());

        assert!(result.is_ok());
    }

    /// Trying to write when the file is not opened fails.
    #[test]
    fn write_file_not_opened_fails() {
        let blob = RandomBlob::default();

        let mut file = MemoryFile::new();

        let result = file.write(0, blob.data());

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileNotOpened(_))));
    }

    /// Trying to read when the file is not opened fails.
    #[test]
    fn read_when_file_not_opened_fails() {
        let file = MemoryFile::new();
        let mut buffer: Vec<u8> = vec![0u8; 512];

        let result = file.read(0, &mut buffer);

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::FileNotOpened(_))));
    }

    /// Reading the content of the whole file succeeds.
    #[test]
    fn read_whole_file_data_is_correctly_read() {
        let content = RandomBlob::new(128);
        let mut file = MemoryFile::new_with_data(content.data());
        let mut buffer = vec![0u8; 128];

        file.open().expect("open should not fail");

        let result = file.read(0, &mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer, content.data());
    }

    /// Reading a part of the file, the data is read correctly
    #[test]
    fn read_part_of_file_data_read_correctly() {
        let read_offset: usize = 32;
        let read_len: usize = 64;
        let content = RandomBlob::new(128);
        let mut file = MemoryFile::new_with_data(content.data());
        let mut buffer = vec![0u8; read_len];

        file.open().expect("open should not fail");

        let result = file.read(read_offset, &mut buffer);

        assert!(result.is_ok());
        assert_eq!(buffer, content.data()[read_offset..read_offset + read_len]);
    }

    /// Reading past the end of the file fails
    #[test]
    fn read_past_the_end_of_the_file_fails() {
        let read_offset: usize = 1024;
        let read_len: usize = 32;
        let content = RandomBlob::new(128);
        let mut file = MemoryFile::new_with_data(content.data());
        let mut buffer = vec![0u8; read_len];

        file.open().expect("open should not fail");

        let result = file.read(read_offset, &mut buffer);

        assert!(result.is_err());
        assert!(matches!(result, Err(FileError::EndOfFileRead { .. })));
    }

    /// Size returns the correct size
    #[test]
    fn size_returns_the_correct_size() {
        let content_size: usize = 128;
        let content = RandomBlob::new(content_size);
        let mut file = MemoryFile::new_with_data(content.data());
        file.open().expect("open should not fail");

        let result = file.size();

        assert!(matches!(result, Ok(value) if value == content_size));
    }

    /// Size returns an error if the file is not opened.
    #[test]
    fn size_when_file_not_opened_fails() {
        let file = MemoryFile::new();

        let result = file.size();

        assert!(matches!(result, Err(FileError::FileNotOpened(_))));
    }
}

use thiserror::Error;

/// Represents error that can occur during file operations.
#[derive(Error, Debug)]
pub enum FileError {
    /// Indicates that the file could not be created because a file with the same name already
    /// exists.
    ///
    /// # Fields
    /// - `0` - A string representing the path of the file that caused the error.
    #[error("The file ({0}) already exists.")]
    FileAlreadyExists(String),

    /// Indicates that an operation failed because the file was not opened.
    ///
    /// # Fields
    /// - `0` - A string representing the path of the file that caused the error.
    #[error("The file ({0}) is not opened.")]
    FileNotOpened(String),

    /// Indicates that an operation failed because the file was opened.
    ///
    /// # Fields
    /// - `0` - A string representing the path of the file that caused the error.
    #[error("The file ({0}) is opened.")]
    FileOpened(String),

    /// Indicates that an operation tried to read past the end of the file.
    #[error("Cannot read past the end of the file ({filename}). The file is {file_size} bytes. An attempt was made to read {read_size} bytes from position {offset}.")]
    EndOfFileRead {
        filename: String,
        file_size: usize,
        offset: usize,
        read_size: usize,
    },
}

/// Represents operations that can be performed on a file.
///
/// This traits define a set of method for interacting with a file. Implementators of this trait
/// should provide concrete implementations for differents operating systems.
///
/// # Errors
///
/// Method in this trait returns [FileError].
pub trait File {
    /// Creates and open a new file.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - the file is already opened
    /// - the file already exists
    /// - any other unexpected reasons why the file can't be created
    fn create(&mut self) -> Result<(), FileError>;

    /// Closes the file.
    ///
    /// # Errors
    ///
    /// This method will returned an error of the file is not opened
    fn close(&mut self) -> Result<(), FileError>;

    /// Opens the file.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - the file is already opened
    /// - the file does not exists
    /// - an unexpected error occurs while opening the file
    fn open(&mut self) -> Result<(), FileError>;

    /// Deletes the file.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - the file is opened
    /// - the file does not exists
    /// - an unexpected error occurs while deleting the file
    fn delete(&mut self) -> Result<(), FileError>;

    /// Write a block of data in the file at a specified offset.
    fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), FileError>;

    /// Read a block of data in the file at a specified offset into a buffer. The size of the data
    /// read is based on the size of the buffer.
    fn read(self, offset: usize, buffer: &mut [u8]) -> Result<(), FileError>;

    // Flush all changes to the disk so it will not be lost in case of a crash or power failure.
    fn sync(self) -> Result<(), FileError>;

    // Get the size of the file.
    //
    // # Errors
    //
    // This method will return an error if the file is not opened.
    fn size(self) -> Result<usize, FileError>;
}

use thiserror::Error;

/// Represents error that can occur during file operations.
#[derive(Error, Debug)]
pub enum FileError {
    /// Indicates that an attemp was made to open a file that is already opened.
    ///
    /// # Fields
    ///
    /// - `0` - A string representing the path of the file that caused the error.
    #[error("The file ({0}) is already opened.")]
    FileAlreadyOpened(String),

    /// Indicates that the file could not be created because a file with the same name already 
    /// exists.
    /// 
    /// # Fields
    /// 
    /// - `0` - A string representing the path of the file that caused the error.
    #[error("The file ({0}) already exists.")]
    FileAlreadyExists(String),

    /// Indicates that an operation failed because the file was not opened.
    /// 
    /// # Fields
    /// 
    #[error("The file ({0}) is not opened.")]
    FileNotOpened(String),
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
    /// - The file is already opened.
    /// - The file already exists.
    /// - Any other unexpected reasons why the file can't be created.
    fn create(&mut self) -> Result<(), FileError>;

    /// Closes the file.
    /// 
    /// # Errors
    /// 
    /// This method will returned an error of the file is not opened
    fn close(&mut self) -> Result<(), FileError>;
}

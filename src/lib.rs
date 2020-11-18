//! A super small crate which contains [AutoDeletePath](struct.AutoDeletePath.html),
//! a path which gets automatically deleted when it goes out of scope.
//!
//! # Examples
//! ```
//! {
//!     let tmp_path = auto_delete_path::AutoDeletePath::temp(); // creates a new path at the default temp folder
//!     std::fs::create_dir(&tmp_path); // AutoDeletePath implements AsRef<Path>
//!     let subfile = tmp_path.as_ref().join("subfile"); // create a subfile
//!     std::fs::File::create(&subfile).unwrap();
//! } // tmp_path dies here, so the directory and its contents will be deleted
//!```
//!
//! See [AutoDeletePath](struct.AutoDeletePath.html) and [include_to_auto_delete_path](macro.include_to_auto_delete_path.html)
//! for more examples.

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU16, Ordering},
};

/// Macro for including a source file, and writing it to a new `AutoDeletePath::temp`.
///
/// Useful for testing.
///
/// # Panics
///
/// Panics if writing to the tempfile fails.
///
/// # Example
///
/// ```
/// let tmp_path = auto_delete_path::include_to_auto_delete_path!("test-resources/test-include.txt");
/// assert_eq!(std::fs::read_to_string(&tmp_path).unwrap(), "Included file!\n");
/// ```
#[macro_export]
macro_rules! include_to_auto_delete_path {
    ($file:expr) => {{
        use std::io::Write;

        let tmp_path = $crate::AutoDeletePath::temp();
        let file_bytes = include_bytes!($file);
        let mut file = std::fs::File::create(&tmp_path).unwrap();
        file.write_all(file_bytes).unwrap();
        tmp_path
    }};
}

/// This struct simply holds an instance of `std::path::PathBuf`.
/// However, when such an instance goes out of scope and is destroyed, its destructor will be called,
/// which attempts to delete the owned path (either file or directory).
///
/// This works even if the program panics.
///
/// Useful for creating temporary files that you want to be deleted automatically.
pub struct AutoDeletePath {
    path: PathBuf,
}

impl AutoDeletePath {
    /// Creates an AutoDeletePath in the default temp directory.
    /// This method just returns a path; If you want to actually make a file or folder,
    /// you have to do that manually.
    ///
    /// # Examples
    ///
    /// File:
    /// ```
    /// let mut temp_path_clone = std::path::PathBuf::new();
    /// {
    ///     let temp_path = auto_delete_path::AutoDeletePath::temp();
    ///     temp_path_clone = temp_path.as_ref().to_owned();
    ///     assert!(!temp_path_clone.exists());
    ///     std::fs::write(&temp_path, "spam").unwrap();
    ///     assert!(temp_path_clone.exists());
    /// } // temp_path dies here, so the file is deleted
    /// assert!(!temp_path_clone.exists());
    /// ```
    ///
    /// Directory:
    /// ```
    /// let mut temp_path_clone = std::path::PathBuf::new();
    /// {
    ///     let temp_path = auto_delete_path::AutoDeletePath::temp();
    ///     temp_path_clone = temp_path.as_ref().to_owned();
    ///     assert!(!temp_path_clone.exists());
    ///     std::fs::create_dir(&temp_path).unwrap();
    ///     assert!(temp_path_clone.exists());
    /// } // temp_path dies here, so the directory is deleted
    /// assert!(!temp_path_clone.exists());
    /// ```
    pub fn temp() -> Self {
        Self {
            path: create_temp_path(),
        }
    }
}

impl std::convert::AsRef<Path> for AutoDeletePath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Drop for AutoDeletePath {
    fn drop(&mut self) {
        if self.path.is_dir() {
            std::fs::remove_dir_all(&self.path).ok();
        } else {
            std::fs::remove_file(&self.path).ok();
        }
    }
}

static PATH_COUNT: AtomicU16 = AtomicU16::new(1);

/// Creates a random path at the default temp directory (usually /tmp).
fn create_temp_path() -> PathBuf {
    create_temp_path_at_directory(std::env::temp_dir())
}

/// Creates a random path at the specified directory.
fn create_temp_path_at_directory<P: AsRef<Path>>(directory: P) -> PathBuf {
    PathBuf::from(format!(
        "{}/rustytemp-{}",
        directory.as_ref().display(),
        PATH_COUNT.fetch_add(1, Ordering::Relaxed)
    ))
}

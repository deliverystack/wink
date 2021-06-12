//! The wink.file module contains a function for reading a file into a string,
//! which is helpful for handling cases where the file does not exist or cannot be read.

//TODO: this logic belongs in a library, not wink

use std::io::Read;

/// Example usage:
/// ```
/// match file::read_file(&path) {
///   Ok(contents) => {
///     let deserialized: InvocableCategoryList = serde_json::from_str(&contents).unwrap();
///   }
/// }
/// ```

/// Read the specified file into a string.
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    std::fs::File::read_to_string(&mut file, &mut data)?;
    Ok(data)
}

use std::io::Read;

// read the specified file into a string
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    std::fs::File::read_to_string(&mut file, &mut data)?;
    Ok(data)
}

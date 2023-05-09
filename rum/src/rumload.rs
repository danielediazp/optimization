use std::convert::TryInto;

/// Reads the instruction set from a specify file or std-in.
/// Returns a Vec containing all the instructions.
///
/// <br>
///
/// # Argument:
/// * `input`: The file path as &str.
pub fn load(input: Option<&str>) -> Vec<u32> {
    // Read the raw data from the file
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    // Convert the file data into a useful 32 bit set of words for the UM
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}

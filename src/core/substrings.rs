use std::{fs::File, io::{self, Write}};

pub fn generate_substrings(input: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;

    // Iterate through the string to generate substrings
    for start in 0..input.len() {
        for end in start + 1..=input.len() {
            let substring = &input[start..end];
            file.write_all(substring.as_bytes())?;
            file.write_all(b"\n")?;
        }
    }

    Ok(())
}
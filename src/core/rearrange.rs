use std::{fs::File, io::{self, Write}};

pub fn rearrange(input: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;

    fn permute(chars: &mut Vec<char>, start: usize, end: usize, output_file: &str, file: &mut File) -> io::Result<()> {
        if start == end {
            let permutation: String = chars.iter().collect();
            writeln!(file, "{}", permutation)?;
        } else {
            for i in start..=end {
                chars.swap(start, i);
                permute(chars, start + 1, end, output_file, file);
                chars.swap(start, i); // Undo the swap for backtracking
            }
        }
        Ok(())
    }

    let mut chars: Vec<char> = input.chars().collect();
    let length = chars.len();
    permute(&mut chars, 0, length - 1, output_file, &mut file)?;

    Ok(())
}
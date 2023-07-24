use std::{fs::OpenOptions, io::Write};
use indicatif::ProgressBar;
use itertools::Itertools;

use crate::cli;



fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn combination(n: usize, r: usize) -> usize {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn num_outcomes(phrase_len: usize, alphabet_len: usize, replacements: usize) -> usize {
    combination(phrase_len, replacements) * alphabet_len.pow(replacements as u32)
}

fn approximate_file_size(phrase_len: usize, alphabet_len: usize, replacements: usize) -> String {
    let num_outcomes = num_outcomes(phrase_len, alphabet_len, replacements) as f64;
    let size_per_outcome = (phrase_len + 1) as f64; // +1 for the newline character
    let total_size_bytes = num_outcomes * size_per_outcome;

    if total_size_bytes < 1024.0 {
        format!("{:.2} B", total_size_bytes)
    } else if total_size_bytes < 1024.0 * 1024.0 {
        format!("{:.2} KB", total_size_bytes / 1024.0)
    } else if total_size_bytes < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} MB", total_size_bytes / (1024.0 * 1024.0))
    } else if total_size_bytes < 1024.0 * 1024.0 * 1024.0 * 1024.0{
        format!("{:.2} GB", total_size_bytes / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{:.2} TB", total_size_bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0))
    }
}

fn generate(
    mut phrase: Vec<char>, 
    alphabet: &[char], 
    pos: usize, 
    replacements: usize, 
    file: &mut std::fs::File,
    pb: &ProgressBar
) {
    if replacements == 0 {
        let result = phrase.iter().collect::<String>() + "\n";
        file.write_all(result.as_bytes()).expect("Error writing to file");
        pb.inc(1);
    } else if pos < phrase.len() {
        for &ch in alphabet {
            let old_ch = phrase[pos];
            phrase[pos] = ch;
            generate(
                phrase.clone(), 
                alphabet, 
                pos + 1, 
                replacements - 1, 
                file,
                pb
            );
            phrase[pos] = old_ch;
        }
        generate(
            phrase, 
            alphabet, 
            pos + 1, 
            replacements, 
            file,
            pb
        );
    }
}

pub fn start_dictionary_generation(
    mut phrase: &str, 
    alphabet: &str, 
    replacements: usize,
    dry_run: bool,
    output_file: &str

) {
    let num_outcomes = num_outcomes(phrase.len(), alphabet.len(), replacements);
    println!("Number of outcomes: {}", num_outcomes);

    let file_size_gb = approximate_file_size(phrase.len(), alphabet.len(), replacements);
    println!("Approximate file size: {}", file_size_gb);

    let pb = cli::progress_bar::get_progress_bar(num_outcomes);
    
    if !dry_run {
        let mut outputFile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file)
        .expect("Unable to open file");

        let mut counter = 0;
        generate(
            phrase.chars().collect(), 
            &alphabet.chars().collect::<Vec<_>>(), 
            0, 
            replacements,
            &mut outputFile,
            &pb
        );
    }

    pb.finish_with_message("All combinations generated");

}
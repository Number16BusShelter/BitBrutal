use std::{fs::OpenOptions, io::Write};
use indicatif::ProgressBar;
use itertools::Itertools;

use crate::cli;

// fn factorial(n: usize) -> usize {
//     (1..=n).product()
// }

// fn combination(n: usize, r: usize) -> usize {
//     factorial(n) / (factorial(r) * factorial(n - r))
// }

// fn num_outcomes(phrase_len: usize, alphabet_len: usize, replacements: usize) -> usize {
//     combination(phrase_len, replacements) * alphabet_len.pow(replacements as u32)
// }

// fn approximate_file_size(phrase_len: usize, alphabet_len: usize, replacements: usize) -> String {
//     let num_outcomes = num_outcomes(phrase_len, alphabet_len, replacements) as f64;
//     let size_per_outcome = (phrase_len + 1) as f64; // +1 for the newline character
//     let total_size_bytes = num_outcomes * size_per_outcome;

//     if total_size_bytes < 1024.0 {
//         format!("{:.2} B", total_size_bytes)
//     } else if total_size_bytes < 1024.0 * 1024.0 {
//         format!("{:.2} KB", total_size_bytes / 1024.0)
//     } else if total_size_bytes < 1024.0 * 1024.0 * 1024.0 {
//         format!("{:.2} MB", total_size_bytes / (1024.0 * 1024.0))
//     } else if total_size_bytes < 1024.0 * 1024.0 * 1024.0 * 1024.0{
//         format!("{:.2} GB", total_size_bytes / (1024.0 * 1024.0 * 1024.0))
//     } else {
//         format!("{:.2} TB", total_size_bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0))
//     }
// }

// fn generate(
//     mut phrase: Vec<char>, 
//     alphabet: &[char], 
//     pos: usize, 
//     replacements: usize, 
//     file: &mut std::fs::File,
//     pb: &ProgressBar
// ) {
//     if replacements == 0 {
//         let result = phrase.iter().collect::<String>() + "\n";
//         file.write_all(result.as_bytes()).expect("Error writing to file");
//         pb.inc(1);
//     } else if pos < phrase.len() {
//         for &ch in alphabet {
//             let old_ch = phrase[pos];
//             phrase[pos] = ch;
//             generate(
//                 phrase.clone(), 
//                 alphabet, 
//                 pos + 1, 
//                 replacements - 1, 
//                 file,
//                 pb
//             );
//             phrase[pos] = old_ch;
//         }
//         generate(
//             phrase, 
//             alphabet, 
//             pos + 1, 
//             replacements, 
//             file,
//             pb
//         );
//     }
// }

// pub fn start_dictionary_generation(
//     mut phrase: &str, 
//     alphabet: &str, 
//     replacements: usize,
//     dry_run: bool,
//     output_file: &str

// ) {
//     let num_outcomes = num_outcomes(phrase.len(), alphabet.len(), replacements);
//     println!("Number of outcomes: {}", num_outcomes);

//     let file_size_gb = approximate_file_size(phrase.len(), alphabet.len(), replacements);
//     println!("Approximate file size: {}", file_size_gb);

//     let pb = cli::progress_bar::get_progress_bar(num_outcomes);
    
//     if !dry_run {
//         let mut outputFile = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .open(output_file)
//         .expect("Unable to open file");

//         let mut counter = 0;
//         generate(
//             phrase.chars().collect(), 
//             &alphabet.chars().collect::<Vec<_>>(), 
//             0, 
//             replacements,
//             &mut outputFile,
//             &pb
//         );
//     }

//     pb.finish_with_message("All combinations generated");

// }


/// This module provides utility functions for generating and managing dictionaries.
pub(crate) mod dictionary_generator {
    use crate::cli::{self, messages::logger};
    use std::{fs::OpenOptions, io::Write};
    use indicatif::ProgressBar;
    use itertools::Itertools;

    /// Calculates the factorial of a given number.
    ///
    /// # Arguments
    ///
    /// * `n` - The number for which to calculate the factorial.
    ///
    /// # Returns
    ///
    /// The factorial of the given number.
    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    /// Calculates the combination of `n` choose `r`.
    ///
    /// # Arguments
    ///
    /// * `n` - The total number of items.
    /// * `r` - The number of items to choose from `n`.
    ///
    /// # Returns
    ///
    /// The combination result.
    fn combination(n: usize, r: usize) -> usize {
        factorial(n) / (factorial(r) * factorial(n - r))
    }

    /// Calculates the total number of possible outcomes given the phrase length, alphabet size, and replacements.
    ///
    /// # Arguments
    ///
    /// * `phrase_len` - The length of the phrase.
    /// * `alphabet_len` - The size of the alphabet.
    /// * `replacements` - The number of simultaneous replacements to make.
    ///
    /// # Returns
    ///
    /// The total number of possible outcomes.
    fn num_outcomes(phrase_len: usize, alphabet_len: usize, replacements: usize) -> usize {
        combination(phrase_len, replacements) * alphabet_len.pow(replacements as u32)
    }

    /// Approximates the file size in a human-readable format (e.g., B, KB, MB, GB, TB) based on the number of outcomes.
    ///
    /// # Arguments
    ///
    /// * `phrase_len` - The length of the phrase.
    /// * `alphabet_len` - The size of the alphabet.
    /// * `replacements` - The number of simultaneous replacements to make.
    ///
    /// # Returns
    ///
    /// A formatted string representing the approximate file size.
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
        } else if total_size_bytes < 1024.0 * 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} GB", total_size_bytes / (1024.0 * 1024.0 * 1024.0))
        } else {
            format!("{:.2} TB", total_size_bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0))
        }
    }

        /// Recursively generates dictionary phrases based on the input parameters and writes them to a file.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The current state of the phrase being generated.
    /// * `alphabet` - The alphabet to use for replacements.
    /// * `pos` - The current position in the phrase being considered for replacement.
    /// * `replacements` - The number of simultaneous replacements to make in the phrase.
    /// * `file` - A mutable reference to the output file where generated variations will be saved.
    /// * `pb` - A reference to the progress bar for tracking the generation progress.
    pub fn generate(
        mut phrase: Vec<char>,
        alphabet: &[char],
        pos: usize,
        replacements: usize,
        file: &mut std::fs::File,
        pb: &ProgressBar,
    ) {
        if replacements == 0 {
            // Convert the phrase to a string and write it to the output file
            let result = phrase.iter().collect::<String>() + "\n";
            file.write_all(result.as_bytes()).expect("Error writing to file");

            // Increment the progress bar
            pb.inc(1);
        } else if pos < phrase.len() {
            for &ch in alphabet {
                let old_ch = phrase[pos];
                phrase[pos] = ch;

                // Recursively generate variations for the next position
                generate(
                    phrase.clone(),
                    alphabet,
                    pos + 1,
                    replacements - 1,
                    file,
                    pb,
                );

                phrase[pos] = old_ch; // Restore the original character for the next iteration
            }

            // Continue generating variations for the next position
            generate(phrase, alphabet, pos + 1, replacements, file, pb);
        }
    }

    /// Generates dictionary phrases based on input parameters.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The input phrase to use as a base for generating variations.
    /// * `alphabet` - The alphabet to use for replacements.
    /// * `replacements` - The number of simultaneous replacements to make in the phrase.
    /// * `dry_run` - A boolean flag indicating whether to generate outcomes or just calculate metadata.
    /// * `output_file` - The name of the output file where generated variations will be saved.
    pub fn start_dictionary_generation(
        mut phrase: &str,
        alphabet: &str,
        replacements: usize,
        dry_run: bool,
        output_file: &str,
    ) {
        // Calculate the total number of outcomes
        let num_outcomes = num_outcomes(phrase.len(), alphabet.len(), replacements);
        logger::info(&format!("Number of outcomes: {}", num_outcomes));

        // Approximate and display the file size
        let file_size_gb = approximate_file_size(phrase.len(), alphabet.len(), replacements);
        logger::info(&format!("Approximate file size: {}", file_size_gb));

        // Initialize a progress bar
        let pb = cli::progress_bar::get_progress_bar(num_outcomes);

        if !dry_run {
            // Open the output file
            let mut output_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(output_file)
                .expect("Unable to open file");

            // Initialize a counter
            let mut counter = 0;

            // Generate dictionary phrases
            generate(
                phrase.chars().collect(),
                &alphabet.chars().collect::<Vec<_>>(),
                0,
                replacements,
                &mut output_file,
                &pb,
            );
        }

        // Finish and display the progress bar
        pb.finish_with_message("All combinations generated");
    }
}
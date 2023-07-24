#![feature(thread_id_value)]

use std::time::Instant;

use core::dict_generator::start_dictionary_generation;
use core::{controller::CheckFn};
// use core::hash::check;
use core::hash::ring::check as ring;
use core::hash::aes::check as aes;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use crate::core::components::HashComponents;
mod core;
mod cli;


// fn main() {
//     let method = "ring";

//     let alphabet: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

//     let hash: &str = "";
//     let hash_components = HashComponents::new(&hash);

//     let password: &str = "bit";
//     // let password: &str = "12aVP18cd5XsbcGQy8u6eywQ6UuA6Q319s";

//     let mut check_functions: HashMap<&str, CheckFn> = HashMap::new();
//     check_functions.insert("ring", ring);
//     check_functions.insert("aes", aes);

//     let check_fn = check_functions.get(method).unwrap();
//     let generator_file = "generator_state.txt";
//     let success_file = "success.txt";

    
//     // println!("Wallet {}\nPassword {}\nResult: {}", hash, password, aes(password, &hash_components));
//     core::controller::process_combinations(alphabet.to_string(), hash.to_string(), 4, *check_fn, generator_file, success_file)
// }

// 4y 8W a7 aG dV j5 q9 AL Lb Na Oo RC


const DEFAULT_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn main() {
    // let phrase = "ABCDE";
    // let alphabet = "FG";
    // let replacements = 1;

    // let alphabet: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // let phrase: &str = "";
    // let replacements: i32 = 3;

    // let dry_run = true;

    // start_dictionary_generation(phrase, alphabet, replacements.try_into().unwrap(), dry_run);
    // println!("Generation complete!");

    let start_time = Instant::now();

    cli::cli::banner();
    let matches = cli::cli::get_matches();
    // You can check the value provided by positional arguments, or option arguments

    let phrase = match matches.get_one::<String>("PHRASE") {
        Some(value) => value,
        None => "password"
    };

    println!("Target phrase is: {}", phrase);

    let alphabet = match matches.get_one::<String>("ALPHABET") {
        Some(value) => value,
        None => DEFAULT_ALPHABET
    };

    println!("Alphabet is: {}", alphabet);

    let dry_run = match  matches.get_one::<bool>("DRY_RUN") {
        Some(value) => value,
        None => &false
    };

    println!("Dry run mode: {}", dry_run);

    let replacements = match matches.get_one::<i32>("REPLACEMENTS") {
        Some(value) => value,
        None => &1
    };

    println!("Number of replacements: {}", replacements);

    let output_file= match matches.get_one::<String>("OUTPUT_FILE") {
        Some(value) => value,
        None => "./output.txt"
    };

    println!("Output file: {}", output_file);

    start_dictionary_generation(phrase, alphabet, *replacements as usize, *dry_run, output_file);
    let duration = start_time.elapsed();
    println!("\n\nGeneration complete in {:.2?}", duration);
}


// 9 char: 1861818336 + 20019552 + 138384 + 558 = 1881976830
// 10 char: 3103030560 + 28599360 + 172980 + 620 + 1881976830 * 10 = 
// 


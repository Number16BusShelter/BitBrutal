use clap::{Command, Arg, ArgMatches, ArgAction};
use colored::*;

pub fn banner() {
    let banner = include_str!("./banner");

    eprintln!("{}", banner.bold().red())
}

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Target phrase
//     #[arg(short, long, default_value = "password")]
//     phrase: String,

//     /// Alphabet
//     #[arg(short, long, default_value = "abc")]
//     alphabet: String,

//     /// Number of replacements
//     #[arg(short, long, default_value_t = 1)]
//     replacements: i32,

//     /// Dry run
//     #[arg(short, long, default_value_t = false)]
//     dry_run: bool,

//     /// Output file
//     #[arg(short, long, default_value = "./output.txt")]
//     output: String
// }



pub fn get_matches() -> ArgMatches {
    Command::new("BitBrutal")
        .version("1.0")
        .author("Number16BusShelter")
        .about("Password mutation generator utility written in Rust.")
        .arg(Arg::new("PHRASE")
            .short('p')
            .long("phrase")
            .help("Phrase to create mutations from")
            .value_name("PHRASE")
            .required(false)
            .default_value("Test")
            .action(ArgAction::Set)
        )
        .arg(
            Arg::new("ALPHABET")
            .short('a')
            .long("alphabet")
            .value_name("ALPHABET")
            .help("Sets the alphabet to use for replacements")
        )
        .arg(Arg::new("REPLACEMENTS")
            .short('r')
            .long("replacements")
            .value_name("REPLACEMENTS")
            .help("Sets the number of simultaneous replacements to make in the phrase")
            .value_parser(clap::value_parser!(i32))
        )
        .arg(Arg::new("DRY_RUN")
            .short('d')
            .long("dry_run")
            .help("Doesn't generate the outcomes, only calculates the number of outcomes and the approximate file size")
            .value_parser(clap::value_parser!(bool)),
        )
        .arg(Arg::new("OUTPUT_FILE")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Sets the output file (default is output.txt)")
        )
        .get_matches()
}
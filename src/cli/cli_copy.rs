use std::path::PathBuf;

use clap::{ArgAction, arg, command};
use clap::{Parser, Subcommand};
use colored::*;

use crate::core::dict_generator::dictionary_generator::start_dictionary_generation;

pub fn banner() {
    let banner = include_str!("./banner");

    eprintln!("{}", banner.bold().red())
}

const DEFAULT_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Parser)]
#[command(
    name="BitBrutal",
    author="Number16Busshelter", 
    version, 
    about="Password mutation generator utility written in Rust.", 
    long_about = None
)]
pub struct CLI {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,
    // },

    #[command(
        name="mutations",
        about="Generate all possible phrase mutations with given alphabet and number of replacements", 
        long_about = None
    )]
    Mutations {
        name: Option<String>,

        #[arg(
            short, long, 
            required=false,
            help="Phrase to create mutations from"
        )]
        phrase: Option<String>,

        #[arg(
            short, long, 
            required=false,
            help="Sets the alphabet to use for replacements",
            default_value=DEFAULT_ALPHABET
        )]
        alphabet: Option<String>,

        #[arg(
            short, long, 
            required=false,
            help="Sets the number of simultaneous replacements to make in the phrase",
            default_value="1",
            value_parser=clap::value_parser!(usize)
        )]
        replacements: Option<usize>,

        #[arg(
            short, long, 
            required=false,
            help="Doesn't generate the outcomes, only calculates the number of outcomes and the approximate file size", 
            value_name="DRY_RUN", 
            action=ArgAction::SetTrue,
            value_parser=clap::value_parser!(bool),
            default_value="false"
        )]
        dry_run: Option<bool>,

        #[arg(
            short, long, 
            required=false,
            help="Sets the output file (default is output.txt)",
            default_value="./output.txt",
            value_parser=clap::value_parser!(PathBuf)
        )]
        output: PathBuf,
    },

    #[command(
        name="substrings",
        about="Get all substrings from a given string", 
        long_about = None
    )]
    Substrings {
        name: Option<String>,

        #[arg(
            short, long, 
            required=true,
            help="Phrase to create mutations from"
        )]
        phrase: Option<String>,

        #[arg(
            short, long, 
            required=false,
            help="Sets the output file (default is output.txt)",
            default_value="./output.txt",
            value_parser=clap::value_parser!(PathBuf)
        )]
        output: PathBuf,

    },
    #[command(
        name="examine",
        about="Test given dict file with BTC wallet.dat sha (from JR)", 
        long_about = None
    )]
    Examine {
        name: Option<String>,

        #[arg(
            short, long, 
            required=false,
            help="Sets the file to run examination (default is dict.txt)",
            default_value="./dict.txt",
            value_parser=clap::value_parser!(PathBuf)
        )]
        file: PathBuf,

        #[arg(
            short, long, 
            required=true,
            help="Testing mode",
            default_value="default",
            value_parser=clap::builder::PossibleValuesParser::new(&["default", "ring", "aes"])
        )]
        mode: Option<String>,

        #[arg(
            short='x', long, 
            required=true,
            help="Hash string",
        )]
        hash: String,
    }
    
}


pub fn main() {
    banner();
    let cli = CLI::parse();
    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    match &cli.command {
        Some(Commands::Mutations {
            phrase,
            alphabet,
            replacements,
            dry_run,
            output, 
            name 
        }) => {
            println!("Running mutations...");
            let p;
            let a;
            let d;
            let o;
            let r;

            if let Some(phrase) = phrase.as_deref() {
                p = phrase;
            } else {
                p = "password"
            }
            println!("Target phrase is: {:?}", p);

            if let Some(alphabet) = alphabet.as_deref() {
                a = alphabet;
            } else {
                a = DEFAULT_ALPHABET;
            }
            println!("Alphabet is: {:?}", a);

            if let Some(dry_run) = dry_run {
                d = *dry_run;
            } else {
                d = false;
            }
            println!("Dry run mode: {:?}", d);
            
            if let Some(output) = output.as_os_str().to_str() {
                o = output
            } else {
                o = "./output.txt"
            }

            println!("Output file: {:?}", o);

            if let Some(replacements) = replacements {
                r = replacements;
            } else {
                r = &2
            }

            println!("Number of replacements: {}", r);

            start_dictionary_generation(
                p, 
                a, 
                *r, 
                d, 
                o
            );
        }
        Some(Commands::Substrings {
            phrase,
            output, 
            name 
        }) => {
            println!("Running substring...");
            let o;
            let p;

            if let Some(phrase) = phrase.as_deref() {
                p = phrase;
            } else {
                p = "password"
            }
            println!("Target phrase is: {:?}", p);

            if let Some(output) = output.as_os_str().to_str() {
                o = output
            } else {
                o = "./output.txt"
            }

            println!("Output file: {:?}", o);

            // Do substring

        }
    
        Some(Commands::Examine {
            file,
            mode,
            name, 
            hash 
        }) => {
            println!("Running examinations...");

            let f;
            let m;
            let h;
            
            if let Some(file) = file.as_os_str().to_str() {
                f = file
            } else {
                f = "./dict.txt"
            }
            println!("Test file: {:?}", f);
            
            if let Some(mode) = mode.as_deref() {
                m = mode
            } else {
                m = "default"
            }
            println!("Mode: {:?}", m);

            h = hash;

            println!("Hash: {:?}", h);
            // Do something

        }
        None => {
            panic!("Command unknown! Aborting")
        }
    }

}
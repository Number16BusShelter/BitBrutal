use std::path::PathBuf;

use clap::{ArgAction, arg, command};
use clap::{Parser, Subcommand};
use colored::*;

use crate::core::dict_generator::dictionary_generator::start_dictionary_generation;
use crate::core::rearrange::rearrange;
use crate::core::substrings::generate_substrings;

use super::messages::logger;

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
            default_value="./dicts/mutations-output.txt",
            value_parser=clap::value_parser!(PathBuf)
        )]
        output: PathBuf,
    },

    #[command(
        name="rearrange",
        about="Rearranges characters in a given string", 
        long_about = None
    )]
    Rearrange {
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
            default_value="./dicts/rearrange-output.txt",
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
            default_value="./dicts/substrings-output.txt",
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

    match &cli.command {
        Some(Commands::Mutations {
            phrase,
            alphabet,
            replacements,
            dry_run,
            output, 
            name 
        }) => {
            logger::info(&format!("Running mutations..."));
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
            logger::info(&format!("Target phrase is: {:?}", p));

            if let Some(alphabet) = alphabet.as_deref() {
                a = alphabet;
            } else {
                a = DEFAULT_ALPHABET;
            }
            logger::info(&format!("Alphabet is: {:?}", a));

            if let Some(dry_run) = dry_run {
                d = *dry_run;
            } else {
                d = false;
            }
            logger::info(&format!("Dry run mode: {:?}", d));
            
            if let Some(output) = output.as_os_str().to_str() {
                o = output
            } else {
                o = "./mutations-output.txt"
            }

            logger::info(&format!("Output file: {:?}", o));

            if let Some(replacements) = replacements {
                r = replacements;
            } else {
                r = &2
            }

            logger::info(&format!("Number of replacements: {}", r));

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
            logger::info(&format!("Running substring..."));
            let o;
            let p;

            if let Some(phrase) = phrase.as_deref() {
                p = phrase;
            } else {
                p = "password"
            }
            logger::info(&format!("Target phrase is: {:?}", p));

            if let Some(output) = output.as_os_str().to_str() {
                o = output
            } else {
                o = "./substring-output.txt"
            }

            logger::info(&format!("Output file: {:?}", o));
            
            generate_substrings(p, o);
            // Do substring
        }

        Some(Commands::Rearrange {
            phrase,
            output, 
            name 
        }) => {
            logger::info(&format!("Running rearrange..."));
            let o;
            let p;

            if let Some(phrase) = phrase.as_deref() {
                p = phrase;
            } else {
                p = "password"
            }
            logger::info(&format!("Target phrase is: {:?}", p));

            if let Some(output) = output.as_os_str().to_str() {
                o = output
            } else {
                o = "./rearrange-output.txt"
            }

            logger::info(&format!("Output file: {:?}", o));
            
            rearrange(p, o);
            // Do substring
        }
    
        Some(Commands::Examine {
            file,
            mode,
            name, 
            hash 
        }) => {
            logger::info(&format!("Running examinations..."));

            let f;
            let m;
            let h;
            
            if let Some(file) = file.as_os_str().to_str() {
                f = file
            } else {
                f = "./dict.txt"
            }
            logger::info(&format!("Test file: {:?}", f));
            
            if let Some(mode) = mode.as_deref() {
                m = mode
            } else {
                m = "default"
            }
            logger::info(&format!("Mode: {:?}", m));

            h = hash;

            logger::info(&format!("Hash: {:?}", h));
            // Do something

        }
        None => {
            panic!("Command unknown! Aborting")
        }
    }

}
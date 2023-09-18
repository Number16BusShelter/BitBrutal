use clap::{Command, Arg, ArgMatches, ArgAction, builder::PossibleValue};
use colored::*;

pub fn banner() {
    let banner = include_str!("./banner");

    eprintln!("{}", banner.bold().red())
}

fn cli_command_mutations() -> Command {
    return Command::new("mutations")
    .about("Generate all possible phrase mutations with given alphabet and number of replacements")
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
        .num_args(0)
        .help("Doesn't generate the outcomes, only calculates the number of outcomes and the approximate file size")
        .value_parser(clap::value_parser!(bool)),
    )
    .arg(Arg::new("OUTPUT_FILE")
        .short('o')
        .long("output")
        .value_name("FILE")
        .help("Sets the output file (default is output.txt)")
    )
}

fn cli_command_rearrange() -> Command {
    return Command::new("rearrange")
    .about("Rearranges characters in a given string")
    .arg(Arg::new("PHRASE")
        .short('p')
        .long("phrase")
        .help("Phrase to create mutations from")
        .value_name("PHRASE")
        .required(false)
        .default_value("Test")
        .action(ArgAction::Set)
    )
    .arg(Arg::new("OUTPUT_FILE")
        .short('o')
        .long("output")
        .value_name("FILE")
        .help("Sets the output file (default is output.txt)")
    )
}

fn cli_command_test_dict() -> Command {
    return Command::new("test_dict")
    .about("Test given dict file with BTC wallet.dat sha (from JR)")
}

fn cli_command_substring() -> Command {
    return Command::new("substring")
    .about("Get all substrings from a given string")
    .arg(Arg::new("PHRASE")
        .short('p')
        .long("phrase")
        .help("Phrase to create mutations from")
        .value_name("PHRASE")
        .required(false)
        .default_value("Test")
        .action(ArgAction::Set)
    )
    .arg(Arg::new("OUTPUT_FILE")
        .short('o')
        .long("output")
        .value_name("FILE")
        .help("Sets the output file (default is output.txt)")
    )
}


fn main_cli_app() -> Command  {
    return Command::new("BitBrutal")
        .version("1.0")
        .author("Number16BusShelter")
        .about("Password mutation generator utility written in Rust.")
        .subcommand(cli_command_test_dict())
        .subcommand(cli_command_mutations())
        .subcommand(cli_command_substring())
        .subcommand(cli_command_rearrange())

}

pub fn get_matches() -> ArgMatches {
    main_cli_app().get_matches()
}
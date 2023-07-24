
# BitBrutal - Rust Phrase Generator

This Rust program generates all possible changes of `r` simultaneous positions in a given phrase with characters from a given alphabet.

## Description

This tool is useful for generating all the possible variations of a phrase, given a certain number of characters to replace and a set of characters to use for replacement. It has uses in scenarios such as password cracking, brute-force algorithm testing, etc.

This project was designed mainly for usage with [hashcat](https://github.com/hashcat/hashcat) for slow hash modes like 11300 (wallet.dat).

Due to huge number of possibilities, when working with 8-10+ characters long passwords, we aim to reduce this number by replacing limited number of characters in a given phrase. This implies that you already have an idea about the password you are looking for, but you are not sure about the correctness of some characters, or you previously made a mistake (typo) when compiling it.

The program calculates the total number of possible outcomes and estimates the size of the output file before generating the phrases.

## Getting Started

### Dependencies

* This project is written in Rust and requires Rust version 1.73.0 or newer.

### Installing

* Clone the repository to your local machine.
* Navigate to the project directory.
* Run `cargo build --release` to build the project.

### Usage

This program provides a command line interface. You can specify parameters like the phrase, alphabet, number of replacements, and output file name.

For example:

```bash
cargo run -- \
	--phrase "ABCDE" \
	--alphabet "FGH" \
	--replacements 1 \
	--output_file "output.txt" \
	--dry_run false
```

For more details on how to use the command line interface, use the `--help` flag:

```bash
cargo run -- --help
```

### Usage with hashcat

After we've generated a file we can run it with mode `0` of hashcat

```bash
hashcat -a 0 -m 11300 ./hash.txt ./output.txt
```

### Contributing

We appreciate all contributions. If you plan to contribute new features, utility functions, or extensions to the core, please first open an issue to discuss what you would like to add.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

---

Remember, this is just a starting point for your README. You should add more specific information about your project, such as its purpose, how to use it, how to contribute, etc. Also, don't forget to include your actual `LICENSE` file if you're planning to make your project open source.

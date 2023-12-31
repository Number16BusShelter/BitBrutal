# BitBrutal - Rust Phrase Generator

This Rust program generates all possible changes of `r` simultaneous positions in a given phrase with characters from a given alphabet.


```bash
@@@@@@@   @@@  @@@@@@@  @@@@@@@   @@@@@@@   @@@  @@@  @@@@@@@   @@@@@@   @@@       
@@@@@@@@  @@@  @@@@@@@  @@@@@@@@  @@@@@@@@  @@@  @@@  @@@@@@@  @@@@@@@@  @@@       
@@!  @@@  @@!    @@!    @@!  @@@  @@!  @@@  @@!  @@@    @@!    @@!  @@@  @@!       
!@   @!@  !@!    !@!    !@   @!@  !@!  @!@  !@!  @!@    !@!    !@!  @!@  !@!       
@!@!@!@   !!@    @!!    @!@!@!@   @!@!!@!   @!@  !@!    @!!    @!@!@!@!  @!!       
!!!@!!!!  !!!    !!!    !!!@!!!!  !!@!@!    !@!  !!!    !!!    !!!@!!!!  !!!       
!!:  !!!  !!:    !!:    !!:  !!!  !!: :!!   !!:  !!!    !!:    !!:  !!!  !!:       
:!:  !:!  :!:    :!:    :!:  !:!  :!:  !:!  :!:  !:!    :!:    :!:  !:!   :!:      
 :: ::::   ::     ::     :: ::::  ::   :::  ::::: ::     ::    ::   :::   :: ::::  
:: : ::   :       :     :: : ::    :   : :   : :  :      :      :   : :  : :: : :  
                                                                                   
Password mutation generator utility written in Rust.

Usage: BitBrutal [OPTIONS] [COMMAND]

Commands:
  mutations   Generate all possible phrase mutations with given alphabet and number of replacements
  rearrange   Rearranges characters in a given string
  substrings  Get all substrings from a given string
  examine     Test given dict file with BTC wallet.dat sha (from JR)
  help        Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...  Turn debugging information on
  -h, --help      Print help
  -V, --version   Print version
```


## Description

This tool is useful for generating all the possible variations of a phrase, given a certain number of characters to replace and a set of characters to use for replacement. It has uses in scenarios such as password cracking, brute-force algorithm testing, etc.

This project was designed mainly for usage with [hashcat](https://github.com/hashcat/hashcat) for slow hash modes like 11300 (wallet.dat).

Due to huge number of possibilities, when working with 8-10+ characters long passwords, we aim to reduce this number by replacing limited number of characters in a given phrase. This implies that you already have an idea about the password you are looking for, but you are not sure about the correctness of some characters, or you previously made a mistake (typo) when compiling it.

The program calculates the total number of possible outcomes and estimates the size of the output file before generating the phrases.

Also this repository contains two functions to check the match of hash with the one, generated by JohnTheReaper, but it seems to produce false-positive results. If you know what's wrong in my algorithm, please, let me know.

## Disclaimer

This project, the Rust Phrase Generator, is intended for educational and legitimate purposes only. It is not designed to assist in illegal activities. Any use of this software for purposes that may violate any laws is strictly prohibited.

The developers of the Rust Phrase Generator are not responsible for any misuse of the software. It is the end user's responsibility to obey all applicable local, state, federal, and international laws. We assume no liability and are not responsible for any misuse or damage caused by this program.

Use of this software is at your own risk. The software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

By using the Rust Phrase Generator, you are agreeing to this disclaimer.

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

### Release build

```bash
cargo build --release
```

### Usage with hashcat

Use tmux to create session

```bash
tmux new-session
```

After we've generated a file we can run it with mode `0` of hashcat

```bash
hashcat -a 0 -m 11300 --session bb-1 ./hash.txt ./bitbrutal-wl-1.txt --status --status-timer 30 -o output.pot
```

Now we can safely detach from session by pressing `ctrl+b, d` and connect back to it using

```bash
tmux attach-session
```

While our test wordlist is running we can generate next files for testing

When our list is tested we can get the result by running

```bash
hashcat -a 0 -m 11300 --session bb-1 ./hash.txt ./bitbrutal-wl-1.txt --status --status-timer 30 -o output.pot
 --show
```

### Helpful commands

Combine two files into one

```bash
cat ./output-1.txt ./output-2.txt > ./output-master.txt
```

Remove duplicates

```bash
sort ./output-master.txt | uniq > ./output-clean.txt
```

### Contributing

We appreciate all contributions. If you plan to contribute new features, utility functions, or extensions to the core, please first open an issue to discuss what you would like to add.

# ================================ STARTUP COMMANDS ================================= #

sudo apt install curl build-essential cargo git hashcat nano screen -y
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | bash -s -- -y
source $HOME/.cargo/env

sudo apt-get update -y
sudo apt upgrade -y

git clone https://github.com/Number16BusShelter/BitBrutal.git

rustup default nightly
rustup toolchain install nightly
rustup run nightly cargo bench 

cd ./BitBrutal
mkdir ./dicts
cargo build --release

echo "Now run something like \n\n./target/release/BitBrutal -d "false" -r 1 -p "password" -o "./dicts/output.txt"\n"

# ================================= MANUAL COMMANDS ================================= #
# Send exisiting dict to server
# scp -P 40766 ./dicts/bf-9+10-master-1+2.txt root@178.54.21.235:/root/BitBrutal

# Test hashcat
# hashcat -w 3 -a 0 -m 11300

# Split file in n parts
# split -n 3

# === ATTACH === #
screen -S hashcat

# Generate dictionary
# ./target/release/BitBrutal -d "false" -r 4 -p "7Gr6ul4tW5" -o "./dicts/bf-10-4-1.txt"

# Make unique wordlist
# sort ./dicts/bf-10-3-1.txt | uniq > ./dicts/bf-10-3-1-clean.txt
# sort ./dicts/bf-9-3-1.txt | uniq > ./dicts/bf-9-3-1-clean.txt

# Run hashcat
hashcat -w 3 -a 0 -m 11300 ./hash.txt ./dicts/bf-10-4-1.txt -o output.pot
hashcat -w 2 -a 0 -m 11300 ./hash.txt ./dicts/bf-10-3-1-clean.txt -o output-3.pot


# === DETACH === #
# CTRL+A, D

# === ATTACH === #
# screen -x hashcat


cat ./dicts/bf-9-3-3-clean.txt ./dicts/bf-9-3-4-clean.txt ./dicts/bf-9-3-5-clean.txt ./dicts/bf-9-3-6-clean.txt ./dicts/bf-9-3-7-clean.txt ./dicts/bf-9-3-8-clean.txt ./dicts/bf-9-3-9-clean.txt ./dicts/bf-9-3-10-clean.txt > ./dicts/bf-9-3-10-master.txt


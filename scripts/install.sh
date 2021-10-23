#!/bin/bash
echo "-! [0]building..."
cargo build

echo "-! [1]copying..."
# make directory
mkdir -p ~/.nylang
chmod +x ~/.nylang

# move the binary
cp target/debug/nylang ~/.nylang
rm target/debug/nylang
chmod +x ~/.nylang/nylang

echo "-! [2]PATHing..."
# bash PATH
echo "export PATH="~/.nylang"":'"$PATH"' >>~/.profile
source ~/.profile
# fish PATH
mkdir -p ~/.config/fish/conf.d
touch ~/.config/fish/conf.d/nylang.fish
echo "set PATH ~/.nylang" : '"$PATH"' >> ~/.config/fish/conf.d/nylang.fish
source ~/.config/fish/conf.d/nylang.fish

echo
GREEN='\033[0;32m'
GREY='\033[0;36m'
NC='\033[0m' # No Color
echo -e "${GREEN}-! [3]DONE${NC}"
echo
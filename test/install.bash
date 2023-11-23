cargo build --release
rm -rf $HOME/.local/procmon
mkdir -p $HOME/.local/procmon
cp ./target/release/procmon ~/.local/procmon

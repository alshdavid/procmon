# Process Monitor

A command line utility that spawns a process and produces a `csv` report on its resource usage.

```bash
procmon node -e "setTimeout(() => console.log('Sup'), 4000)"
```

This will produce a file named something similar to `20231123-162035.csv` which will contain

```
time,memory,cpu,disk_read,disk_write
1,39321600,0.0,0,0
2,39321600,0.0,0,0
3,39321600,0.0,0,0
4,39321600,0.0,0,0
```

# Installation

## MacOS

```bash
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
echo "\nexport \PATH=\$PATH:\$HOME/.local/procmon\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

## Linux

```bash
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
echo "\nexport \PATH=\$PATH:\$HOME/.local/procmon\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

## Windows

I'm not good at PowerShell - follow the same steps as the Linux/MacOS scripts

## Credit

[Matt Jones](https://github.com/mattcompiles) 

# Process Monitor

A command line utility that spawns a process and produces a `csv` report on its resource usage.

```bash
procmon --help
```

```bash
procmon -- node -e "setTimeout(() => console.log('Sup'), 4000)"
```

This will produce a file named something similar to `report.csv`

```bash
procmon --mem-units kb --time-units ms --report something.csv -- node -e "setTimeout(() => console.log('Sup'), 4000)"
```

This will produce a file named something similar to `something.csv`


The csv file looks like:

```
time_s,cpu,memory_mb,disk_read,disk_write
0.000,0,0,0,0
0.065,0,16,5976064,0
0.567,0,37,16556032,0
1.068,0,37,0,0
1.569,0,37,0,0
2.069,0,37,0,0
2.570,0,37,0,0
3.070,0,37,0,0
3.571,0,37,0,0
4.073,0,37,0,0
4.145,0,0,0,0
```

# Installation

## MacOS

```bash
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
echo "\nexport \PATH=\$PATH:\$HOME/.local/procmon\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

#### Updating

```bash
rm -rf $HOME/.local/procmon
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/macos-arm64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
```

## Linux

```bash
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
echo "\nexport \PATH=\$PATH:\$HOME/.local/procmon\n" >> $HOME/.zshrc
source $HOME/.zshrc
```

#### Updating

```bash
rm -rf $HOME/.local/procmon
mkdir -p $HOME/.local/procmon
curl -L --url https://github.com/alshdavid/procmon/releases/latest/download/linux-amd64.tar.gz | tar -xvzf - -C $HOME/.local/procmon
```

## Windows

I'm not good at PowerShell - follow the same steps as the Linux/MacOS scripts

## Credit

[Matt Jones](https://github.com/mattcompiles) 
[David Alsh](https://github.com/alshdavid) 

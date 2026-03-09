# Process Monitor

A command line utility that spawns a process and produces a `csv` report on its resource usage.

```bash
Usage: procmon [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Command to run

Options:
  -r, --report <REPORT_PATH>      Output path for the generated report [env: PM_REPORT=] [default: procmon.csv]
  -i, --interval <POLL_INTERVAL>  How often to probe the process for details in milliseconds [env: PM_INTERVAL=] [default: 200]
  -m, --mem-units <MEM_UNITS>     What units to use for recording memory [env: PM_MEM_UNITS=] [default: mb] [possible values: mb, kb, b]
  -t, --time-units <TIME_UNITS>   What units to use for recording time [env: PM_TIME_UNITS=] [default: ms] [possible values: s, ms]
  -o, --override-report           Override report file if exists
      --no-cpu                    Don't measure CPU usage [env: PM_NO_CPU=]
      --no-memory                 Don't measure memory usage [env: PM_NO_MEMORY=]
      --no-disk                   Don't measure disk usage [env: PM_NO_DISK=]
  -h, --help                      Print help
```

# Usage

```bash
procmon -- node -e "setTimeout(() => console.log('Sup'), 4000)"
```

This will produce a file named `procmon.csv`

```bash
procmon --report something.csv -- node -e "setTimeout(() => console.log('Sup'), 4000)"
```

This will produce a report called `something.csv`

The csv file looks like:
```javascript
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

## Credit

[Matt Jones](https://github.com/mattcompiles) 
[David Alsh](https://github.com/alshdavid) 

# Dotfiles Installer

A Rust-powered tool to set up your Arch Linux system with dotfiles, drivers, and packages. Because doing it manually is so 2020.

## Features

- Clones your dotfiles repository (hopefully the right one)
- Installs GPU drivers (AMD, Intel, NVIDIA - we don't discriminate)
- Sets up packages from groups (base, dev, de, etc.)
- Detects if you're on a laptop and acts accordingly
- Has a 60% chance of working on the first try

## Installation

```bash
git clone https://github.com/sakuyma/dotfiles-installer
cd dotfiles-installer
cargo build --release
sudo ./target/release/dotfiles-installer
```

## Usage
```bash
dotfiles-installer

# TODO: Add actual CLI arguments
# Coming soon
```

## How it works

1. Checks if you have git and stow (you probably do)
2. Clones your dotfiles repo (if it exists in the multiverse)
3. Runs stow to symlink everything (the magic part)
4. Detects your GPU and installs drivers (Russian roulette edition)
5. Installs package groups (base, de, dev, etc.)
6. If you're on a laptop, enables laptop mode (battery savings)

## Project structure
.
├── configs/          # Dotfile management
│   ├── clone.rs      # Git clone (please work)
│   ├── install.rs    # Stow magic
│   └── laptop.rs     # Power saving tricks
├── hardware/         # GPU driver mayhem
│   ├── amd.rs        # Red team
│   ├── intel.rs      # Blue team  
│   ├── nvidia.rs     # Green team (good luck)
│   └── videocard.rs  # GPU detector 9000
├── packages/         # Package management
│   ├── install.rs    # Pacman + paru = love
│   └── list.rs       # Package groups (endless list)
└── main.rs           # Where the magic (or tragedy) begins

## Requirements
- Arch Linux (btw)
- git and stow (install them if you havent)
- paru for AUR packages
- Root priviges (because touching system files is fun)

## TODO
- [ ] Error handling - Actually handle errors instead of pretending they don't exist
- [ ] User interaction - Ask questions instead of assuming things

- [ ] CLI Arguments - So you don't have to edit the source code to change behavior

- [ ] Config profiles - Different setups for different moods

- [ ] Config file - So you don't have to edit the source code

- [ ] Rollback - For when things inevitably go wrong

- [ ] Package install - Make it actually install all the things

- [ ] Logging - So you know what exploded and when

- [ ] Network retry - For when your wifi gives up

- [ ] Parallel - Make things faster by doing multiple things at once (maybe)

## License
MIT - Do whatever you want, just don't blame us if your computer catches fire.

## Acknowledgments
- Coffee (for keeping me alive)
- The Rust compiler (for yelling at me until the code works)
- Stack Overflow (for all the copy-pasted code)
- You (for actually reading this far)


## Disclaimer: 
This tool may or may not work. If it breaks your system, you get to keep both pieces. Use at your own risk, preferably with a backup (you have backups, right?).

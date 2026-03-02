# Dotfiles Installer

[![AUR version](https://img.shields.io/aur/version/dotfiles-installer)](https://aur.archlinux.org/packages/dotfiles-installer)
[![AUR votes](https://img.shields.io/aur/votes/dotfiles-installer)](https://aur.archlinux.org/packages/dotfiles-installer)
[![AUR license](https://img.shields.io/aur/license/dotfiles-installer)](https://aur.archlinux.org/packages/dotfiles-installer)

A Rust-powered tool to set up your Arch Linux system with dotfiles, drivers, and packages. Because doing it manually is so 2020.

## Features

- Clones your dotfiles repository (hopefully the right one)
- Installs GPU drivers (AMD, Intel, NVIDIA - we don't discriminate)
- Sets up packages from groups (base, dev, de, etc.)
- Detects if you're on a laptop and acts accordingly
- Has a small chance of working on the first try

## Requirements

- Arch Linux (btw)
- git and stow (install them if you havent)
- paru for AUR packages
- Root priviliges (because touching system files is fun)

## Installation

### Arch Linux (AUR)
The package is available in the [AUR](https://aur.archlinux.org/packages/dotfiles-installer):


```bash
# Using yay
yay -S dotfiles-installer

# Using paru
paru -S dotfiles-installer

# From source
git clone https://github.com/sakuyma/dotfiles-installer
cd dotfiles-installer
cargo build --release
sudo ./target/release/dotfiles-installer
```

## Usage

```bash
dotfiles-installer [OPTIONS] [COMMAND]

Commands:
  list      List available package groups (so you know what you're getting into)
  install   Install specific packages (when groups are too mainstream)
  remove    Remove packages (for when you regret everything)
  init      Create example config (because reading docs is hard)
  help      Print help (you are here)

Options:
  -c, --config <FILE>     Config file path (if you're fancy)
  -v, --verbose           Tell me EVERYTHING (you'll regret this)
  -g, --group <GROUP>     Package groups to install (pick your poison)
  -y, --assume-yes        Answer yes to everything (living dangerously)
  -i, --interactive       Ask before doing anything (for the cautious ones)
  -n, --dry-run           Show what would happen (without actually breaking things)
  -f, --force             Force reinstall (because sometimes things need a good slap)
      --log               Write logs to ~/.cache/dotfiles-installer (for future therapists)
  -h, --help              Print help (you're already here)
  -V, --version           Print version (brag about it)


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
├── config/               # Config parsing (where the magic starts)
│   ├── mod.rs             
│   ├── parser.rs          
│   ├── settings.rs        
│   └── structs.rs         
├── dotfiles_manager/     # Dotfile handling (symlink city)
│   ├── clone.rs           
│   ├── install.rs         
│   ├── laptop.rs          
│   └── mod.rs             
├── hardware/             # GPU drivers (prayer circle included)
│   ├── amd.rs             
│   ├── intel.rs           
│   ├── mod.rs             
│   ├── nvidia.rs          
│   ├── utils.rs           
│   └── videocard.rs       
├── packages/             # Package management (pacman's therapist)
│   ├── install.rs         
│   ├── list.rs            
│   └── mod.rs             
├── utils/                # Utility functions (the duct tape)
│   └── network_retry.rs  # Because wifi hates you
├── logging.rs            # Tracks your mistakes
├── cli/                  # Command line interface (button pusher)
│   ├── args.rs           
│   ├── commands/i        
│   ├── formatter.rs      
│   ├── handlers.rs       
│   ├── interactive.rs    
│   ├── mod.rs            
│   ├── prompt_manager.rs 
│   └── validator.rs      
└── main.rs                # Where the chaos begins


## Roadmap (The "I'll get to it eventually" list)
### Coming up next

- [x] Progress bars - Watch the magic happen in real-time

- [ ] Templates - Pre-made configs for common setups (desktop, server, potato)

- [ ] User management - Actually create users instead of just talking about it

- [ ] System config - Set hostname, locale, timezone, etc without editing 15 files

### Maybe someday

- [ ] Config profiles - Different setups for different moods

- [ ] System config - Set hostname, locale, timezone, etc without editing 15 files

- [ ] Auto-commit - Because remembering to commit is so 2020 (automatic git commits when you change files)

- [ ] Hooks - Run custom scripts because your setup is "special"

- [ ] Rollback - For when things inevitably go wrong

- [ ] Parallel - Make things faster by doing multiple things at once (maybe)


## Contributing

Found a bug? Want a feature? Open an issue. Better yet, open a PR and pretend you know what you're doing. We won't judge (much).

## License

MIT - Do whatever you want, just don't blame us if your computer catches fire.

## Acknowledgments

- Coffee (for keeping me alive)
- The Rust compiler (for yelling at me until the code works)
- Stack Overflow (for all the copy-pasted code)
- You (for actually reading this far)

## Disclaimer:

This tool may or may not work. If it breaks your system, you get to keep both pieces. Use at your own risk, preferably with a backup (you have backups, right?).


## Changelog

### v0.1.5
- Package in AUR now

## Technical Debt

This codebase is currently supported by:
- Hope
- Duct tape
- Stack Overflow copy-paste
- The Rust compiler's patience

**TODO:** Remove crutches before someone breaks a leg


## If you like it, drop a star!

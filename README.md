# `cfg`

A CLI tool for quickly opening config files for various programs.

## Installation

On Linux:

```bash
cargo install cfg-cli
```

## Usage

For example, running

```bash
cfg bash
```

will open `~/.bashrc` with the editor set in `~/.config/cfg/cfg.toml` (set to `$EDITOR` by default). You can also explicitly pass an editor command:

```bash
cfg neovim --with vim
```

will open `~/.config/nvim/init.lua`.

Many programs also have aliases, which are shortened names that can be used to configure them, i.e., to configure Neovim, you can also do:

```bash
cfg nvim
```

For a full list of programs and their config files and aliases, see [the default configuration](https://github.com/vi013t/cfg/tree/main/src/default_config.toml).

All config file paths and aliases can be changed, and new programs can be added via the config in `~/.config/cfg/cfg.toml`, which will be automatically created upon the first time running `cfg`.

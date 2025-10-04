# `cfg`

A CLI tool for quickly opening config files for various programs.

## Installation

`cfg` is available through cargo:

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

## Configuration

### Options

The `cfg` configuration file is located at `~/.config/cfg/cfg.toml`. The default config is as follows:

```toml
[options]
editor = "$EDITOR"

[programs]
alacritty = { config = ["$XDG_CONFIG_HOME/alacritty/alacritty.toml", "$XDG_CONFIG_HOME/alacritty.toml", "~/.config/alacritty/alacritty.toml", "~/.alacritty.toml"], aliases = ["alac"] }
awesome = { config = ["~/.config/awesome/rc.lua"] }
bash = { config = ["~/.bashrc"] }
bottom = { config = ["~/.config/bottom/bottom.toml"] }
browsh = { config = ["~/.config/browsh/config.toml"] }
cfg = { config = ["~/.config/cfg/cfg.toml"] }
codium = { config = ["~/.config/VSCodium/User/settings.json"] }
fish = { config = ["~/.config/fish/config.fish"] }
flameshot = { config = ["~/.config/flameshot/flameshot.ini"] }
hyprland = { config = ["~/.confg/hypr/hyprland.conf"], aliases = ["hypr"] }
i3 = { config = ["~/.i3/config", "~/.config/i3/config"] }
joshuto = { config = ["~/.config/joshuto/joshuto.toml"] }
kitty = { config = ["~/.config/kitty/kitty.conf"] }
nano = { config = ["~/.nanorc"] }
neofetch = { config = ["~/.config/neofetch/config.conf"] }
neovim = { config = ["~/.config/nvim/init.lua", "~/AppData/Local/nvim/init.lua"], aliases = ["nvim"] }
nixos = { config = ["~/.config/nixos/configuration.nix"], aliases = ["nix"] }
ohmyposh = { config = ["~/.config/ohmyposh/ohmyposh.toml"], aliases = ["posh"] }
onedrive = { config = ["~/.config/onedrive/config"] }
picom = { config = ["~/.config/picom.conf", "~/.config/picom/picom.conf"] }
pls = { config = ["~/.config/pls/pls.toml" ] }
ranger = { config = ["~/.config/ranger/rc.conf", "~/.config/ranger/rifle.conf", "~/.config/ranger/scope.sh", "~/.config/ranger/commands.py"] }
rustfmt = { config = ["~/.config/rustfmt/rustfmt.toml"] }
stylua = { config = ["~/.config/stylua/stylua.toml"] }
vim = { config = ["~/.vimrc"] }
wezterm = { config = ["$WEZTERM_CONFIG_FILE", "$XDG_CONFIG_HOME/wezterm/wezterm.lua", "~/.config/wezterm/wezterm.lua", "~/.wezterm.lua"], aliases = ["wez"] }
zed = { config = ["~/.config/zed/settings.json"] }
zsh = { config = ["~/.zshrc"] }
```

### Autocomplete

`cfg` supports autocomplete. Add the following to your `~/.bashrc` or somewhere else that's sourced:

```bash
eval "$(cfg --init-autocomplete)"
```

Note that autocomplete will filter out programs that it detects you don't have installed, even if they're in your config.

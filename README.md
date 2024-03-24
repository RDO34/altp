# altp

A simple theme-picker for [alacritty](https://alacritty.org) A cross-platform, GPU-accelerated terminal emulator.

`[al]acritty [t]heme [p]icker`

Full credits to [@rajasegar](https://github.com/rajasegar) and [@juanvqz](https://github.com/juanvqz) for the awesome [alacritty-themes](https://github.com/rajasegar/alacritty-themes) NPM package that inspired this.

Credit for the included themes goes to the original authors. See the individual themes for the author details.

> Requires Alacritty >= v0.13.0

## Motivation

I'd wanted to try out some other themes for Alacritty so thought I'd give [alacritty-themes](https://github.com/rajasegar/alacritty-themes) a go, but found it didn't support the newer TOML config syntax for Alacritty.

Naturally, like any sane human being and instead of just updating my Alacritty config manually, I decided to re-write the entire thing in Rust with support for TOML config files.

## Installation

If not already installed, first install [rust](https://www.rust-lang.org/tools/install).

Then install altp globally:

```sh
$ cargo install altp
```

## Usage

Using the command with no arguments or options will show an interactive menu to select a theme:

```sh
$ altp

Select a theme: [Page 1/7]
> 3024 (dark)
  3024 (light)
  Afterglow
  Alabaster
  Alabaster (dark)
  Argonaut
  Ashes (dark)
  Ashes (light)
  Atelierdune (dark)
  Atelierdune (light)
  Atelierforest (dark)
  Atelierforest (light)
  Atelierheath (dark)
  Atelierheath (light)
```

More advanced options can be used to customise behaviour.

Use the `h` or `help` options to view the documentation.

```sh
$ altp -h

Usage: altp [OPTIONS] [THEME]

Arguments:
  [THEME]  The name of the theme to use

Options:
  -l, --list       Print a list of available themes
  -C, --current    Print the current theme name
  -d, --dir <DIR>  The directory to search for the alacritty config file
  -c, --create     Whether to create a new config file if one does not already exist
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version
```

Note that, by default `altp` will assume the alacritty config dir path to be:

```
C:/Users/user/AppData/Roaming/alacritty
```

for Windows and:

```
/home/user/.config/alacritty
```

for all other operating systems.

Use the `d` or `dir` argument to provide a custom alacritty config path.

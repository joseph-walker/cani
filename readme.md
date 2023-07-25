# CanIUse

[![CI](https://github.com/joseph-walker/cani/actions/workflows/ci.yml/badge.svg)](https://github.com/joseph-walker/cani/actions/workflows/ci.yml)

A command line tool for accessing CanIUse data offline and in your terminal

## Installation

_Instructions coming soon_

## Build + Dev

```bash
git clone https://github.com/joseph-walker/cani
cd cani

cargo build
# or
cargo run sync
````

## Usage

1) Ensure the tool is installed
2) Run `cani sync` to download a mirror of the CanIUse data
3) Run `cani ls` to see all the features available
4) Run `cani use <feature_key>` to see CanIUse data

By default, the mirrored CanIUse data will be stored in your home directory, at `~/.cani/caniuse.json`.

## Advanced Usage

`cani` uses the Unix philosophy and is designed to compose with other tools.

For example, integration with `fzf` might look like:

```bash
cani ls | fzf -d : --preview "cani use {1}" --preview-window right:75%:wrap
```

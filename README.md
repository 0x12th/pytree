# pytree

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/pytree.svg)](https://crates.io/crates/pytree)
[![CI](https://github.com/0x12th/pytree/actions/workflows/ci.yml/badge.svg)](https://github.com/0x12th/pytree/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/crates/d/pytree.svg)](https://crates.io/crates/pytree)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Clean `tree`-like output for Python projects.

## Install

Cargo:

```sh
cargo install --locked pytree
```

Homebrew:

```sh
brew tap 0x12th/pytree
brew install pytree
```

Binary:

```sh
curl -LO https://github.com/0x12th/pytree/releases/download/v0.1.0/pytree-x86_64-unknown-linux-gnu.tar.gz
tar -xzf pytree-x86_64-unknown-linux-gnu.tar.gz
./pytree --version
```

## Usage

```sh
pytree [PATH] [OPTIONS]
```

```sh
pytree
pytree --depth 2
pytree --dirs-only
pytree --ignore "*.sqlite3"
pytree --format json
```

Download platform-specific archives from [GitHub Releases](https://github.com/0x12th/pytree/releases).

## Options

| Option | Description |
| --- | --- |
| `--depth N` | Maximum tree depth |
| `--all` | Show hidden and ignored files |
| `--dirs-only` | Show directories only |
| `--no-gitignore` | Do not read `.gitignore` rules |
| `--ascii` | Use ASCII tree connectors |
| `--ignore PATTERN` | Add an ignore pattern |
| `--format tree\|json` | Output format |

## Development

```sh
cargo fmt
cargo fmt-check
cargo typecheck
cargo lint
cargo qa
cargo package-check
```

## Release

See [docs/release.md](docs/release.md).

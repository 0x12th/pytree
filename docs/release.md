# Release

## One-time setup

1. Create a crates.io token:

```sh
cargo login
```

For GitHub Actions, add the token as repository secret `CARGO_REGISTRY_TOKEN`.

2. Create a Homebrew tap repository:

```sh
gh repo create 0x12th/homebrew-pytree --public
```

Copy `homebrew-pytree/Formula/pytree.rb` from this repository into the tap.

## Local preflight

```sh
cargo fmt-check
cargo typecheck
cargo lint
cargo qa
cargo package-check
```

To inspect the publish payload:

```sh
cargo package --locked --list
```

## crates.io

Manual publish:

```sh
cargo login
cargo package --locked
cargo publish --locked
```

The release workflow also runs `cargo publish --locked` when a version tag is pushed.

## GitHub Release

Release `0.1.0`:

```sh
git switch master
git pull --ff-only
cargo package --locked
git tag -a v0.1.0 -m "v0.1.0"
git push origin v0.1.0
```

The `Release` workflow creates the GitHub Release and uploads:

- `pytree-v0.1.0-macos-arm64.tar.gz`
- `pytree-v0.1.0-macos-x86_64.tar.gz`
- `pytree-v0.1.0-linux-x86_64.tar.gz`
- `pytree-v0.1.0-windows-x86_64.zip`

It publishes to crates.io before creating the GitHub Release. If crates.io publish fails,
the release assets are not uploaded.

## Homebrew

After the GitHub Release has assets, compute checksums:

```sh
VERSION=0.1.0
curl -LO https://github.com/0x12th/pytree/releases/download/v$VERSION/pytree-v$VERSION-macos-arm64.tar.gz
curl -LO https://github.com/0x12th/pytree/releases/download/v$VERSION/pytree-v$VERSION-macos-x86_64.tar.gz
shasum -a 256 pytree-*.tar.gz
```

Update the formula in `0x12th/homebrew-pytree`:

```sh
./scripts/update-homebrew-formula.sh 0.1.0 ../homebrew-pytree
brew audit --strict Formula/pytree.rb
brew test Formula/pytree.rb
git add Formula/pytree.rb
git commit -m "pytree 0.1.0"
git push
```

Install command:

```sh
brew tap 0x12th/pytree
brew install pytree
```

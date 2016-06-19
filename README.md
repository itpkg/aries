# Aries

A web framework for rust language.

## Devel

### Rust

```
pacman -S rust cargo
cargo test -- --nocapture
```

### Plugins(for atom)

```
apm install atom-beautify
apm install language-rust
cargo install rustfmt
echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> ~/.zshrc
```

## Build

```
cargo build --release
strip target/release/aries
```

## Thinks

- <https://github.com/hyperium/hyper>

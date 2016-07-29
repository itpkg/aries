# Aries

A web framework for rust language.

## Devel

### Rust

```
pacman -S rust cargo
git clone https://github.com/itpkg/aries.git
cd aries
cargo test -- --nocapture
```

### Plugins(for atom)

```
apm install atom-beautify
apm install language-rust
apm install linter-rust
cargo install rustfmt
echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> ~/.zshrc
```

## Build

```
cargo build --release
strip target/release/aries
```

## Notes
* Format rust code file
```
cargo fmt -- --write-mode=replace
```
OR
```
find . -type f -name "*.rs" -exec rustfmt --write-mode replace {} \;
```

## Thinks

- <https://doc.rust-lang.org/stable/book/>
- <http://zsiciarz.github.io/24daysofrust/>
- <https://github.com/hyperium/hyper>
- <https://github.com/kud1ing/awesome-rust>

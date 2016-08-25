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
cargo install rustfmt
cargo install racer
echo "RUST_SRC_PATH=\$HOME/src/rust\nPATH=\$HOME/.cargo/bin:\$PATH\nexport RUST_SRC_PATH PATH" >> ~/.zshrc
```

remember to enable autosave

```
git clone https://github.com/rust-lang/rust.git $RUST_SRC_PATH
```

* atom-beautify(beautify on save)
* git-plus
* language-rust
* [racer](https://github.com/phildawes/racer)
* autosave(enable)
* language-mustache
* language-gitignore


## Build

```
cargo build --release
strip target/release/aries
RUST_LOG=info ./target/release/aries
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
- <https://kaisery.gitbooks.io/rust-book-chinese/content/>
- <http://mustache.github.io/>

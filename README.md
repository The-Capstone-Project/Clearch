# Clearch

A command line search engine based on AI models like gemini (more soon to come), Written in rust

## Project Structure

```text
.
├── .gitignore
├── .vscode/
│   └── settings.json
├── build.sh
├── Cargo.lock
├── Cargo.toml
├── packager.py
├── README.md
├── src/
│   ├── llms/
│   │   ├── claude.rs
│   │   ├── gemini.rs
│   │   ├── gpt.rs
│   │   └── mod.rs
│   ├── main.rs
│   ├── prompt
│   └── traits.rs
└── test.py
```

## Description of working

Clearch is a command-line search engine that uses AI models for its operation. It's written in Rust, with the main logic of the project located in the ` src ` directory.

The ` src/llms ` directory contains the implementation of different AI models that Clearch uses. Each file in this directory corresponds to a specific AI model. For example, ` gemini.rs ` contains the implementation for the Gemini model, ` claude.rs ` for the Claude model, and so on. These models are structured as Rust modules and are imported in ` src/llms/mod.rs `.

The `LLMRequest` trait defined in ``src/traits.rs``is implemented by each of these models. This trait defines a common interface that all models must adhere to, ensuring that they can be used interchangeably in the main program. For example, the `ClaudeModel` struct in ``src/llms/claude.rs``implements the `LLMRequest` trait.

The `clap` library is used in ``src/main.rs``  to handle command-line inputs. It's a popular Rust library for parsing command-line arguments. In this project, it's used to define the command-line interface for the Clearch program. The `#derive(Parser)` attribute is used to automatically generate the necessary code for parsing command-line arguments into a `Gemini` struct. This struct represents the command-line interface of the program, with each field representing a different command-line argument or option.

Here are the relevant files and symbols:

- `src/llms/mod.rs`
- `src/llms/gemini.rs`
- `src/llms/claude.rs`
- `src/traits.rs`
- `LLMRequest`
- `src/main.rs`
- `Gemini`

### packager.py

The `packager.py` script performs the following steps:

1. Changes to the Rust project directory.
2. Builds the Rust project using `cargo build --release`.
3. Installs `cargo-deb` if it's not already installed.
4. Packages the executable into a .deb file using `cargo deb`.
5. Verifies the .deb file using `dpkg-deb -I`.
6. Prints the path of the .deb file.

You can run the scripat using the following command:

```sh
python3 packager.py
```

This will build the project and create a .deb file in the current directory.

### PKGBUILD

The `PKGBUILD` script is used to build and package the Clearch project for Arch Linux. Here are the steps it performs:

1. Defines the package metadata, including the package name (`clearch`), version (`0.1.0`), architecture (`x86_64`), URL, license (`MIT`), and dependencies (`glibc`).
2. Specifies the build dependencies (`rust`, `cargo`).
3. Defines the source file URL.
4. Specifies the build command (`cargo build --release --locked --all-features --target-dir=target`).
5. Defines the package function, which installs the `clearch` executable, the `LICENSE` file, and the `README.md` file to the appropriate directories in the package.

Here is the content of the `PKGBUILD` file:

```sh
# Maintainer: Eric TK <ericatkusa@gmail.com>
pkgname=clearch
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple CLI tool written in Rust"
arch=('x86_64')
url="https://github.com/Zane-Dev14/ClearchOnlyTar"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
source=("https://github.com/Zane-Dev14/ClearchOnlyTar/raw/main/clearch-cli-0.1.0.tar.gz")
sha256sums=('SKIP')
build() {
    cd "$srcdir"
    cargo build --release --locked --all-features --target-dir=target
}
package() {
    install -Dm755 "$srcdir/target/release/clearch" "$pkgdir/usr/bin/clearch"
    install -Dm644 "$srcdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "$srcdir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
```

This will build the project and create a package for Arch Linux.

---

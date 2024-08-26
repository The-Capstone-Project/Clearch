# CONTRIBUTION DETAILS

## Table of Contents 📚
- -> [Relevant Files and Symbols 📂](#relevant-files-and-symbols-)
- -> [Packaging Scripts 📦](#packaging-scripts-)
- -> [packager.py 🐍](#packagerpy-)
- -> [PKGBUILD 💽](#pkgbuild-)

#### Relevant Files and Symbols 📂 

- `src/llms/mod.rs` 
- `src/llms/gemini.rs` 
- `src/llms/claude.rs` 
- `src/traits.rs` 
- `LLMRequest` 
- `src/main.rs` 
- `Gemini` 

## Packaging Scripts 📦 

### packager.py 🐍 

The `packager.py` script automates the build and packaging process of the Rust project. It performs the following steps: 

1. Implements changes in the Rust project directory. 
2. Builds the Rust project using `cargo build --release`. 
3. Installs `cargo-deb` if it is not already installed. 
4. Packages the executable into a `.deb` file using `cargo deb`. 
5. Verifies the `.deb` file using `dpkg-deb -I`. 
6. Prints the path of the generated `.deb` file. 

To run the script, execute the following command: 

```sh 
python3 packager.py 
``` 

This command compiles the project and produces a `.deb` file in the current directory. 📄 

### PKGBUILD 💽 

The `PKGBUILD` script is tailored for building and packaging Clearch for Arch Linux. Here’s a summary of its operation: 

1. Defines package metadata: name (`clearch`), version (`0.1.0`), architecture (`x86_64`), URL, license (`MIT`), and dependencies (`glibc`). 
2. Specifies build dependencies: `rust`, `cargo`. 
3. Defines the source file URL. 
4. Specifies the build command: `cargo build --release --locked --all-features --target-dir=target`. 
5. Defines the package function to install the `clearch` executable, the `LICENSE` file, and the `README.md` to their respective directories in the package. 

The `PKGBUILD` file is as given below: 

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

This script will effectively build the project and generate a package for Arch Linux. 🛠️ 

---

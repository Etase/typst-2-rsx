# typst-2-rsx

Convert **Typst** documents (`.typ`) into **RSX** elements as SVG components.

## 📌 Overview

`typst-2-rsx` is a Rust library that automates the conversion of **Typst** documents into **RSX** elements. It first uses the `typst` CLI to compile `.typ` files into `.svg`, and then parses the `.svg` file into RSX elements, making it easy to integrate Typst-generated content into Rust-based UI frameworks like **Dioxus**.

## 🚀 Installation

### Using Cargo

```sh
cargo add typst-2-rsx
```

Or manually add to Cargo.toml:

```toml
[dependencies]
typst-2-rsx = "0.1.2"
```

Then run the cargo build to compile.

## 📖 Usage

```rust
use typst_2_rsx::typst_to_rsx;

let rsx_svg = typst_to_rsx("example.typ").expect("Conversion failed");
println!("{}", rsx_svg);
```

## 🔧 How It Works

1. **Compiles** `.typ` to `.svg` using the `typst` CLI.
2. **Parses** the generated `.svg` file.
3. **Transforms** the SVG elements into **RSX components**.

## 🛠 Dependencies

- [typst-cli](https://github.com/typst/typst) – Required for Typst compilation.
- [dioxus](https://dioxuslabs.com/)(optional) – For rendering RSX.

## 📜 License

This project is licensed under **MIT OR Apache-2.0**.

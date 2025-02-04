# typst-2-rsx

Convert **Typst** documents (`.typ`) into **RSX** elements as SVG components.

## 📌 Overview

`typst-2-rsx` is a Rust library that automates the conversion of **Typst** documents into **RSX** elements. It first uses the `typst` CLI to compile `.typ` files into `.svg`, and then parses the `.svg` file into RSX elements, making it easy to integrate Typst-generated content into Rust-based UI frameworks like **Dioxus**.

## ✨ Features

- 🖋 **Automatic conversion**: Transform `.typ` documents into RSX-compatible SVG elements.
- ⚡ **Seamless integration**: Use Typst-generated content in Rust UI frameworks.
- 🔄 **CLI and Library support**: Can be used both as a library and a command-line tool.

## 🚀 Installation

### Using Cargo

```sh
cargo add typst-2-rsx
```

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

## 💡 Example Output

### **Input (`example.typ`)**

```typst
#set text(20pt)
Hello, **Typst**!
```

### **Generated RSX**

```rust
rsx! {
    svg {{
        text {{ x: "10", y: "20", "Hello, ", tspan {{ font-weight: "bold", "Typst!" }} }}
    }}
}
```

## 🛠 Dependencies

- [typst-cli](https://github.com/typst/typst) – Required for Typst compilation.
- [xml-rs](https://crates.io/crates/xml-rs) – Used for parsing SVG XML.
- [dioxus](https://dioxuslabs.com/) – For rendering RSX (optional).

## 🤝 Contributing

We welcome contributions! To get started:

1. Fork the repository.
2. Create a new branch (`feature/my-feature`).
3. Commit your changes.
4. Push to your branch and open a PR.

## 📜 License

This project is licensed under **MIT License**.

---

💡 **Need help?** Open an issue or reach out to us on GitHub!

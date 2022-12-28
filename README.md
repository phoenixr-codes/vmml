VMML
====

Very Minimal Markup Language (VMML) is a lightweight markup language that supports
nested elements that each contain an attribute.

The purpose of VMML is giving the developer the ability to define how the attribute
syntax is. Other things like escaping and capturing elements is done by the parser for
the developer.

An example of a VMML document looks as follows:
```text
The [quick](bold) brown [fox](orange) jumps [over the [lazy](bold) dog](italic).
```

* `quick` is an element with an attribute `bold`.
* `fox` is an elment with an attribute `orange`.
* `over the lazy dog` is an element with an attribute `italic`.
* Inside the above element is another element `lazy` with an attribute bold. It is a child of the
  element `over the lazy dog`.

An XML version of this **could** be the following:
```xml
The <bold>quick</bold> brown <orange>fox</orange> jumps <italic>over the <bold>lazy</bold> dog</italic>.
```

**VMML has no built-in support to translate files into other markup languages sich as XML.**

The file extension for VMML files is `.vmml`.


Usage
=====

```toml
[dependencies]
vmml = "1.0.0"
```

```rust
extern crate vmml;

fn main() {
    let document = "Hello, [World](bold)!";
    let tree = vmml::parse(&document);
    println!("{:#?}", tree);
}
```


Links
=====
* [Documentation](https://docs.rs/vmml/latest/vmml/)
* [Repository](https://github.com/phoenixr-codes/vmml)
* [crates.io](https://crates.io/crates/vmml)



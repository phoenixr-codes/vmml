//! VMML
//! ====
//!
//! Very Minimal Markup Language (VMML) is a lightweight markup language that supports
//! nested elements that each contain an attribute.
//!
//! The purpose of VMML is giving the developer the ability to define how the attribute
//! syntax is. Other things like escaping and capturing elements is done by the parser for
//! the developer.
//!
//! An example of a VMML document looks as follows:
//! ```text
//! The [quick](bold) brown [fox](orange) jumps [over the [lazy](bold) dog](italic).
//! ```
//!
//! * `quick` is an element with an attribute `bold`.
//! * `fox` is an elment with an attribute `orange`.
//! * `over the lazy dog` is an element with an attribute `italic`.
//! * Inside the above element is another element `lazy` with an attribute bold. It is a child of the
//!   element `over the lazy dog`.
//!
//! An XML version of this **could** be the following:
//! ```xml
//! The <bold>quick</bold> brown <orange>fox</orange> jumps <italic>over the <bold>lazy</bold> dog</italic>.
//! ```
//!
//! **VMML has no built-in support to translate files into other markup languages sich as XML.**
//!
//! The file extension for VMML files is `.vmml`.
//!
//!
//! Usage
//! =====
//!
//! ```toml
//! [dependencies]
//! vmml = "1.0.0"
//! ```
//!
//! ```rust
//! extern crate vmml;
//!
//! fn main() {
//!     let document = "Hello, [World](bold)!";
//!     let tree = vmml::parse(&document);
//!     println!("{:#?}", tree);
//! }
//! ```
//!
//!
//! Links
//! =====
//! * [Documentation](https://docs.rs/vmml/latest/vmml/)
//! * [Repository](https://github.com/phoenixr-codes/vmml)
//! * [crates.io](https://crates.io/crates/vmml)

#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "vmml.pest"]
struct VMMLParser;

/// A node may be text or an `element`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Node<'a> {
    /// Node as text.
    Text(&'a str),
    /// Node as element..
    Element {
        /// The content of the `element`.
        inner: Vec<Box<Node<'a>>>,
        /// The attribute of the `element`.
        attr: &'a str,
    },
}

type Nodes<'a> = Vec<Box<Node<'a>>>;

fn parse_element<'a>(content: &mut Nodes<'a>, pairs: Pairs<'a, Rule>) -> Option<&'a str> {
    let mut result: Option<&'a str> = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::text_attr => {
                result = Some(pair.as_str());
            }

            Rule::element => {
                let mut elem_content: Nodes<'a> = Vec::new();
                let elem_attr = parse_element(&mut elem_content, pair.into_inner());
                content.push(Box::new(Node::Element {
                    inner: elem_content,
                    attr: elem_attr.unwrap(),
                }));
            }

            Rule::text | Rule::text_elem => {
                content.push(Box::new(Node::Text(pair.as_str())));
            }

            _ => {}
        }
    }
    result
}

/// Parses a VMML document.
///
/// # Arguments
///
/// * `document` - The VMML document.
///
/// # Examples
///
/// ```
/// let parse1 = vmml::parse(r#"Hello, [wonderful](uppercase) World"#);
/// let parse2 = vmml::parse(r#"Hello, \[wonderful\]\(uppercase\) World"#);
/// ```
pub fn parse<'a>(document: &'a str) -> Nodes {
    let mut result: Nodes = Vec::new();
    let parse = VMMLParser::parse(Rule::document, document);
    match parse {
        Ok(mut pairs) => {
            let doc = pairs.next().unwrap();
            parse_element(&mut result, doc.into_inner());
        }
        Err(e) => panic!("{e}"),
    }
    result
}

/// Escapes all tokens of VMML (backslashes, brackets, parens)
///
/// # Arguments
///
/// * `text` - The text to escape.
///
/// #  Examples
///
/// ```
/// let text = vmml::escape(r#"[Hello](World)"#);
/// ```
pub fn escape(text: &str) -> String {
    text.replace(r#"\"#, r#"\\"#)
        .replace(r#"["#, r#"\["#)
        .replace(r#"]"#, r#"\]"#)
        .replace(r#"("#, r#"\("#)
        .replace(r#")"#, r#"\)"#)
}

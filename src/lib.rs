//! Very Minimal Markup Language (VMML) is a lightweight markup language that supports
//! nested `fields` (also known as "elements" in other markup languages such as XML)
//! that contain an attribute.
//!
//! The purpose of VMML is giving the developer the ability to define how the attribute
//! syntax is. Other things like escaping and capturing fields is done by the parser for
//! the developer.
//!
//! An example of a VMML document looks as follows
//!
//! ```text
//! The [quick](bold) brown [fox](orange) jumps [over the [lazy](bold) dog](italic).
//! ```
//!

#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "vmml.pest"]
struct VMMLParser;

/// A node may be text or a `field`.
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    /// Text of node.
    Text(&'a str),
    /// Field of node.
    Field {
        /// The content of the `field`.
        inner: Vec<Box<Node<'a>>>,
        /// Attribute of the `field`.
        attr: &'a str,
    },
}

type Nodes<'a> = Vec<Box<Node<'a>>>;

fn parse_field<'a>(content: &mut Nodes<'a>, pairs: Pairs<'a, Rule>) -> Option<&'a str> {
    let mut result: Option<&'a str> = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::text_attr => {
                result = Some(pair.as_str());
            }

            Rule::field => {
                let mut field_content: Nodes<'a> = Vec::new();
                let field_attr = parse_field(&mut field_content, pair.into_inner());
                content.push(Box::new(Node::Field {
                    inner: field_content,
                    attr: field_attr.unwrap(),
                }));
            }

            Rule::text | Rule::text_target => {
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
            parse_field(&mut result, doc.into_inner());
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

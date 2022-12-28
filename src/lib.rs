#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "vmml.pest"]
struct VMMLParser;

/// A node may be text or a `field`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Node<'a> {
    /// Text of node.
    Text(&'a str),
    /// Field of node.
    Element {
        /// The content of the `field`.
        inner: Vec<Box<Node<'a>>>,
        /// Attribute of the `field`.
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

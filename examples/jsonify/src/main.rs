extern crate vmml;

fn jsonify(document: &str) -> serde_json::Result<String> {
    let tree = vmml::parse(document);
    serde_json::to_string(&tree)
}

fn main() {
    let json = jsonify("Hello [World](bold)").unwrap();
    println!("{json}");
}

use vmml::Node;

#[test]
fn parse_normal_text() {
    assert_eq!(
        vmml::parse(r#"Hello, beautiful World"#),
        vec![Box::new(Node::Text(r#"Hello, beautiful World"#))]
    );
}

#[test]
fn parse_one_field() {
    assert_eq!(
        vmml::parse(r#"Hello, [beautiful](bold) World"#),
        vec![
            Box::new(Node::Text(r#"Hello, "#)),
            Box::new(Node::Field {
                inner: vec![Box::new(Node::Text(r#"beautiful"#))],
                attr: r#"bold"#,
            }),
            Box::new(Node::Text(r#" World"#)),
        ]
    );
}

#[test]
fn parse_nested() {
    assert_eq!(
        vmml::parse(r#"Hello, [[beauti](italic)ful](bold) World"#),
        vec![
            Box::new(Node::Text(r#"Hello, "#)),
            Box::new(Node::Field {
                inner: vec![
                    Box::new(Node::Field {
                        inner: vec![Box::new(Node::Text(r#"beauti"#))],
                        attr: r#"italic"#,
                    }),
                    Box::new(Node::Text(r#"ful"#)),
                ],
                attr: r#"bold"#,
            }),
            Box::new(Node::Text(r#" World"#)),
        ]
    );
}

#[test]
fn parse_multi_nested() {
    assert_eq!(
        vmml::parse(r#"a[b[c[d](z)](y)](x)"#),
        vec![
            Box::new(Node::Text(r#"a"#)),
            Box::new(Node::Field {
                inner: vec![
                    Box::new(Node::Text(r#"b"#)),
                    Box::new(Node::Field {
                        inner: vec![
                            Box::new(Node::Text(r#"c"#)),
                            Box::new(Node::Field {
                                inner: vec![Box::new(Node::Text(r#"d"#)),],
                                attr: r#"z"#,
                            }),
                        ],
                        attr: r#"y"#,
                    }),
                ],
                attr: r#"x"#,
            }),
        ]
    );
}

#[test]
fn parse_multiple() {
    assert_eq!(
        vmml::parse(r#"[Hello](bold), beautiful [World](italic)"#),
        vec![
            Box::new(Node::Field {
                inner: vec![Box::new(Node::Text(r#"Hello"#)),],
                attr: r#"bold"#,
            }),
            Box::new(Node::Text(r#", beautiful "#)),
            Box::new(Node::Field {
                inner: vec![Box::new(Node::Text(r#"World"#)),],
                attr: r#"italic"#,
            }),
        ]
    );
}

#[test]
fn parse_empty() {
    assert_eq!(vmml::parse(r#""#), vec![]);
}

#[test]
#[should_panic]
fn attr_contains_field() {
    vmml::parse(r#"Hello, [beautiful](a[b](c)) World"#);
}

#[test]
#[should_panic]
fn unclosed_target() {
    vmml::parse(r#"Hello, [beautiful(bold) World"#);
}

#[test]
#[should_panic]
fn unclosed_attr() {
    vmml::parse(r#"Hello, [beautiful](bold World"#);
}

#[test]
#[should_panic]
fn unclosed_target_due_to_escape() {
    vmml::parse(r#"Hello, [beautiful\](bold) World"#);
}

#[test]
#[should_panic]
fn unclosed_attr_due_to_escape() {
    vmml::parse(r#"Hello, [beautiful](bold\) World"#);
}

#[test]
#[should_panic]
fn unclosed_inner_target() {
    vmml::parse(r#"Hello, [[beautiful](bold) World"#);
}

#[test]
#[should_panic]
fn space_between() {
    vmml::parse(r#"Hello, [beautiful] (bold) World"#);
}

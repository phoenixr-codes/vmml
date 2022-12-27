#[test]
fn normal_text() {
    assert_eq!(
        vmml::escape(r#"Hello There"#),
        String::from(r#"Hello There"#)
    );
}

#[test]
fn just_one_backslash() {
    assert_eq!(
        vmml::escape(r#"Hello \ There"#),
        String::from(r#"Hello \\ There"#)
    );
}

#[test]
fn two_backslashes() {
    assert_eq!(
        vmml::escape(r#"Hello \\ There"#),
        String::from(r#"Hello \\\\ There"#)
    );
}

#[test]
fn three_backslashes() {
    assert_eq!(
        vmml::escape(r#"Hello \\\ There"#),
        String::from(r#"Hello \\\\\\ There"#)
    );
}

#[test]
fn escaped_tokens() {
    assert_eq!(
        vmml::escape(r#"[ ] ( ) \"#),
        String::from(r#"\[ \] \( \) \\"#)
    );
}

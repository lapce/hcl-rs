use hcl_edit::parser::parse_body;
use indoc::indoc;
use pretty_assertions::assert_eq;

macro_rules! assert_error {
    ($hcl:expr, $msg:expr) => {
        match parse_body($hcl) {
            Ok(s) => panic!("parsed to: {:#?}", s),
            Err(e) => assert_eq!($msg, e.to_string()),
        }
    };
}

#[test]
fn invalid_structures() {
    assert_error!(
        "foo = 1\nbar [",
        indoc! {r#"
             --> HCL parse error in line 2, column 5
              |
            2 | bar [
              |     ^---
              |
              = invalid structure; expected `{`, `=`, `"` or identifier"#}
    );
}

#[test]
fn invalid_blocks() {
    assert_error!(
        "ident {",
        indoc! {r#"
             --> HCL parse error in line 1, column 8
              |
            1 | ident {
              |        ^---
              |
              = invalid block body; expected `}`, newline or identifier"#}
    );

    assert_error!(
        "ident \"label\" {",
        indoc! {r#"
             --> HCL parse error in line 1, column 16
              |
            1 | ident "label" {
              |                ^---
              |
              = invalid block body; expected `}`, newline or identifier"#}
    );

    assert_error!(
        "ident { foo }",
        indoc! {r#"
             --> HCL parse error in line 1, column 13
              |
            1 | ident { foo }
              |             ^---
              |
              = invalid attribute; expected `=`"#}
    );

    assert_error!(
        "ident { [ }",
        indoc! {r#"
             --> HCL parse error in line 1, column 9
              |
            1 | ident { [ }
              |         ^---
              |
              = invalid block body; expected `}`, newline or identifier"#}
    );
}

#[test]
fn invalid_exprs() {
    assert_error!(
        "ident = ''",
        indoc! {r#"
             --> HCL parse error in line 1, column 9
              |
            1 | ident = ''
              |         ^---
              |
              = invalid expression; expected `"`, `[`, `{`, `-`, `!`, `(`, `_`, `<`, letter or digit"#}
    );

    assert_error!(
        "ident = var.%",
        indoc! {r#"
             --> HCL parse error in line 1, column 13
              |
            1 | ident = var.%
              |             ^---
              |
              = invalid traversal operator; expected `*`, identifier or unsigned integer"#}
    );

    assert_error!(
        "ident = { foo = \"\"\" }",
        indoc! {r#"
             --> HCL parse error in line 1, column 19
              |
            1 | ident = { foo = """ }
              |                   ^---
              |
              = invalid object item; expected `}`, `,` or newline"#}
    );
}

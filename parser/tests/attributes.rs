// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate env_logger;
extern crate revonslang_base as base;
extern crate revonslang_parser as parser;

#[macro_use]
mod support;

use crate::support::*;

#[test]
fn any_tokens() {
    let _ = ::env_logger::try_init();
    let text = r#"
#[test(ident "string" 42 = 'a' + )]
let (+) x y = error ""
{ }
"#;
    parse_clear_span!(text);
}

#[test]
fn bindings() {
    let _ = ::env_logger::try_init();
    let text = r#"
#[infix(left, 6)]
let (+) x y = error ""
#[implicit]
type Test = Int

{
    #[abc()]
    Test,
    #[test]
    t = \_ -> True
}
"#;
    parse_clear_span!(text);
}

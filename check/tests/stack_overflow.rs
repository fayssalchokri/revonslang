// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate revonslang_base as base;
extern crate revonslang_check as check;
extern crate revonslang_parser as parser;

mod support;

#[test]
fn dont_stack_overflow_on_let_bindings() {
    let text = r#"
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in
let _ = 1
in 1
"#;
    support::typecheck(text).unwrap();
}

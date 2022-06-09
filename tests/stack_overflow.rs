// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
use revonslang::{new_vm, ThreadExt};

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
    let vm = new_vm();
    vm.load_script("", text).unwrap();
}

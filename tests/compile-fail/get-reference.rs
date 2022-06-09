// Programmed By Fayssal Chokri <fayssalchokri@revons.co>
// REVONSRDT licence
// Bluespacengineers <no-replay@bluespacengineers.com>
// Revons Developers Team <no-replay-rdt@outlook.com>
// Copyright 2020 Revons Community , Bluespacengineers
extern crate revonslang;
use revonslang::new_vm;
use revonslang::vm::api::Getable;
use revonslang::vm::internal::Value;
use revonslang::vm::Variants;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    unsafe {
        let vm = new_vm();
        let value = Value::int(0);
        let value = Variants::new(&value);
        //~^ Error `value` does not live long enough
        let _: &'static str = <&'static str>::from_value(&vm, value);
    }
}

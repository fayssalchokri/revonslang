extern crate fachok;

fn main() {
    fachok::Configuration::new()
        .use_cargo_dir_conventions()
        .process_file("src/grammar.fachok")
        .unwrap();

    println!("cargo:rerun-if-changed=src/grammar.fachok");
}

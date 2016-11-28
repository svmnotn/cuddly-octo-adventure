#[cfg(feature = "serde_codegen")]
fn main() {
    extern crate serde_codegen;

    use std::env;
    use std::path::PathBuf;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let files = vec!["answer", "archive", "question", "settings", "team", "topic"];

    for s in files {
        let src = PathBuf::from(format!("src/cuddle/{}.in.rs", s));
        let dst = PathBuf::from(&out_dir).join(&format!("{}.rs", s));

        serde_codegen::expand(&src, &dst).unwrap();
    }
}

#[cfg(feature = "serde_derive")]
fn main() {
    // do nothing
}

use std::{env, fs, path::PathBuf};

fn main() {
    let env_str = if cfg!(feature = "test-prod") {
        include_str!("../.env.prod")
    } else {
        include_str!("../.env")
    };

    env_str
        .lines()
        .filter(|x| !x.starts_with("#"))
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            println!("cargo:rustc-env={}", x);
        });
}

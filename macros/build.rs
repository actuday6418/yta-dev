fn main() {
    let env_str = include_str!("../.env");
    env_str
        .lines()
        .filter(|x| !x.starts_with("#"))
        .filter(|x| !x.is_empty())
        .for_each(|x| {
            println!("cargo:rustc-env={}", x);
        });
}

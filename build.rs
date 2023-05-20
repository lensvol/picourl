use serde::Deserialize;
use std::env;
use std::fs::write;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Redirection {
    pub slug: &'static str,
    pub url: &'static str,
}

fn main() {
    let redirections: Vec<Redirection> = serde_yaml::from_str(include_str!("urls.yaml")).unwrap();
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push("generated_redirections.rs");

    write(out, format!("{:?}", redirections)).unwrap()
}

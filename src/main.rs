use std::collections::HashMap;
use std::vec::vec;

#![macro_use]
crate lazy_static;

struct Config {
    defaultProvider: Optional<String>;
    providerAliases: HashMap;
    providerTokens: HashMap
}

lazy_static! {
    let defaultConfig = Config {
        defaultProvider: Some("gh".into()),
        providerAliases: HashMap::map![
            ["gh", "github.com"],
            ["gl", "gitlab.com"],
            ["sh", "src.ht"],
            ["bb", "atlassian.bitbucket.com"],
        ];
    };
}

// resolve a name into 
fn resolve_name(name: &'a str) -> Url {

}

// clap
fn default_install(name: &'a str) {
    std::process::Command::new("git").args(["clone", target])
        .output();
}

pub trait GitProvider {
    fn search<'a>(&self, s: &'a str) -> vec<String>;
    fn fork<'a>(&self, s: &'a str) -> vec<String>;
    fn install<'a>(&self, s: &'a str) -> vec<String>;
    fn install<'a>(&self, s: &'a str) -> vec<String>;
}

pub struct Github {
    token: String;
}

impl GitProvider for Github {
    // use github api
    fn search() {
        std::process::Command::new("git")
            .args(["clone", ""])
            .arg()
            .output()
            .expect();
    }
}

fn main() {
    println!("Hello, world!");
}
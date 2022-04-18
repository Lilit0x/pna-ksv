use clap::{arg, Command};

pub struct KvStore {

}

impl KvStore {
    pub fn new() -> Self { Self {} }

    pub fn set(&mut self, _key: String, _value: String) {
      panic!()
    }

    pub fn get(&mut self, _key: String) -> Option<String> {
      panic!()
    }

    pub fn remove(&mut self, _key: String) {
      panic!()
    }
}

pub fn cli() -> Command<'static> {
  Command::new("kvs")
    .about("A key store value cli tool. My version of the PNA Rust projects")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .version("v0.1.24")
    .author("Lilit, baasisek01@gmail.com")
    .subcommand(
      Command::new("get")
        .about("Retrieves a key")
        .arg(arg!(<key> "The key to retrieve"))
        .arg_required_else_help(true)
    )
    .subcommand(
      Command::new("set")
      .about("Saves a key to the value")
      .args(&[arg!(<key> "The key for the value"), arg!(<value> "The value to set")])
      .arg_required_else_help(true)
    )
    .subcommand(
      Command::new("rm")
      .about("Removes the specified key")
      .arg(arg!(<key> "The key to be removed"))
      .arg_required_else_help(true)
    )
}

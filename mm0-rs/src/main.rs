extern crate log;

mod server;
use std::{env, io};

fn main() -> io::Result<()> {
  let mut args = env::args().skip(1);
  match args.next().expect("expected a subcommand").as_str() {
    "server" => Ok(server::main(args)),
    _ => panic!("incorrect subcommand, expected {server}")
  }
}

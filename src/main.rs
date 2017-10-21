extern crate clap;
extern crate reqwest;
extern crate tar;
extern crate libflate;

mod cmd;

use clap::{App};
use cmd::{install, ls};

fn main() {
    let nom = App::new("nom")
        .version("1.0.0")
        .author("Aodhan Hayter <aodhan.hayter@gmail.com>")
        .about("Management program for node.js versions")
        .subcommand(install::init())
        .subcommand(ls::init())
        .get_matches();

    match nom.subcommand() {
        ("install", Some(args)) => install::run(&args),
        ("ls", Some(args))      => println!("ls was used"),
        _                       => println!("No subcommands")
    }
}

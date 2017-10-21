extern crate clap;
use clap::{App, Arg, SubCommand};

pub fn init<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("ls")
}


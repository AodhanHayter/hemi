use std::env::{home_dir};
use std::path::{PathBuf};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::io::{Read, Write};
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::{get, Response};
use tar::Archive;
use libflate::gzip;

static NODE_BASE: &'static str = "https://nodejs.org/dist";

pub fn init<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("install")
        .arg(Arg::with_name("version")
        .short("v")
        .help("Installs a specified node.js version")
        .value_name("NODE_VERSION")
        .takes_value(true))
}

pub fn run(args: &ArgMatches) {
    match args.value_of("version") {
        Some(version) => install_version(&version),
        None          => println!("No Version Supplied") // Prompt them to install a version
    }
}

fn install_version(version: &str) {
    println!("Installing node.js at {}", version);
    match get_install_location(version) {
        Ok(mut install_path) => download_version(&version, &mut install_path),
        Err(err)             => println!("{:?}", err)
    }
}

fn download_version (version: &str, install_path: &mut PathBuf) {
    let url = build_url(&version);
    match get(&url) {
        Ok(resp) => write_file(resp, version, install_path),
        Err(err) => println!("{:?}", err)
    }
}

fn write_file (mut resp: Response, version: &str, install_path: &mut PathBuf) {
    install_path.push(version);
    let mut buf = vec![];
    resp.read_to_end(&mut buf).unwrap();

    let mut decoder = gzip::Decoder::new(buf.as_slice()).unwrap();
    let mut dcomp_data = Vec::new();
    decoder.read_to_end(&mut dcomp_data).unwrap();

    File::create(&install_path)
        .map_err(|err| println!("{:?}", err))
        .map(|mut file| {
          file.write_all(&dcomp_data).unwrap();
          println!("{}", version);
          let mut archive = Archive::new(File::open(&install_path).unwrap());
          archive.unpack(&install_path).unwrap();
          // for file in archive.entries().unwrap() {
          //     let mut f = file.unwrap();
          //     println!("{:?}", f.header().path().unwrap());
          // }
        })
        .unwrap();
}

fn build_url (version: &str) -> String {
    let platform = "darwin-x64"; // TODO conditional compilation for linux and Windows
    format!("{base}/{version}/node-{version}-{platform}.tar.gz", base = NODE_BASE, version = version, platform = platform)
}

fn get_install_location (version: &str) -> Result<PathBuf, String>{
    home_dir().ok_or_else(|| "No Home Directory Found")
        .map_err(|err| err.to_string())
        .map(|mut home_path| {
            home_path.push(".hemi/".to_string() + version);
            if home_path.exists() {
                home_path
            } else {
                println!("CREATING DIR {:?}", home_path);
                create_dir_all(&home_path)
                    .map_err(|err| err.to_string())
                    .map(|_| home_path)
                    .unwrap()
            }
        })
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::{Path, PathBuf};
    use std::fs::{create_dir, remove_dir_all};
    use super::{get_install_location, build_url};

    #[test]
    fn gets_install_location() {
        let tmp_dir = env::temp_dir();
        env::set_var("HOME", tmp_dir);
        let home_dir = env::var("HOME").unwrap();
        println!("{:?}", home_dir);
        let expected_loc = PathBuf::from(home_dir + "/.nom/v1.0.0");
        let actual_loc = get_install_location("v1.0.0");
        match actual_loc {
            Err(err)     => panic!(err),
            Ok(location) => assert_eq!(expected_loc, location)
        }
    }

    #[test]
    fn formats_url_correctly() {
        let expected_url = "https://nodejs.org/dist/v1.0.0/node-v1.0.0-darwin-x64.tar.gz";
        let actual_url = build_url("v1.0.0");
        assert_eq!(expected_url, actual_url)
    }
}

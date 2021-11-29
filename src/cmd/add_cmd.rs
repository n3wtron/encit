use crate::{EncItError, EncItPEM};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{stdin, Read};

pub fn add_cmd<'a>(name: &str) -> App<'a, 'a> {
    SubCommand::with_name(name)
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .takes_value(true)
                .required(true)
                .possible_values(&["pem", "hex-pem", "base64-pem"]),
        )
        .arg(
            Arg::with_name("key-file")
                .takes_value(true)
                .help("key file"),
        )
}

pub fn get_key(arg_matches: &ArgMatches) -> Result<EncItPEM, EncItError> {
    let key_content = read_key(arg_matches)?;
    let format = arg_matches.value_of("format").unwrap();
    match format {
        "pem" => Ok(EncItPEM::Pem(key_content)),
        "hex-pem" => Ok(EncItPEM::Hex(key_content)),
        "base64-pem" => Ok(EncItPEM::Base64(key_content)),
        _ => Err(EncItError::InvalidCommand(String::new())),
    }
}

fn read_key(arg_matches: &ArgMatches) -> Result<String, EncItError> {
    let mut file_cnt = String::new();
    if let Some(file_path) = arg_matches.value_of("key-file") {
        let mut key_file = File::open(file_path)?;
        key_file.read_to_string(&mut file_cnt)?;
    } else {
        let mut stdin_cnt = String::new();
        stdin().read_to_string(&mut stdin_cnt)?;
    }
    Ok(file_cnt)
}

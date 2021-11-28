use crate::{encrypt, EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::debug;
use std::fs::File;
use std::io::{stdin, Read};
use std::rc::Rc;

pub fn encrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("encrypt")
        .about("encrypt a file/text")
        .arg(
            Arg::with_name("identity")
                .long("identity")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("Identity name (has to be present in the encit configuration file)"),
        )
        .arg(
            Arg::with_name("friend")
                .long("friend")
                .short("f")
                .required(true)
                .takes_value(true)
                .help("Friend name (has to be present in the encit configuration file)"),
        )
        .arg(
            Arg::with_name("subject")
                .long("subject")
                .short("s")
                .takes_value(true)
                .help("Message subject"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("file to encrypt"),
        )
}

pub fn encrypt_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity").unwrap();
    let friend = cmd_matches.value_of("friend").unwrap();
    let subject = cmd_matches.value_of("subject");
    let mut message = Vec::new();
    read_byte_message(cmd_matches, &mut message)?;

    debug!("message: {:?}", &message);
    let b64_message = base64::encode(message);
    let enc_message = encrypt(config, identity, friend, subject, &b64_message)?;
    println!("{}", enc_message);
    Ok(())
}

fn read_byte_message(
    cmd_matches: &ArgMatches,
    encrypted_message: &mut Vec<u8>,
) -> Result<(), EncItError> {
    if let Some(file_path) = cmd_matches.value_of("file") {
        let mut fl = File::open(file_path)?;
        fl.read_to_end(encrypted_message)?;
    } else {
        stdin().read_to_end(encrypted_message)?;
    };
    Ok(())
}

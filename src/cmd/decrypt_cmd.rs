use crate::{decrypt, EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::debug;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::rc::Rc;

pub fn decrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("decrypt")
        .about("decrypt an encIt message")
        .arg(
            Arg::with_name("identity")
                .long("identity")
                .short("i")
                .takes_value(true)
                .help("Identity name (has to be present in the encit configuration file)"),
        )
        .arg(Arg::with_name("json").long("json"))
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("file to encrypt"),
        )
}

pub fn decrypt_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity");
    let mut encrypted_message = String::new();
    read_string_message(cmd_matches, &mut encrypted_message)?;
    debug!("encrypted message: {}", encrypted_message);

    let decrypted_message = decrypt(config, &encrypted_message, identity)?;
    if cmd_matches.is_present("json") {
        println!("{}", serde_json::to_string(&decrypted_message).unwrap());
    } else {
        let payload = base64::decode(decrypted_message.payload())?;
        stdout().write_all(payload.as_slice())?;
    }
    Ok(())
}

fn read_string_message(
    cmd_matches: &ArgMatches,
    encrypted_message: &mut String,
) -> Result<(), EncItError> {
    if let Some(file_path) = cmd_matches.value_of("file") {
        let mut fl = File::open(file_path)?;
        fl.read_to_string(encrypted_message)?;
    } else {
        stdin().read_to_string(encrypted_message)?;
    };
    *encrypted_message = encrypted_message.trim().to_string();
    Ok(())
}

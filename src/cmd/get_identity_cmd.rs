use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::rc::Rc;

pub fn get_identity_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("identity")
        .arg(Arg::with_name("name").takes_value(true).required(true))
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .takes_value(true)
                .required(true)
                .default_value("hex-pem")
                .possible_values(&["pem", "hex-pem", "base64-pem"]),
        )
        .arg(
            Arg::with_name("private-key")
                .long("private-key")
                .help("display private key"),
        )
}

pub fn get_identity_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity_name = arg_matches.value_of("name").unwrap();
    let identity = config.identity(identity_name);
    if identity.is_none() {
        return Err(EncItError::IdentityNotFound(identity_name.to_string()));
    }
    let identity = identity.unwrap();
    println!("Identity: {}", identity_name);
    let public_key = match arg_matches.value_of("format").unwrap() {
        "pem" => identity
            .private_key()
            .public_key_pem()
            .map(|pub_key_vec| String::from_utf8(pub_key_vec).unwrap())?,
        "base64-pem" => identity
            .private_key()
            .public_key_pem()
            .map(base64::encode)?,
        _ => identity.private_key().public_key_pem_hex()?,
    };
    println!("Public Key: {}", public_key);
    if arg_matches.is_present("private-key") {
        println!(
            "Private Key: {}",
            identity
                .private_key()
                .pem()
                .map(|pub_key_vec| String::from_utf8(pub_key_vec).unwrap())?
        );
    }
    Ok(())
}

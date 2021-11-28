use std::fs::{create_dir, File};
use std::io::{stdin, stdout, Read, Write};
use std::rc::Rc;
use std::string::String;

use clap::{App, Arg, ArgMatches, SubCommand};
use log::debug;
use openssl::rsa::Rsa;

use crate::config::{EncItConfig, EncItConfigImpl, EncItPEM};
use crate::enc::{decrypt, encrypt};
use crate::errors::EncItError;
use crate::EncItError::InvalidCommand;

mod config;
mod enc;
mod errors;

fn main() -> Result<(), EncItError> {
    env_logger::init();

    let config: Rc<dyn EncItConfig> = get_config()?;

    let app = App::new("encit")
        .about("offline e2e encryption client")
        .subcommand(encrypt_cmd())
        .subcommand(decrypt_cmd())
        .subcommand(
            SubCommand::with_name("get")
                .subcommand(SubCommand::with_name("friends"))
                .subcommand(SubCommand::with_name("identities"))
                .subcommand(get_identity_cmd()),
        )
        .subcommand(
            SubCommand::with_name("add")
                .subcommand(add_cmd("friend"))
                .subcommand(add_cmd("identity")),
        )
        .subcommand(SubCommand::with_name("new").subcommand(new_identity_cmd()));

    let matches = app.get_matches();

    match matches.subcommand() {
        ("add", Some(add_matches)) => match add_matches.subcommand() {
            ("friend", Some(cmd_matches)) => add_friend_exec(cmd_matches, config),
            ("identity", Some(cmd_matches)) => add_identity_exec(cmd_matches, config),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("get", Some(get_matches)) => match get_matches.subcommand() {
            ("friends", _) => {
                for friend in config.friends() {
                    println!("{}", friend.name());
                }
                Ok(())
            }
            ("identities", _) => {
                for identity in config.identities() {
                    println!("{}", identity.name());
                }
                Ok(())
            }
            ("identity", Some(cmd_args)) => get_identity_exec(cmd_args, config),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("new", Some(get_matches)) => match get_matches.subcommand() {
            ("identity", Some(cmd_matches)) => new_identity_exec(cmd_matches, config),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("encrypt", Some(encrypt_matches)) => encrypt_exec(encrypt_matches, config),
        ("decrypt", Some(encrypt_matches)) => decrypt_exec(encrypt_matches, config),
        (_, _) => Err(EncItError::InvalidCommand(String::new())),
    }
}

fn get_identity_cmd<'a>() -> App<'a, 'a> {
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

fn get_identity_exec(
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

fn new_identity_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("identity").arg(
        Arg::with_name("name")
            .long("name")
            .short("n")
            .takes_value(true)
            .required(true),
    )
}

fn new_identity_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity_name = arg_matches.value_of("name").unwrap();
    let key = Rsa::generate(2048)?;
    let key = EncItPEM::Hex(hex::encode(key.private_key_to_pem()?));
    config.add_identity(identity_name, &key, None)?.save()
}

fn add_identity_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let key = get_key(arg_matches)?;
    let passphrase = arg_matches.value_of("passphrase");
    // check if is a valid public key
    let _ = key.private_key(passphrase)?;
    let hex_hey = EncItPEM::Hex(key.hex_pem()?);

    let identity_name = arg_matches
        .value_of("name")
        .ok_or_else(|| InvalidCommand("identity name is mandatory".into()))?;
    if identity_name.contains(' ') {
        return Err(InvalidCommand("identity name could not have spaces".into()));
    }
    config
        .add_identity(identity_name, &hex_hey, passphrase)?
        .save()
}

fn add_friend_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let key = get_key(arg_matches)?;
    // check if is a valid public key
    let _ = key.public_key()?;
    let hex_hey = EncItPEM::Hex(key.hex_pem()?);

    let friend_name = arg_matches
        .value_of("name")
        .ok_or_else(|| InvalidCommand("friend name is mandatory".into()))?;

    if friend_name.contains(' ') {
        return Err(InvalidCommand("friend name could not have spaces".into()));
    }
    config.add_friend(friend_name, &hex_hey)?.save()
}

fn get_key(arg_matches: &ArgMatches) -> Result<EncItPEM, EncItError> {
    let key_content = read_key(arg_matches)?;
    let format = arg_matches.value_of("format").unwrap();
    match format {
        "pem" => Ok(EncItPEM::Pem(key_content)),
        "hex-pem" => Ok(EncItPEM::Hex(key_content)),
        "base64-pem" => Ok(EncItPEM::Base64(key_content)),
        _ => Err(EncItError::InvalidCommand(String::new())),
    }
}

fn get_config() -> Result<Rc<dyn EncItConfig>, EncItError> {
    let config_file = dirs::home_dir()
        .expect("cannot find home directory")
        .join(".encit")
        .join("config.yml");
    let config = if !&config_file.exists() {
        let config_dir = config_file.parent().unwrap();
        if !config_dir.exists() {
            create_dir(config_dir)?;
        }
        EncItConfigImpl::create(&config_file)?
    } else {
        EncItConfigImpl::load(config_file.as_path())?
    };
    Ok(Rc::new(config))
}

fn encrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("encrypt")
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

fn encrypt_exec(cmd_matches: &ArgMatches, config: Rc<dyn EncItConfig>) -> Result<(), EncItError> {
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

fn decrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("decrypt")
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

fn decrypt_exec(cmd_matches: &ArgMatches, config: Rc<dyn EncItConfig>) -> Result<(), EncItError> {
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
    mut encrypted_message: &mut String,
) -> Result<(), EncItError> {
    if let Some(file_path) = cmd_matches.value_of("file") {
        let mut fl = File::open(file_path)?;
        fl.read_to_string(&mut encrypted_message)?;
    } else {
        stdin().read_to_string(&mut encrypted_message)?;
    };
    *encrypted_message = encrypted_message.trim().to_string();
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

fn add_cmd<'a>(name: &str) -> App<'a, 'a> {
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
            Arg::with_name("stdin")
                .long("stdin")
                .help("read the public key from stdin"),
        )
        .arg(
            Arg::with_name("key-file")
                .takes_value(true)
                .help("key file"),
        )
}

fn read_key(arg_matches: &ArgMatches) -> Result<String, EncItError> {
    if arg_matches.is_present("stdin") {
        let mut stdin_cnt = String::new();
        stdin().read_to_string(&mut stdin_cnt)?;
        Ok(stdin_cnt)
    } else if let Some(file_path) = arg_matches.value_of("key-file") {
        let mut file_cnt = String::new();
        let mut key_file = File::open(file_path)?;
        key_file.read_to_string(&mut file_cnt)?;
        Ok(file_cnt)
    } else {
        Err(EncItError::InvalidCommand(
            "missing key-file parameter".into(),
        ))
    }
}

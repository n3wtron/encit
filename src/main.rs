use std::fs::create_dir;

use std::rc::Rc;
use std::string::String;

use crate::cmd::add_cmd::add_cmd;
use crate::cmd::add_identity_cmd::add_identity_exec;
use crate::cmd::decrypt_cmd::{decrypt_cmd, decrypt_exec};
use crate::cmd::encrypt_cmd::{encrypt_cmd, encrypt_exec};
use crate::cmd::get_identity_cmd::{get_identity_cmd, get_identity_exec};
use crate::cmd::new_identity_cmd::{new_identity_cmd, new_identity_exec};
use clap::{App, SubCommand};

use crate::cmd::add_friend_cmd::add_friend_exec;

use crate::config::{EncItConfig, EncItConfigImpl, EncItPEM};
use crate::enc::{decrypt, encrypt};
use crate::errors::EncItError;
use crate::EncItError::InvalidCommand;

mod cmd;
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
                .about("retrieve encit information")
                .subcommand(SubCommand::with_name("friends"))
                .subcommand(SubCommand::with_name("identities"))
                .subcommand(get_identity_cmd()),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add friend/identity to encit")
                .subcommand(add_cmd("friend"))
                .subcommand(add_cmd("identity")),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create new identity to encIt")
                .subcommand(new_identity_cmd()),
        );

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

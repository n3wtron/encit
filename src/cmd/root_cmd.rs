use crate::cmd::add_cmd::add_cmd;
use crate::cmd::add_friend_cmd::add_friend_exec;
use crate::cmd::add_identity_cmd::{add_identity_cmd, add_identity_exec};
use crate::cmd::decrypt_cmd::{decrypt_cmd, decrypt_exec};
use crate::cmd::encrypt_cmd::{encrypt_cmd, encrypt_exec};
use crate::cmd::get_identity_cmd::{get_identity_cmd, get_identity_exec};
use crate::cmd::new_identity_cmd::{new_identity_cmd, new_identity_exec};
use crate::{EncItConfig, EncItError};
use clap::{App, ArgMatches, SubCommand};
use std::rc::Rc;

use crate::cmd::get_friends_cmd::{get_friends_cmd, get_friends_exec};
use crate::cmd::get_identities_cmd::{get_identities_cmd, get_identities_exec};
#[cfg(test)]
use mockall::automock;

pub struct CommandsImpl {
    config: Rc<dyn EncItConfig>,
}

impl CommandsImpl {
    pub fn new(config: Rc<dyn EncItConfig>) -> Self {
        CommandsImpl { config }
    }
}

#[cfg_attr(test, automock)]
pub trait Commands {
    fn get_config(&self) -> Rc<dyn EncItConfig>;
    fn add_friend<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn add_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn get_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn get_identities<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn get_friends<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn new_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn encrypt<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
    fn decrypt<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError>;
}

impl Commands for CommandsImpl {
    fn get_config(&self) -> Rc<dyn EncItConfig> {
        self.config.clone()
    }

    fn add_friend<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        add_friend_exec(arg_matches, self.get_config())
    }

    fn add_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        add_identity_exec(arg_matches, self.get_config())
    }

    fn get_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        get_identity_exec(arg_matches, self.get_config())
    }

    fn get_identities<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        get_identities_exec(arg_matches, self.get_config())
    }

    fn get_friends<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        get_friends_exec(arg_matches, self.get_config())
    }

    fn new_identity<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        new_identity_exec(arg_matches, self.get_config())
    }

    fn encrypt<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        encrypt_exec(arg_matches, self.get_config())
    }

    fn decrypt<'a>(&self, arg_matches: &'a ArgMatches<'a>) -> Result<(), EncItError> {
        decrypt_exec(arg_matches, self.get_config())
    }
}

pub fn root_cmd<'a>() -> App<'a, 'a> {
    App::new("encit")
        .about("offline e2e encryption client")
        .subcommand(encrypt_cmd())
        .subcommand(decrypt_cmd())
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve encit information")
                .subcommand(get_friends_cmd())
                .subcommand(get_identities_cmd())
                .subcommand(get_identity_cmd()),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add friend/identity to encit")
                .subcommand(add_cmd("friend"))
                .subcommand(add_identity_cmd()),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create new identity to encIt")
                .subcommand(new_identity_cmd()),
        )
}

pub fn root_exec(commands: Rc<dyn Commands>, matches: &ArgMatches) -> Result<(), EncItError> {
    match matches.subcommand() {
        ("add", Some(add_matches)) => match add_matches.subcommand() {
            ("friend", Some(cmd_matches)) => commands.add_friend(cmd_matches),
            ("identity", Some(cmd_matches)) => commands.add_identity(cmd_matches),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("get", Some(get_matches)) => match get_matches.subcommand() {
            ("friends", Some(cmd_matches)) => commands.get_friends(cmd_matches),
            ("identities", Some(cmd_matches)) => commands.get_identities(cmd_matches),
            ("identity", Some(cmd_args)) => commands.get_identity(cmd_args),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("new", Some(get_matches)) => match get_matches.subcommand() {
            ("identity", Some(cmd_matches)) => commands.new_identity(cmd_matches),
            (_, _) => Err(EncItError::InvalidCommand(String::new())),
        },
        ("encrypt", Some(encrypt_matches)) => commands.encrypt(encrypt_matches),
        ("decrypt", Some(encrypt_matches)) => commands.decrypt(encrypt_matches),
        (_, _) => Err(EncItError::InvalidCommand(String::new())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_friend() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec![
            "encit", "add", "friend", "--format", "pem", "--name", "friend1",
        ]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_add_friend()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn add_identity() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec![
            "encit",
            "add",
            "identity",
            "--format",
            "pem",
            "--name",
            "identity1",
        ]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_add_identity()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn get_identity() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec![
            "encit",
            "get",
            "identity",
            "identity1",
            "--format",
            "pem",
        ]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_get_identity()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn get_identities() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec!["encit", "get", "identities"]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_get_identities()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn get_friends() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec!["encit", "get", "friends"]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_get_friends()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn new_identity() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches =
            Rc::new(cmd.get_matches_from(vec!["encit", "new", "identity", "identity1"]));
        let expected_arg_matches = format!(
            "{:?}",
            arg_matches.subcommand().1.unwrap().subcommand().1.unwrap()
        );
        let mut commands = MockCommands::new();
        commands
            .expect_new_identity()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn encrypt() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec![
            "encit",
            "encrypt",
            "--identity",
            "identity1",
            "--friend",
            "friend1",
            "file.txt",
        ]));
        let expected_arg_matches = format!("{:?}", arg_matches.subcommand().1.unwrap());
        let mut commands = MockCommands::new();
        commands
            .expect_encrypt()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }

    #[test]
    fn decrypt() -> Result<(), EncItError> {
        let cmd = root_cmd();
        let arg_matches = Rc::new(cmd.get_matches_from(vec!["encit", "decrypt", "file.txt"]));
        let expected_arg_matches = format!("{:?}", arg_matches.subcommand().1.unwrap());
        let mut commands = MockCommands::new();
        commands
            .expect_decrypt()
            .withf(move |arg_matches_param| {
                expected_arg_matches == format!("{:?}", arg_matches_param)
            })
            .returning(|_| Ok(()));
        let rc_commands: Rc<dyn Commands> = Rc::new(commands);
        root_exec(rc_commands, &arg_matches)
    }
}

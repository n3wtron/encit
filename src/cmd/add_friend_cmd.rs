use crate::cmd::add_cmd::{get_key, get_key_reader, KeyReader};
use crate::{EncItConfig, EncItError, EncItPEM, InvalidCommand};
use clap::ArgMatches;
use std::cell::RefCell;
use std::rc::Rc;

pub fn add_friend_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let key_reader = RefCell::new(get_key_reader(arg_matches)?);
    add_friend(arg_matches, config, key_reader)
}

fn add_friend(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
    key_reader: RefCell<Box<dyn KeyReader>>,
) -> Result<(), EncItError> {
    let key = get_key(arg_matches, key_reader)?;
    // check if is a valid public key
    let _ = key.public_key()?;
    let hex_hey = EncItPEM::Hex(key.hex_pem()?);

    let friend_name = arg_matches.value_of("name").unwrap();

    if friend_name.contains(' ') {
        return Err(InvalidCommand("friend name could not have spaces".into()));
    }
    config.add_friend(friend_name, &hex_hey)?.save()
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::rc::Rc;
    use std::sync::Arc;

    use mockall::predicate::eq;
    use openssl::rsa::Rsa;

    use crate::config::MockEncItConfig;
    use crate::{add_cmd, EncItConfig, EncItPEM};

    use super::*;

    #[test]
    fn add_friend_invalid_name() {
        let friend_name = "friend with space 1";
        let cmd = add_cmd("friend");
        let matches =
            cmd.get_matches_from(vec!["friend", "--name", friend_name, "--format", "hex-pem"]);
        let priv_key = Rsa::generate(2048).unwrap();
        let pub_key_hex = Arc::new(priv_key.public_key_to_pem().map(hex::encode).unwrap());
        let hex_key = Box::leak(Box::new(pub_key_hex));
        let key_reader: RefCell<Box<dyn KeyReader>> = RefCell::new(Box::new(hex_key.as_bytes()));
        let cfg_mock: Rc<dyn EncItConfig> = Rc::new(MockEncItConfig::new());
        let result = add_friend(&matches, cfg_mock, key_reader);
        assert!(result.is_err());
    }

    #[test]
    fn add_friend_hex() {
        let friend_name = "friend-hex-1";
        let cmd = add_cmd("friend");
        let matches =
            cmd.get_matches_from(vec!["friend", "--name", friend_name, "--format", "hex-pem"]);
        let priv_key = Rsa::generate(2048).unwrap();
        let pub_key_hex = Arc::new(priv_key.public_key_to_pem().map(hex::encode).unwrap());
        let expected_encit_pem = EncItPEM::Hex(pub_key_hex.to_string());
        let hex_key = Box::leak(Box::new(pub_key_hex));
        let key_reader: RefCell<Box<dyn KeyReader>> = RefCell::new(Box::new(hex_key.as_bytes()));
        check_add_friend(friend_name, &matches, key_reader, expected_encit_pem);
    }

    #[test]
    fn add_friend_base64() {
        let friend_name = "friend-base64-1";
        let cmd = add_cmd("friend");
        let matches = cmd.get_matches_from(vec![
            "friend",
            "--name",
            friend_name,
            "--format",
            "base64-pem",
        ]);
        let priv_key = Rsa::generate(2048).unwrap();
        let pub_key_hex = Arc::new(priv_key.public_key_to_pem().map(hex::encode).unwrap());
        let pub_key_base64 = Arc::new(priv_key.public_key_to_pem().map(base64::encode).unwrap());
        let expected_encit_pem = EncItPEM::Hex(pub_key_hex.to_string());
        let base64_key = Box::leak(Box::new(pub_key_base64));
        let key_reader: RefCell<Box<dyn KeyReader>> = RefCell::new(Box::new(base64_key.as_bytes()));
        check_add_friend(friend_name, &matches, key_reader, expected_encit_pem);
    }

    fn check_add_friend(
        friend_name: &'static str,
        matches: &ArgMatches,
        key_reader: RefCell<Box<dyn KeyReader>>,
        expected_encit_pem: EncItPEM,
    ) {
        let mut cfg_mock = MockEncItConfig::new();
        cfg_mock
            .expect_add_friend()
            .with(eq(friend_name), eq(expected_encit_pem))
            .returning(|_, _| {
                let mut new_cfg = MockEncItConfig::new();
                new_cfg.expect_save().returning(|| Ok(()));
                Ok(Box::new(new_cfg))
            });

        let cfg: Rc<dyn EncItConfig> = Rc::new(cfg_mock);
        add_friend(matches, cfg, key_reader).expect("add friend in error");
    }

    impl KeyReader for &'static [u8] {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

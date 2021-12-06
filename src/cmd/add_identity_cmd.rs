use std::cell::RefCell;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};

use crate::cmd::add_cmd::get_key;
use crate::cmd::reader::{get_file_reader, EncItFileReader};
use crate::{add_cmd, EncItConfig, EncItError, EncItPEM, InvalidCommand};

pub fn add_identity_cmd<'a>() -> App<'a, 'a> {
    add_cmd("identity").arg(
        Arg::with_name("passphrase")
            .long("passphrase")
            .short("p")
            .takes_value(true),
    )
}

pub fn add_identity_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let reader = RefCell::new(get_file_reader(arg_matches, "key-file")?);
    add_identity(arg_matches, config, reader)
}

fn add_identity(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
    reader: RefCell<Box<dyn EncItFileReader>>,
) -> Result<(), EncItError> {
    let key = get_key(arg_matches, reader)?;
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    use openssl::rsa::Rsa;
    use openssl::symm::Cipher;

    use crate::config::MockEncItConfig;
    use crate::{EncItConfig, EncItPEM};

    use super::*;

    #[test]
    fn add_identity_invalid_name() {
        let friend_name = "identity with space 1";
        let cmd = add_identity_cmd();
        let matches = cmd.get_matches_from(vec![
            "identity",
            "--name",
            friend_name,
            "--format",
            "hex-pem",
        ]);
        let priv_key = Rsa::generate(2048).unwrap();
        let priv_key_hex = Arc::new(priv_key.private_key_to_pem().map(hex::encode).unwrap());
        let hex_key = Box::leak(Box::new(priv_key_hex));
        let key_reader: RefCell<Box<dyn EncItFileReader>> =
            RefCell::new(Box::new(hex_key.as_bytes()));
        let cfg_mock: Rc<dyn EncItConfig> = Rc::new(MockEncItConfig::new());
        let result = add_identity(&matches, cfg_mock, key_reader);
        assert!(result.is_err());
    }

    #[test]
    fn add_identity_hex() {
        let identity_name = "identity-hex-1";
        let cmd = add_identity_cmd();
        let matches = cmd.get_matches_from(vec![
            "identity",
            "--name",
            identity_name,
            "--format",
            "hex-pem",
        ]);
        let priv_key = Rsa::generate(2048).unwrap();
        let priv_key_hex = Arc::new(priv_key.private_key_to_pem().map(hex::encode).unwrap());
        let expected_encit_pem = EncItPEM::Hex(priv_key_hex.to_string());
        let hex_key = Box::leak(Box::new(priv_key_hex));
        let key_reader: RefCell<Box<dyn EncItFileReader>> =
            RefCell::new(Box::new(hex_key.as_bytes()));
        check_add_identity(
            identity_name,
            &matches,
            key_reader,
            expected_encit_pem,
            None,
        );
    }

    #[test]
    fn add_identity_base64_passphrase() {
        let identity_name = "identity-base64-1";
        let cmd = add_identity_cmd();
        let priv_key_password = "identity-password";
        let matches = cmd.get_matches_from(vec![
            "identity",
            "--name",
            identity_name,
            "--format",
            "base64-pem",
            "--passphrase",
            priv_key_password,
        ]);
        let priv_key = Rsa::generate(2048).unwrap();
        let priv_key_with_password = Rc::new(
            priv_key
                .private_key_to_pem_passphrase(Cipher::aes_128_cbc(), priv_key_password.as_bytes())
                .unwrap(),
        );
        let priv_key_hex = Arc::new(hex::encode(priv_key_with_password.to_vec()));
        let priv_key_base64 = Arc::new(base64::encode(priv_key_with_password.to_vec()));
        let expected_encit_pem = EncItPEM::Hex(priv_key_hex.to_string());
        let base64_key = Box::leak(Box::new(priv_key_base64));
        let key_reader: RefCell<Box<dyn EncItFileReader>> =
            RefCell::new(Box::new(base64_key.as_bytes()));
        check_add_identity(
            identity_name,
            &matches,
            key_reader,
            expected_encit_pem,
            Some(priv_key_password),
        );
    }

    fn check_add_identity(
        identity_name: &'static str,
        matches: &ArgMatches,
        key_reader: RefCell<Box<dyn EncItFileReader>>,
        private_key: EncItPEM,
        passphrase: Option<&'static str>,
    ) {
        let mut cfg_mock = MockEncItConfig::new();
        cfg_mock
            .expect_add_identity()
            .withf(
                move |identity_name_param, private_key_param, passphrase_param| {
                    identity_name_param == identity_name
                        && *private_key_param == private_key
                        && passphrase_param.as_deref() == passphrase
                },
            )
            .returning(|_, _, _| {
                let mut new_cfg = MockEncItConfig::new();
                new_cfg.expect_save().returning(|| Ok(()));
                Ok(Box::new(new_cfg))
            });

        let cfg: Rc<dyn EncItConfig> = Rc::new(cfg_mock);
        add_identity(matches, cfg, key_reader).expect("add friend in error");
    }
}

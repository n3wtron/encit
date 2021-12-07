use crate::{EncItConfig, EncItError, EncItPEM};
use clap::{App, Arg, ArgMatches, SubCommand};
use openssl::rsa::Rsa;
use std::rc::Rc;

pub fn new_identity_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("identity").arg(Arg::with_name("name").takes_value(true).required(true))
}

pub fn new_identity_exec(
    arg_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity_name = arg_matches.value_of("name").unwrap();
    let key = Rsa::generate(2048)?;
    let key = EncItPEM::Hex(hex::encode(key.private_key_to_pem()?));
    config.add_identity(identity_name, &key, None)?.save()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MockEncItConfig;
    use crate::EncItError;

    #[test]
    fn new_identity_test() -> Result<(), EncItError> {
        let cmd = new_identity_cmd();
        let identity_name = "new-identity-1";
        let cmd_matches = cmd.get_matches_from(vec!["identity", identity_name]);
        let mut cfg_mock = MockEncItConfig::new();
        cfg_mock
            .expect_add_identity()
            .withf(move |identity_name_param, _, passphrase| {
                identity_name_param == identity_name && passphrase.is_none()
            })
            .returning(|_, _, _| {
                let mut new_cfg = MockEncItConfig::new();
                new_cfg.expect_save().returning(|| Ok(()));
                Ok(Box::new(new_cfg))
            });
        new_identity_exec(&cmd_matches, Rc::new(cfg_mock))?;

        Ok(())
    }
}

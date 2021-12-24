use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};

use std::sync::Arc;

pub fn new_identity_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("identity").arg(Arg::with_name("name").takes_value(true).required(true))
}

pub fn new_identity_exec(
    arg_matches: &ArgMatches,
    config: Arc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let identity_name = arg_matches.value_of("name").unwrap();
    config.new_identity(identity_name)?.save()
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
            .expect_new_identity()
            .withf(move |identity_name_param| identity_name_param == identity_name)
            .returning(|_| {
                let mut new_cfg = MockEncItConfig::new();
                new_cfg.expect_save().returning(|| Ok(()));
                Ok(Arc::new(new_cfg))
            });
        new_identity_exec(&cmd_matches, Arc::new(cfg_mock))?;

        Ok(())
    }
}

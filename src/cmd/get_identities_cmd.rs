use crate::{EncItConfig, EncItError};
use clap::{App, ArgMatches, SubCommand};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

pub fn get_identities_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("identities")
}

pub fn get_identities_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let writer = Rc::new(RefCell::new(stdout()));
    get_identities(cmd_matches, config, writer)
}

pub fn get_identities(
    _: &ArgMatches,
    config: Rc<dyn EncItConfig>,
    writer: Rc<RefCell<dyn Write>>,
) -> Result<(), EncItError> {
    let mut writer_mut = writer.borrow_mut();
    for identity in config.identities() {
        writer_mut.write_all(identity.name().as_bytes())?;
        writer_mut.write_all("\n".as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MockEncItConfig;
    use crate::enc::tests::generate_identity;

    #[test]
    fn get_friends_test() -> Result<(), EncItError> {
        let cmd = get_identities_cmd();
        let cmd_matches = cmd.get_matches_from(vec!["identities"]);
        let mut cfg = MockEncItConfig::new();

        let (_, identity1) = generate_identity("identity1", None);
        let (_, identity2) = generate_identity("identity2", None);
        let identity1 = *identity1;
        let identity2 = *identity2;
        let identities = vec![identity1, identity2];
        cfg.expect_identities().with().return_const(identities);
        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));

        get_identities(&cmd_matches, Rc::new(cfg), writer.clone())?;
        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, "identity1\nidentity2\n");
        Ok(())
    }
}

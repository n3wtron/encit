use crate::{EncItConfig, EncItError};
use clap::{App, ArgMatches, SubCommand};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;

pub fn get_friends_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("friends")
}

pub fn get_friends_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    get_friends(cmd_matches, config, Rc::new(RefCell::new(stdout())))
}

fn get_friends(
    _: &ArgMatches,
    config: Rc<dyn EncItConfig>,
    writer: Rc<RefCell<dyn Write>>,
) -> Result<(), EncItError> {
    let mut mut_writer = writer.borrow_mut();
    for friend in config.friends() {
        mut_writer.write_all(friend.name().as_bytes())?;
        mut_writer.write_all("\n".as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MockEncItConfig;
    use crate::enc::tests::generate_friend;

    #[test]
    fn get_friends_test() -> Result<(), EncItError> {
        let cmd = get_friends_cmd();
        let cmd_matches = cmd.get_matches_from(vec!["friends"]);
        let mut cfg = MockEncItConfig::new();

        let (_, friend1) = generate_friend("friend1", None);
        let (_, friend2) = generate_friend("friend2", None);
        let friend1 = *friend1;
        let friend2 = *friend2;
        let friends = vec![friend1, friend2];
        cfg.expect_friends().with().return_const(friends);
        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));

        get_friends(&cmd_matches, Rc::new(cfg), writer.clone())?;
        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, "friend1\nfriend2\n");
        Ok(())
    }
}

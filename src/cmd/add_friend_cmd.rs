use crate::cmd::add_cmd::get_key;
use crate::{EncItConfig, EncItError, EncItPEM, InvalidCommand};
use clap::ArgMatches;
use std::rc::Rc;

pub fn add_friend_exec(
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

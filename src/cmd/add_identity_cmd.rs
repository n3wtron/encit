use crate::cmd::add_cmd::get_key;
use crate::{EncItConfig, EncItError, EncItPEM, InvalidCommand};
use clap::ArgMatches;
use std::rc::Rc;

pub fn add_identity_exec(
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

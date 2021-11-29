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

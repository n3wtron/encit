use crate::cmd::reader::{get_file_reader, EncItFileReader};
use crate::enc::{EncIt, EncItImpl};
use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::cell::RefCell;
use std::io::{stdout, Read, Write};
use std::rc::Rc;

pub fn decrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("decrypt")
        .about("decrypt an encIt message")
        .arg(
            Arg::with_name("identity")
                .long("identity")
                .short("i")
                .takes_value(true)
                .help("Identity name (has to be present in the encit configuration file)"),
        )
        .arg(Arg::with_name("json").long("json"))
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("file to encrypt"),
        )
}

pub fn decrypt_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let enc_it = Rc::new(EncItImpl::new(config));
    let reader = RefCell::new(get_file_reader(cmd_matches, "file")?);
    decrypt(cmd_matches, enc_it, reader)
}

fn decrypt(
    cmd_matches: &ArgMatches,
    enc_it: Rc<dyn EncIt>,
    reader: RefCell<Box<dyn EncItFileReader>>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity");
    let mut encrypted_message = String::new();
    reader.borrow_mut().read_to_string(&mut encrypted_message)?;

    let decrypted_message = enc_it.decrypt(&encrypted_message, identity)?;
    if cmd_matches.is_present("json") {
        println!("{}", serde_json::to_string(&decrypted_message).unwrap());
    } else {
        let payload = base64::decode(decrypted_message.payload())?;
        stdout().write_all(payload.as_slice())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enc::{EncItMessage, MockEncIt};
    use crate::EncItError;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn auto_decrypt() -> Result<(), EncItError> {
        let cmd = decrypt_cmd();
        let cmd_matches = cmd.get_matches_from(vec!["decrypt"]);

        let jwe_message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let jwe_message2 = Box::leak(Box::new(jwe_message.clone()));

        let mut encit_mock = MockEncIt::new();
        encit_mock
            .expect_decrypt()
            .withf(move |jwe_param, opt_identity| jwe_param == jwe_message && *opt_identity == None)
            .returning(|_, _| {
                let base64_payload = base64::encode("payload");
                Ok(EncItMessage::new(
                    "sender".to_string(),
                    "receiver".to_string(),
                    None,
                    base64_payload,
                    true,
                ))
            });
        let rc_encit_mock = Rc::new(encit_mock);
        let reader = RefCell::new(Box::new(jwe_message2.as_bytes()));
        decrypt(&cmd_matches, rc_encit_mock, reader)?;

        Ok(())
    }
}

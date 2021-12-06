use crate::cmd::reader::{get_file_reader, EncItFileReader};
use crate::enc::{EncIt, EncItImpl};
use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::debug;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

pub fn encrypt_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("encrypt")
        .about("encrypt a file/text")
        .arg(
            Arg::with_name("identity")
                .long("identity")
                .short("i")
                .required(true)
                .takes_value(true)
                .help("Identity name (has to be present in the encit configuration file)"),
        )
        .arg(
            Arg::with_name("friend")
                .long("friend")
                .short("f")
                .required(true)
                .takes_value(true)
                .help("Friend name (has to be present in the encit configuration file)"),
        )
        .arg(
            Arg::with_name("subject")
                .long("subject")
                .short("s")
                .takes_value(true)
                .help("Message subject"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .help("file to encrypt"),
        )
}

pub fn encrypt_exec(
    cmd_matches: &ArgMatches,
    config: Rc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let enc_it = Rc::new(EncItImpl::new(config));
    let reader = RefCell::new(get_file_reader(cmd_matches, "file")?);
    encrypt(cmd_matches, enc_it, reader)
}

fn encrypt(
    cmd_matches: &ArgMatches,
    enc_it: Rc<dyn EncIt>,
    reader: RefCell<Box<dyn EncItFileReader>>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity").unwrap();
    let friend = cmd_matches.value_of("friend").unwrap();
    let subject = cmd_matches.value_of("subject");
    let mut message = Vec::new();

    reader.borrow_mut().read_to_end(&mut message)?;

    debug!("message: {:?}", &message);
    let b64_message = base64::encode(message);

    let enc_message = enc_it.encrypt(identity, friend, subject, &b64_message)?;
    println!("{}", enc_message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enc::MockEncIt;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn encrypt_stdin() -> Result<(), EncItError> {
        let identity_name = "identity1";
        let friend_name = "friend1";
        let subject = "subject1";
        let cmd = encrypt_cmd();
        let cmd_matches = cmd.get_matches_from(vec![
            "encrypt",
            "--identity",
            identity_name,
            "--friend",
            friend_name,
            "--subject",
            subject,
        ]);
        let message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let message2 = Box::leak(Box::new(message.clone()));
        let message_bytes = message.as_bytes();
        let b64_message = base64::encode(message_bytes);

        let mut encit_mock = MockEncIt::new();
        encit_mock
            .expect_encrypt()
            .withf(
                move |identity_name_param, friend_name_param, subject_param, message_param| {
                    identity_name_param == identity_name
                        && friend_name_param == friend_name
                        && *subject_param == Some(subject)
                        && message_param == b64_message
                },
            )
            .returning(|_, _, _, _| Ok(String::from("enc")));
        let rc_encit_mock = Rc::new(encit_mock);
        let message = RefCell::new(Box::new(message2.as_bytes()));

        encrypt(&cmd_matches, rc_encit_mock, message)?;
        Ok(())
    }
}

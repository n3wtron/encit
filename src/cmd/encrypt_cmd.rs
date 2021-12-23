use crate::cmd::reader::{get_file_reader, EncItFileReader, ReaderType};
use crate::enc::{EncIt, EncItImpl, MessageType};
use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::debug;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

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
    config: Arc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let enc_it = Rc::new(EncItImpl::new(config));
    let mut reader = get_file_reader(cmd_matches, "file")?;
    let writer = Rc::new(RefCell::new(stdout()));
    encrypt(cmd_matches, enc_it, &mut reader, writer)
}

fn encrypt(
    cmd_matches: &ArgMatches,
    enc_it: Rc<dyn EncIt>,
    reader: &mut EncItFileReader,
    writer: Rc<RefCell<dyn Write>>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity").unwrap();
    let friend = cmd_matches.value_of("friend").unwrap();
    let subject = cmd_matches.value_of("subject").or_else(|| {
        cmd_matches
            .value_of("file")
            .map(|fl_path| Path::new(fl_path))
            .and_then(|p| p.file_name())
            .and_then(|fl_name| fl_name.to_str())
    });
    let mut message = Vec::new();

    reader.read_to_end(&mut message)?;

    debug!("message: {:?}", &message);
    let b64_message = base64::encode(message);
    let message_type = if *reader.reader_type() == ReaderType::Stdin {
        MessageType::PlainText
    } else {
        MessageType::File
    };

    let enc_message = enc_it.encrypt(identity, friend, subject, message_type, &b64_message)?;
    writer
        .borrow_mut()
        .write_all(enc_message.as_bytes())
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::reader::ReaderType;
    use crate::enc::MockEncIt;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn encrypt_stdin() -> Result<(), EncItError> {
        encrypt_test(false)
    }

    #[test]
    fn encrypt_file() -> Result<(), EncItError> {
        encrypt_test(true)
    }

    fn encrypt_test(from_file: bool) -> Result<(), EncItError> {
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
        let (reader_type, message_type) = if from_file {
            (ReaderType::File, MessageType::File)
        } else {
            (ReaderType::Stdin, MessageType::PlainText)
        };
        let mut in_message = EncItFileReader::new(Box::new(message2.as_bytes()), reader_type);
        encit_mock
            .expect_encrypt()
            .withf(
                move |identity_name_param,
                      friend_name_param,
                      subject_param,
                      message_type_param,
                      message_param| {
                    identity_name_param == identity_name
                        && friend_name_param == friend_name
                        && *subject_param == Some(subject)
                        && *message_type_param == message_type
                        && message_param == b64_message
                },
            )
            .returning(|_, _, _, _, _| Ok(String::from("fake enc")));
        let rc_encit_mock = Rc::new(encit_mock);
        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        encrypt(&cmd_matches, rc_encit_mock, &mut in_message, writer.clone())?;
        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, "fake enc");
        Ok(())
    }
}

use crate::cmd::reader::{get_file_reader, EncItFileReader};
use crate::enc::{EncIt, EncItImpl};
use crate::{EncItConfig, EncItError};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;
use std::sync::Arc;

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
    config: Arc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let enc_it = Rc::new(EncItImpl::new(config));
    let mut reader = get_file_reader(cmd_matches, "file")?;
    let rc_stdout: Rc<RefCell<dyn Write>> = Rc::new(RefCell::new(stdout()));
    decrypt(cmd_matches, enc_it, &mut reader, rc_stdout)
}

fn decrypt(
    cmd_matches: &ArgMatches,
    enc_it: Rc<dyn EncIt>,
    reader: &mut EncItFileReader,
    writer: Rc<RefCell<dyn Write>>,
) -> Result<(), EncItError> {
    let identity = cmd_matches.value_of("identity");
    let mut encrypted_message = String::new();
    reader.read_to_string(&mut encrypted_message)?;

    let decrypted_message = enc_it.decrypt(&encrypted_message, identity)?;
    let mut writer = writer.borrow_mut();
    if cmd_matches.is_present("json") {
        writer.write_all(serde_json::to_vec(&decrypted_message)?.as_slice())?;
    } else {
        let payload = base64::decode(decrypted_message.payload())?;
        writer.write_all(payload.as_slice())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::reader::ReaderType;
    use crate::enc::{EncItMessage, MessageType, MockEncIt};
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
                    MessageType::PlainText,
                    base64_payload,
                    true,
                ))
            });
        let rc_encit_mock = Rc::new(encit_mock);
        let mut reader = EncItFileReader::new(Box::new(jwe_message2.as_bytes()), ReaderType::Stdin);

        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        decrypt(&cmd_matches, rc_encit_mock, &mut reader, writer.clone())?;

        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, "payload");
        Ok(())
    }

    #[test]
    fn identity_decrypt() -> Result<(), EncItError> {
        let cmd = decrypt_cmd();
        let cmd_matches = cmd.get_matches_from(vec!["decrypt", "--identity", "identity-1"]);

        let jwe_message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let jwe_message2 = Box::leak(Box::new(jwe_message.clone()));

        let mut encit_mock = MockEncIt::new();
        encit_mock
            .expect_decrypt()
            .withf(move |jwe_param, opt_identity| {
                jwe_param == jwe_message && *opt_identity == Some("identity-1")
            })
            .returning(|_, _| {
                let base64_payload = base64::encode("payload");
                Ok(EncItMessage::new(
                    "sender".to_string(),
                    "receiver".to_string(),
                    None,
                    MessageType::PlainText,
                    base64_payload,
                    true,
                ))
            });
        let rc_encit_mock = Rc::new(encit_mock);
        let mut reader = EncItFileReader::new(Box::new(jwe_message2.as_bytes()), ReaderType::Stdin);

        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        decrypt(&cmd_matches, rc_encit_mock, &mut reader, writer.clone())?;

        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, "payload");
        Ok(())
    }

    #[test]
    fn auto_decrypt_json() -> Result<(), EncItError> {
        let cmd = decrypt_cmd();
        let cmd_matches = cmd.get_matches_from(vec!["decrypt", "--json"]);

        let jwe_message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let jwe_message2 = Box::leak(Box::new(jwe_message.clone()));

        let base64_payload = base64::encode("payload");
        let expected_message = EncItMessage::new(
            "sender".to_string(),
            "receiver".to_string(),
            None,
            MessageType::PlainText,
            base64_payload,
            true,
        );
        let mock_output = Box::leak(Box::new(expected_message.clone()));
        let json_expected_message = serde_json::to_string(&expected_message)?;
        let mut encit_mock = MockEncIt::new();
        encit_mock
            .expect_decrypt()
            .withf(move |jwe_param, opt_identity| jwe_param == jwe_message && *opt_identity == None)
            .returning(move |_, _| Ok((*mock_output).clone()));
        let rc_encit_mock = Rc::new(encit_mock);
        let mut reader = EncItFileReader::new(Box::new(jwe_message2.as_bytes()), ReaderType::Stdin);

        let writer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        decrypt(&cmd_matches, rc_encit_mock, &mut reader, writer.clone())?;

        let result = String::from_utf8(writer.borrow().to_vec())?;
        assert_eq!(result, json_expected_message);
        Ok(())
    }
}

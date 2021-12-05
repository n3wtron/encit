use crate::{EncItError, EncItPEM};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::any::Any;
use std::cell::RefCell;
use std::fs::File;
use std::io::{stdin, Read, Stdin};

pub fn add_cmd<'a>(name: &str) -> App<'a, 'a> {
    SubCommand::with_name(name)
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .takes_value(true)
                .required(true)
                .possible_values(&["pem", "hex-pem", "base64-pem"]),
        )
        .arg(
            Arg::with_name("key-file")
                .takes_value(true)
                .help("key file"),
        )
}

pub fn get_key(
    arg_matches: &ArgMatches,
    reader: RefCell<Box<dyn KeyReader>>,
) -> Result<EncItPEM, EncItError> {
    let mut key_content = String::new();
    reader.borrow_mut().read_to_string(&mut key_content)?;
    let format = arg_matches.value_of("format").unwrap();
    match format {
        "pem" => Ok(EncItPEM::Pem(key_content)),
        "hex-pem" => Ok(EncItPEM::Hex(key_content)),
        "base64-pem" => Ok(EncItPEM::Base64(key_content)),
        _ => Err(EncItError::InvalidCommand(String::new())),
    }
}

pub trait KeyReader: Read {
    fn as_any(&self) -> &dyn Any;
}

impl KeyReader for Stdin {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl KeyReader for File {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn get_key_reader(arg_matches: &ArgMatches) -> Result<Box<dyn KeyReader>, EncItError> {
    if let Some(file_path) = arg_matches.value_of("key-file") {
        println!("file!1");
        let fl: Box<dyn KeyReader> = Box::new(File::open(file_path)?);
        Ok(fl)
    } else {
        println!("stdin");
        let stdin: Box<dyn KeyReader> = Box::new(stdin());
        Ok(stdin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EncItError;
    use std::io::Stdin;
    use tempfile::NamedTempFile;

    #[test]
    fn get_key_reader_stdin() -> Result<(), EncItError> {
        let cmd = add_cmd("test");
        let matches =
            cmd.get_matches_from(vec!["test", "--name", "test", "--format", "base64-pem"]);
        let reader = get_key_reader(&matches)?;
        let file = reader.as_any().downcast_ref::<Stdin>();
        assert!(file.is_some());
        Ok(())
    }

    #[test]
    fn get_key_reader_file() -> Result<(), EncItError> {
        let cmd = add_cmd("test");
        let key_file = NamedTempFile::new()?;
        let matches = cmd.get_matches_from(vec![
            "test",
            "--name",
            "test",
            "--format",
            "base64-pem",
            key_file.path().to_str().unwrap(),
        ]);
        let reader = get_key_reader(&matches)?;
        let file = reader.as_any().downcast_ref::<File>();
        assert!(file.is_some());
        Ok(())
    }
}

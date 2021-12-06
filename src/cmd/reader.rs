use crate::EncItError;
use clap::ArgMatches;
use std::any::Any;
use std::fs::File;
use std::io::{stdin, Read, Stdin};

pub trait EncItFileReader: Read {
    fn as_any(&self) -> &dyn Any;
}

impl EncItFileReader for Stdin {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EncItFileReader for File {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn get_file_reader(
    arg_matches: &ArgMatches,
    param_name: &str,
) -> Result<Box<dyn EncItFileReader>, EncItError> {
    if let Some(file_path) = arg_matches.value_of(param_name) {
        let fl: Box<dyn EncItFileReader> = Box::new(File::open(file_path)?);
        Ok(fl)
    } else {
        let stdin: Box<dyn EncItFileReader> = Box::new(stdin());
        Ok(stdin)
    }
}

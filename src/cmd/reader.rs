use crate::EncItError;
use clap::ArgMatches;
use std::fs::File;
use std::io;
use std::io::{stdin, Read};

#[derive(PartialEq, Debug)]
pub enum ReaderType {
    Stdin,
    File,
}

pub struct EncItFileReader {
    reader: Box<dyn Read>,
    reader_type: ReaderType,
}

impl EncItFileReader {
    pub fn new(reader: Box<dyn Read>, reader_type: ReaderType) -> Self {
        EncItFileReader {
            reader,
            reader_type,
        }
    }
    pub fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.reader.read_to_string(buf)
    }
    pub fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.reader.read_to_end(buf)
    }
    pub fn reader_type(&self) -> &ReaderType {
        &self.reader_type
    }
}

pub fn get_file_reader(
    arg_matches: &ArgMatches,
    param_name: &str,
) -> Result<EncItFileReader, EncItError> {
    if let Some(file_path) = arg_matches.value_of(param_name) {
        Ok(EncItFileReader::new(
            Box::new(File::open(file_path)?),
            ReaderType::File,
        ))
    } else {
        Ok(EncItFileReader::new(Box::new(stdin()), ReaderType::Stdin))
    }
}

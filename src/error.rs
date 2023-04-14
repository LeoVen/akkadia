use std::{
    fmt::Display,
    process::{ExitCode, Termination},
};

use log::SetLoggerError;

#[derive(Debug)]
pub struct AkkadiaError {
    pub code: u8,
    pub errors: Vec<Error>,
}

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for AkkadiaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .errors
            .iter()
            .rev()
            .fold(String::new(), |acc, e| acc + &e.to_string() + " ");
        write!(f, "error {}: {}", self.code, message)
    }
}

impl Termination for AkkadiaError {
    fn report(self) -> std::process::ExitCode {
        eprintln!("{}", self);
        ExitCode::from(self.code)
    }
}

impl From<&str> for AkkadiaError {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<String> for AkkadiaError {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<SetLoggerError> for AkkadiaError {
    fn from(value: SetLoggerError) -> Self {
        todo!()
    }
}

impl From<clap::Error> for AkkadiaError {
    fn from(value: clap::Error) -> Self {
        todo!()
    }
}

impl AkkadiaError {
    pub fn chain(&mut self, error: Error) {
        todo!()
    }
}

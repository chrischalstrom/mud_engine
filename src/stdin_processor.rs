use std::io;

use crate::command_processor::CommandProcessor;

pub struct StdinProcessor {}

impl CommandProcessor for StdinProcessor {
    fn accept_input(&self) -> Result<String, io::Error> {
        let mut buf = String::new();

        match io::stdin().read_line(&mut buf) {
            Ok(_n) => Ok(buf),
            Err(error) => Err(error),
        }
    }
}

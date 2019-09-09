use std::io;
use std::sync::mpsc;

// probably should have one thread handling this and another thread for updating the game
// TODO: log message handler..way to handle output from the game
// - printing to stdout vs e.g. ws
//   - needs to be able to respond to events (combat)
pub trait CommandProcessor {
    fn accept_input(&self) -> Result<String, io::Error>;
}

pub fn listen(processor: &CommandProcessor, tx: mpsc::Sender<String>) {
    loop {
        match processor.accept_input() {
            Ok(input) => {
                match tx.send(input) {
                    Ok(_m) => (),
                    Err(send_err) => println!("could not transmit: {}", send_err),
                };
            }
            Err(err) => println!("err {}", err),
        }
    }
}

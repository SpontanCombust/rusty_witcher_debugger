use std::{sync::mpsc::Sender, io::{self, BufRead}};

/// Waits for user to press Enter and then sends the signal using sender
pub(crate) fn input_waiter(sender: Sender<()>) {
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    sender.send(()).unwrap();
}
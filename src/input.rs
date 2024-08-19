use std::io;
use std::io::Write;

pub(crate) fn get() -> String {
    let mut input: String = String::new();

    print!("|> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input
}

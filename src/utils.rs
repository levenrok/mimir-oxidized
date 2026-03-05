use std::io::Write;

pub enum Kind {
    SUCCESS,
    INFO,
    WARNING,
    ERROR,
}

pub fn pretty_print<W: Write>(writer: &mut W, msg: &str, kind: Kind) {
    let colour = match kind {
        Kind::SUCCESS => "\x1b[32m",
        Kind::INFO => "\x1b[34m",
        Kind::WARNING => "\x1b[33m",
        Kind::ERROR => "\x1b[31m",
    };

    write!(writer, "{}┌", colour).unwrap();
    for _i in 0..msg.len() {
        write!(writer, "─").unwrap();
    }
    writeln!(writer, "┐\x1b[0m").unwrap();
    writeln!(writer, "{}│{}│\x1b[0m", colour, msg).unwrap();
    write!(writer, "{}└", colour).unwrap();
    for _i in 0..msg.len() {
        write!(writer, "─").unwrap();
    }
    writeln!(writer, "┘\x1b[0m").unwrap();
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_success() {
        let mut buffer = Vec::new();
        pretty_print(&mut buffer, "This is a success message!", Kind::SUCCESS);

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output,
            "\x1b[32m┌──────────────────────────┐\x1b[0m\n\
             \x1b[32m│This is a success message!│\x1b[0m\n\
             \x1b[32m└──────────────────────────┘\x1b[0m\n",
        );
    }

    #[test]
    fn pretty_print_info() {
        let mut buffer = Vec::new();
        pretty_print(&mut buffer, "This is a info message!", Kind::INFO);

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output,
            "\x1b[34m┌───────────────────────┐\x1b[0m\n\
             \x1b[34m│This is a info message!│\x1b[0m\n\
             \x1b[34m└───────────────────────┘\x1b[0m\n",
        );
    }

    #[test]
    fn pretty_print_warning() {
        let mut buffer = Vec::new();
        pretty_print(&mut buffer, "This is a warning message!", Kind::WARNING);

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output,
            "\x1b[33m┌──────────────────────────┐\x1b[0m\n\
             \x1b[33m│This is a warning message!│\x1b[0m\n\
             \x1b[33m└──────────────────────────┘\x1b[0m\n",
        );
    }

    #[test]
    fn pretty_print_error() {
        let mut buffer = Vec::new();
        pretty_print(&mut buffer, "This is a error message!", Kind::ERROR);

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output,
            "\x1b[31m┌────────────────────────┐\x1b[0m\n\
             \x1b[31m│This is a error message!│\x1b[0m\n\
             \x1b[31m└────────────────────────┘\x1b[0m\n",
        );
    }
}

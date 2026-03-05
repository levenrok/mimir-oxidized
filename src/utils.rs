pub fn print_fmt_err(err: &str) {
    eprint!("\x1b[31m┌");
    for _i in 0..err.len() {
        eprint!("─");
    }
    eprintln!("┐\x1b[0m");
    eprintln!("\x1b[31m│{}│\x1b[0m", err);
    eprint!("\x1b[31m└");
    for _i in 0..err.len() {
        eprint!("─");
    }
    eprintln!("┘\x1b[0m");
}

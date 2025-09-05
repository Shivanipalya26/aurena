use crossterm::terminal::size;

pub fn get_terminal_size() -> Option<(u32, u32)> {
    match size() {
        Ok((cols, rows)) => Some((cols as u32 * 2, rows as u32 * 6)),
        Err(_) => None,
    }
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

pub fn flush_display() -> std::io::Result<()> {
    use std::io::Write;
    std::io::stdout().flush()
}
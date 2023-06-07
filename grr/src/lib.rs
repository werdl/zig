use std::io::{self, Write};
pub fn matchp(content: &str, pattern: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in content.lines() {
        if line.contains(pattern) {
            _=writeln!(handle,"{}",line);
        }
    }
}
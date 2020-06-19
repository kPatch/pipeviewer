use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024; // sizes the buffer, 16 Kilobytes

fn main() {
    if true == true {}
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    dbg!(silent);
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            // returns the number of bytes read
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break, // ignore the error, use placeholder '_'
        };

        dbg!(total_bytes += num_read);
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if dbg!(!silent) {
        eprint!("num_read: {} ", total_bytes);
    }
}

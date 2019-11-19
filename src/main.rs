//! Echoes lines read on stdin to stdout.

use async_std::io;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut line = String::new();

    loop {
        // Read a line from stdin.
        let n = stdin.read_line(&mut line).await?;

        // If this is the end of stdin, return.
        if n == 0 {
            return Ok(());
        }

        // Write the line to stdout.
        stdout.write_all(line.as_bytes()).await?;
        stdout.flush().await?;
        line.clear();
    }
}
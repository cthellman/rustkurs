/*  ex13a_bufwriter  */
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::create("utfil.txt")?;
    let mut writer = std::io::BufWriter::new(file);
    for line in std::io::stdin().lock().lines() {
	let line = line?;
	writer.write_all(line.as_bytes())?;
	writer.write_all("\r\n".as_bytes())?;
    }
    Ok(())
}






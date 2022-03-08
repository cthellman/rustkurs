/*  ex11a_generisk  */
use std::io::{Write, BufRead, Result, stdin};

fn spara(rad: &str, fh: &mut dyn Write) -> Result<()> {
    fh.write_all(rad.as_bytes())?;
    fh.write_all("\r\n".as_bytes())?;
    fh.flush()
}

fn main() -> Result<()> {
    let mut fh = std::fs::File::create("utfil.txt")?;
    for rad in stdin().lock().lines() {
	spara(&rad?, &mut fh)?;
    }
    fh.sync_all()
}






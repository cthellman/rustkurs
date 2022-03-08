/*  ex12a_generisk2  */
use std::io::{Write, BufRead, Result, stdin};

fn spara(rad: &str, fh: &mut dyn Write) -> Result<()> {
    fh.write_all(rad.as_bytes())?;
    fh.write_all("\r\n".as_bytes())?;
    fh.flush()
}

fn main() -> Result<()> {
    let mut v = Vec::<u8>::new();
    for rad in stdin().lock().lines() {
	spara(&rad?, &mut v)?;
    }
    let s = std::str::from_utf8(&v);
    print!("{}", s.unwrap());
    Ok(())
}






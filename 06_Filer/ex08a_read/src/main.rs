/*  ex08a_read  */
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut fh = std::fs::File::open("scores.txt")?;
    let mut buf = [0u8; 16];
    loop {
	let n = fh.read(&mut buf)?;
	if n == 0 { break; }
	match std::str::from_utf8(&buf[..n]) {
	    Ok(s) => println!("Läste <{}>", s),
	    Err(_) => println!("Läste {:?}", &buf[..n]),
	}
    }
    Ok(())
}






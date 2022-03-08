/*  ex10a_utfil  */
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::create("utfil.txt")?;
    writeln!(file, "Hej, hopp!")?;
    file.sync_all()
}







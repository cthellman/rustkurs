/*  ex14a_klient  */
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let klient = std::net::TcpStream::connect("127.0.0.1:8000")?;
    let reader = std::io::BufReader::new(klient);
    for line in reader.lines() {
	println!("{}", line?);
    }
    Ok(())
}






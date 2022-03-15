/*  ex15a_server  */
use std::io::Write;

fn main() -> std::io::Result<()> {
    let s = "Bill 12\r\nSteve 19\r\nLinus 33\r\nKen 27\r\n";
    let server = std::net::TcpListener::bind("127.0.0.1:8000")?;
    loop {
        if let Ok((mut klient, adress)) = server.accept() {
            println!("Koppel från {:?}", adress);
	    klient.write_all(s.as_bytes())?;
        }
    }
}






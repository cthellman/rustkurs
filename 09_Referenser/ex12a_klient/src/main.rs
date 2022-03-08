/*  ex12a_klient  */
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut socket = std::net::TcpStream::connect("127.1:8000")?;

    let socket_dup = socket.try_clone()?;
    let reader = std::io::BufReader::new(socket_dup);
    let mut from_server = reader.lines();

    'huvud: loop {
	print!("Kommando: ");
	std::io::stdout().flush()?;
	let mut kommando = String::new();
	std::io::stdin().read_line(&mut kommando)?;
	if kommando.trim().len() == 0 { break; }
	socket.write_all(kommando.as_bytes())?;
	socket.flush()?;
    
	while let Some(rad) = from_server.next() {
	    let rad = rad?;
	    if rad.len() == 0 {
		continue 'huvud;
	    }
	    println!("{}", rad);
	}
	break;
    }
    Ok(())
}



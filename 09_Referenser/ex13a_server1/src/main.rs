/*  ex13a_server1  */

use std::net::{TcpListener,TcpStream,SocketAddr};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.1:8000")?;
    while let Ok((klient, adress)) = server.accept() {
	let _ = ny_klient(klient, adress);
    }
    Ok(())
}

use std::io::prelude::*;

fn ny_klient(mut socket: TcpStream, addr: SocketAddr) -> std::io::Result<()> {
    println!("Koppel från {:?}", addr);

    let socket_dup = socket.try_clone()?;
    let mut from_client = std::io::BufReader::new(socket_dup);

    let s = "Bill 12\r\nSteve 19\r\nLinus 33\r\nKen 27\r\n\r\n";
    loop {
	let mut rad = String::new();
	if from_client.read_line(&mut rad)? == 0 {
	    break;
	}
	println!("{} från {:?}", rad.trim(), addr);
	if rad.trim() == "scores" {
	    socket.write_all(s.as_bytes())?;
	} else {
	    socket.write_all(b"unknown command\r\n\r\n")?;
	}
	socket.flush()?;
    }
    println!("Hejdå {:?}", addr);
    Ok(())
}



/*  ex15a_server3  */

use std::io::prelude::*;
use std::net::{TcpListener,TcpStream,SocketAddr};

use std::collections::HashMap;
use std::sync::{Mutex,Arc};

fn ny_klient(socket: TcpStream, addr: SocketAddr,
	     db: Arc<Mutex<HashMap<String, bool>>>) -> bool {
    println!("Koppel från {:?}", addr);
    let res = hantera_klient(socket);
    let ok = match res { Ok(_) => true, Err(_) => false };
    println!("Hejdå {:?}", addr);
    let mut locked_db = db.lock().unwrap();
    locked_db.insert(addr.to_string(), ok);
    ok
}

fn hantera_klient(mut socket: TcpStream) -> std::io::Result<()> {
    let socket_dup = socket.try_clone()?;
    let mut from_client = std::io::BufReader::new(socket_dup);

    let s = "Bill 12\r\nSteve 19\r\nLinus 33\r\nKen 27\r\n\r\n";
    loop {
	let mut rad = String::new();
	if from_client.read_line(&mut rad)? == 0 {
	    break;
	}
	if rad.trim() == "scores" {
	    socket.write_all(s.as_bytes())?;
	} else {
	    socket.write_all(b"unknown command\r\n\r\n")?;
	}
	socket.flush()?;
    }
    Ok(())
}

use std::thread::spawn;

fn main() -> std::io::Result<()> {
    let mut thread_db = HashMap::<String, _>::new();
    let done_db = Arc::new(Mutex::new(HashMap::<String, bool>::new()));
    let server = TcpListener::bind("127.1:8000")?;
    while let Ok((klient, adress)) = server.accept() {
	let db = done_db.clone();
	let thr = spawn(move || ny_klient(klient, adress, db));
	thread_db.insert(adress.to_string(), thr);	
	let mut locked_db = done_db.lock().unwrap();
	for addr in locked_db.keys() {
	    if let Some(th) = thread_db.remove(addr) {
		if let Ok(res) = th.join() {
		    println!("Klient {} resultat {}", addr, res);
		} else {
		    println!("Klient {} panic", addr);
		}
	    }
	}
	locked_db.clear();
    }
    Ok(())
}

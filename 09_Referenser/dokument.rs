/*  ex01e_moved  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for x in lista {
	if x == 5 { lista.remove(2); }
               //   ^^^^^^^^^^^^^^^ value borrowed here after move
    }
    println!("Lista: {:?}", lista);
}
// error[E0382]: borrow of moved value: `lista`



//********************************//


/*  ex02e_borrow  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for &x in &lista {
	if x == 5 { lista.remove(2); }
	       //   ^^^^^^^^^^^^^^^ mutable borrow occurs here
    }
    println!("Lista: {:?}", lista);
}
// error[E0502]: cannot borrow `lista` as mutable because it is also borrowed as immutable



//********************************//


/*  ex03e_mut_borrow  */
fn main() {
    let mut lista = vec![2, 3, 5, 7, 11];
    for x in &mut lista {
	if *x == 5 { lista.remove(2); }
	        //   ^^^^^^^^^^^^^^^ second mutable borrow occurs here
    }
    println!("Lista: {:?}", lista);
}
// error[E0499]: cannot borrow `lista` as mutable more than once at a time




//********************************//


/*  ex04e_ref_local  */
fn main() {
    let poäng_ref;
    {
	let mut poäng = 33;
        poäng_ref = &mut poäng;
    }
    println!("Värdet är {}", *poäng_ref);
}
// error[E0597]: `poäng` does not live long enough




//********************************//


/*  ex05e_ret_local  */
fn p() -> &usize {
    let mut poäng = 33;
    &mut poäng
}

fn main() {
    let poäng_ref = p();
    println!("Värdet är {}", *poäng_ref);
}
// error[E0515]: cannot return reference to local variable `poäng`




//********************************//


/*  ex06a_ref_return  */
fn max(v: &[usize]) -> &usize {
    v.iter().max().unwrap()
}

fn main() {
    let tal: Vec<usize> = vec![12, 19, 33, 27];
    let m = max(&tal);
    println!("Värdet är {}", *m);
}
// Värdet är 33





//********************************//


/*  ex07e_dropped  */
fn max(v: &[usize]) -> &usize {
    v.iter().max().unwrap()
}

fn main() {
    let m;
    {
	let tal: Vec<usize> = vec![12, 19, 33, 27];
	m = max(&tal);
    }
    println!("Värdet är {}", *m);
}
// error[E0597]: `tal` does not live long enough




//********************************//


/*  ex08a_tick  */
fn max<'a>(v: &'a [usize]) -> &'a usize {
    v.iter().max().unwrap()
}

fn main() {
    let tal: Vec<usize> = vec![12, 19, 33, 27];
    let m = max(&tal);
    println!("Värdet är {}", *m);
}
// Värdet är 33





//********************************//


/*  ex09e_missing_tick  */
fn max(v1: &[String], v2: &[String]) -> &String {
    let s1 = v1.iter().max_by_key(|s| s.len()).unwrap();
    let s2 = v2.iter().max_by_key(|s| s.len()).unwrap();
    if s1.len() > s2.len() { s1 }  else { s2 }
}
// error[E0106]: missing lifetime specifier

/*  exa_lifetime  */
fn max<'a>(v1: &'a [String], v2: &'a [String]) -> &'a String {
    let s1 = v1.iter().max_by_key(|s| s.len()).unwrap();
    let s2 = v2.iter().max_by_key(|s| s.len()).unwrap();
    if s1.len() > s2.len() { s1 }  else { s2 }
}

fn main() {
    let n1: Vec<String> = vec!["Bill".to_string(), "Steve".to_string()];
    let n2: Vec<String> = vec!["Linus".to_string(), "Ken".to_string()];
    let m = max(&n1, &n2);
    println!("Värdet är {}", *m);
}
// Värdet är Linus




//********************************//


/*  ex10e_inflexible  */
fn max<'a>(v1: &'a [String], v2: &'a [String]) -> Option<&'a String> {
    if let Some(s1) = v1.iter().max_by_key(|s| s.len()) {	
	v2.iter().find(|s| s.len() >= s1.len())
    } else {
	None
    }
}

fn main() {
    let m;
    let n2: Vec<String> = vec!["Linus".to_string(), "Ken".to_string()];
    {
	let n1: Vec<String> = vec!["Bill".to_string(), "Steve".to_string()];
	m = max(&n1, &n2);
    }
    println!("Värdet är {:?}", m);
}
// error[E0597]: `n1` does not live long enough



//********************************//


/*  ex11a_flexible  */
fn max<'a, 'b>(v1: &'a [String], v2: &'b [String]) -> Option<&'b String> {
    if let Some(s1) = v1.iter().max_by_key(|s| s.len()) {	
	v2.iter().find(|s| s.len() >= s1.len())
    } else {
	None
    }
}

fn main() {
    let m;
    let n2: Vec<String> = vec!["Linus".to_string(), "Ken".to_string()];
    {
	let n1: Vec<String> = vec!["Bill".to_string(), "Steve".to_string()];
	m = max(&n1, &n2);
    }
    println!("Värdet är {:?}", m);
}
// Värdet är Some("Linus")



//********************************//


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




//********************************//


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




//********************************//


/*  ex14a_server2  */

use std::net::{TcpListener,TcpStream,SocketAddr};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.1:8000")?;
    while let Ok((klient, adress)) = server.accept() {
	let _ = std::thread::spawn(
	    move || ny_klient(klient, adress)
	);
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




//********************************//


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


//********************************//



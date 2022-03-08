/*  ex01f_propagera  */

// Om opt är ett Option,
// så betyder opt? följande:
match opt {
    Some(n) => n,
    None => {
	return None;
    }
}



//********************************//


/*  ex02a_hej_hopp  */
fn hej(n: usize) -> Option<usize> {
    if n > 5 { Some(n) } else { None }
}

fn hopp(a: usize, b: usize) -> Option<usize> {
    Some(hej(a)? + hej(b)?)
}

fn main() {
    println!("Summa: {:?}", hopp(3, 9));
    println!("Summa: {:?}", hopp(7, 8));
}
// Summa: None
// Summa: Some(15)



//********************************//


/*  ex03f_result  */

// Om res är ett Result, så
// betyder res? ungefär följande:
match res {
    Ok(n) => n,
    Err(fel) => return Err(fel)
}



//********************************//


/*  ex04a_mata_in_tal  */
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    print!("Skriv ett tal: ");
    std::io::stdout().flush()?;
    for rad in std::io::stdin().lock().lines() {
	if let Ok(n) = rad?.parse::<usize>() {
	    println!("Du skrev {}", n);
	    break;
	}
	print!("Ej tal, försök igen: ");
	std::io::stdout().flush()?;
    }
    Ok(())
}
// Skriv ett tal: ew
// Ej tal, försök igen: 43
// Du skrev 43




//********************************//


/*  ex05a_filsumma  */
use std::io::{BufReader, BufRead};

fn filsumma(fil: &str) -> Result<usize, std::io::Error> {
    let fh = std::fs::File::open(fil)?;
    let reader = BufReader::new(fh);
    let mut summa = 0;
    for rad in reader.lines() {
	if let Ok(n) = rad?.parse::<usize>() {
	    summa += n;
	}
    }
    Ok(summa)
}

// tal.txt:
// 12
// 19
// 33
// 27

fn main() {
    if let Ok(n) = filsumma("tal.txt") {
	println!("Summa: {}", n);
    }
}
// Summa: 91



//********************************//


/*  ex06a_vinnare  */
use std::io::{BufReader, BufRead};

fn vinnare(fil: &str) -> std::io::Result<String> {
    let fh = std::fs::File::open(fil)?;
    let mut max_p = 0;
    let mut ledare = String::new();
    for rad in BufReader::new(fh).lines() {
	let rad = rad?;
	let mut v = rad.split_whitespace();
	if let Some(namn) = v.next() {
	    if let Some(tal) = v.next() {
		if let Ok(n) = tal.parse::<usize>() {
		    if n >= max_p {
			max_p = n;
			ledare = namn.to_string();
		    }
		}
	    }
	}
    }
    Ok(ledare)
}

fn main() {
    if let Ok(namn) = vinnare("scores.txt") {
	println!("Vinnare: {}", namn);
    }
}
// Vinnare: Linus

//********************************//


/*  ex07f_read  */

// https://doc.rust-lang.org/std/io/trait.Read.html
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    // ...
}



//********************************//


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



//********************************//


/*  ex09f_write  */

// https://doc.rust-lang.org/std/io/trait.Write.html
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
    // ...
}



//********************************//


/*  ex10a_utfil  */
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::create("utfil.txt")?;
    writeln!(file, "Hej, hopp!")?;
    file.sync_all()
}




//********************************//


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



//********************************//


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



//********************************//


/*  ex13a_bufwriter  */
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::create("utfil.txt")?;
    let mut writer = std::io::BufWriter::new(file);
    for line in std::io::stdin().lock().lines() {
	let line = line?;
	writer.write_all(line.as_bytes())?;
	writer.write_all("\r\n".as_bytes())?;
    }
    Ok(())
}



//********************************//


/*  ex14a_klient  */
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let klient = std::net::TcpStream::connect("127.1:8000")?;
    let reader = std::io::BufReader::new(klient);
    for line in reader.lines() {
	println!("{}", line?);
    }
    Ok(())
}



//********************************//


/*  ex15a_server  */
use std::io::Write;

fn main() -> std::io::Result<()> {
    let s = "Bill 12\r\nSteve 19\r\nLinus 33\r\nKen 27\r\n";
    let server = std::net::TcpListener::bind("127.1:8000")?;
    loop {
        if let Ok((mut klient, adress)) = server.accept() {
            println!("Koppel från {:?}", adress);
	    klient.write_all(s.as_bytes())?;
        }
    }
}



//********************************//


/* Övning 601: wc

Skriv funktionen wc som tar ett
filnamn som parameter och som
returnerar en tupel av tre usize.
De tre talen ska vara antalet
rader, antalet ord och antalet
tecken i filen.

 */



//********************************//


/* Övning 602: kopiera

Skriv funktionen kopiera som tar
två filnamn som argument. Alla rader
från första filen som är på formatet
Bill 12
(dvs. en sträng följd av blanktecken
följd av ett icke-negativt heltal)
ska skrivas till den andra filen.

 */

//********************************//



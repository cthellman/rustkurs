/*  ex01a_slumptal  */
fn main() {
    let a: u8 = rand::random();
    let b: u8 = rand::random();
    println!("Slumptal: {} och {}", a, b);
}
// Slumptal: 10 och 206


//********************************//


/*  ex02e_annotation  */
fn main() {
    println!("Tal: {}", rand::random());
}
// error[E0282]: type annotations needed


//********************************//


/*  ex03a_turbofisken  */
fn main() {
    println!("Tal: {}", rand::random::<u8>());
}
// Tal: 36


//********************************//


/*  ex04a_utf8  */
fn main() {
    let namn: &str = "Göran";
    println!("Namn: {} (längd {})", namn, namn.len());
}
// Namn: Göran (längd 6)


//********************************//


/*  ex05a_hej  */
fn hälsa(namn: &str) {
    println!("Hej, {}!", namn);
}

fn main() {
    hälsa("Linus");
    let s = "Ken Thompson".to_string();
    hälsa(&s);
    hälsa(&s[0..3]);
}
// Hej, Linus!
// Hej, Ken Thompson!
// Hej, Ken!


//********************************//


/*  ex06a_multibyte  */
fn main() {
    let s = "Göran";
    println!("{}", &s[0..2]);
}
// panic: not a char boundary


//********************************//


/*  ex07a_enum  */
enum Färg {
    Klöver,
    Ruter,
    Hjärter,
    Spader,
}

fn main() {
    let s = Färg::Hjärter;
    match s {
	Färg::Hjärter | Färg::Ruter => {
	    println!("Rött")
	}
	_ => println!("Svart")
    }
}
// Rött


//********************************//


/*  ex08a_equality  */
#[derive(PartialEq)]
enum Färg {
    Klöver,
    Ruter,
    Hjärter,
    Spader,
}

fn main() {
    let s = Färg::Hjärter;
    if s == Färg::Klöver || s == Färg::Spader {
	println!("Svart kort.");
    } else {
	println!("Rött kort.");
    }
}
// Rött kort.


//********************************//


/*  ex09a_kortlek  */
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use Färg::*;

fn main() {
    let s = Hjärter(5);
    match s {
	Hjärter(n) => println!("Hjärter {}", n),
	Ruter(n) => println!("Ruter {}", n),
	_ => println!("Svart kort."),
    }
}
// Hjärter 5


//********************************//


/*  ex10a_if_let  */
#[derive(PartialEq)]
enum Färg {
    Klöver(u8),
    Ruter(u8),
    Hjärter(u8),
    Spader(u8),
}

use Färg::*;

fn main() {
    let s = Hjärter(5);
    if let Hjärter(n) = s {
	println!("Hjärter {}", n);
    }
}
// Hjärter 5


//********************************//


/*  ex11f_Result  */

// https://doc.rust-lang.org/std/result/index.html
enum Result<T, E> {
    Ok(T),
    Err(E),
}


//********************************//


/*  ex12a_parse  */
fn main() {
    let res: Result<usize, _> = "215".parse();
    match res {
	Ok(n) => println!("Talet är {}", n),
	Err(fel) => println!("Ej tal: {}.", fel),
    }
}
// Talet är 215


//********************************//


/*  ex13a_parse  */
fn main() {
    let tal = String::from("215");
    if let Ok(n) = tal.parse::<usize>() {
	println!("Talet är {}", n);
    } else {
	println!("Ej tal.");
    }
}
// Talet är 215


//********************************//


/*  ex14a_unwrap  */
fn main() {
    let tal = String::from("215");   
    let n = tal.parse::<usize>().unwrap();
    println!("Talet är {}", n);
}
// Talet är 215


//********************************//


/*  ex15a_expect  */
fn main() {
    let tal = String::from("21.5");   
    let n = tal.parse::<usize>().expect("Ej tal");
    println!("Talet är {}", n);
}
// Thread 'main' panicked at 'Ej tal




//********************************//


/*  ex16a_read_line  */
fn main() {
    let mut namn = String::new();
    println!("Vad heter du?");
    std::io::stdin().read_line(&mut namn)
	.expect("Kunde inte läsa från stdin");
    println!("Hej, {}!", namn);
}
// Vad heter du?
// Göran
// Hej, Göran
// !


//********************************//


/*  ex17f_Trait  */

// https://doc.rust-lang.org/std/io/trait.Write.html

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    ...




//********************************//


/*  ex18a_flush  */
use std::io::Write;

fn main() {
    let mut namn = String::new();
    print!("Vad heter du? ");
    std::io::stdout().flush().expect("Misslyckades");
    std::io::stdin().read_line(&mut namn)
	.expect("Kunde inte läsa från stdin");
    println!("Hej, {}!", namn.trim());
}
// Vad heter du? Göran
// Hej, Göran!


//********************************//


/*  ex19a_felhantering  */
use std::io::{stdin, stdout, Write};

fn main() {
    let mut namn = String::new();
    print!("Vad heter du? ");
    stdout().flush().expect("Misslyckades");
    match stdin().read_line(&mut namn) {
	Ok(_n) => println!("Hej, {}!", namn.trim()),
	Err(_fel) => println!("Misslyckades"),
    }
}




//********************************//


/*  ex20a_addera  */
use std::io::{stdin, stdout, Write};

fn main() {
    let a = rand::random::<usize>() % 10 + 1;
    let b = rand::random::<usize>() % 10 + 1;
    loop {
	print!("Vad är {} + {}? ", a, b);
	stdout().flush().expect("Misslyckades");
	let mut svar = String::new();
	match stdin().read_line(&mut svar) {
	    Ok(n) => if n == 0 { break; },
	    Err(_) => {
		print!("Förstår ej, försök igen");
		continue;
	    }
	}
	match svar.trim().parse::<usize>() {
	    Ok(n) => {
		if n == a + b {
		    println!("Rätt!");
		    break;
		}
		print!("Fel, försök igen! ");
	    } Err(_) => {
		print!("Ej tal, försök igen! ");
	    }
	}
    }
}
// Vad är 6 + 4? 11
// Fel, försök igen! Vad är 6 + 4? 10
// Rätt!



//********************************//


/* Övning 301: Gissa tal

Välj ett slumptal mellan 1 och 1000. Låt användaren
gissa vilket talet är. Om gissningen är för liten
eller för stor, så ska programmet säga det och sedan
låta användaren gissa igen. Ifall gissningen är rätt
ska programmet berätta det och sedan avslutas. T.ex.

Gissa tal (1-1000): 500
För litet. Gissa tal (1-1000): kalle
Ej tal! Gissa tal (1-1000): 600
För stort. Gissa tal (1-1000): 510
För stort. Gissa tal (1-1000): 505
Rätt efter 4 gissningar!

*/

use std::io::{stdin, stdout, Write};

fn main() {

}


//********************************//


/* Övning 302:

Kör programmet nedan först kompilerat som debug

   cargo run

och sedan som release:

   cargo run --release

Vad blir skillnaden?
Vad händer om man gör sluttal ännu större?

*/
    
fn talsumma(n: usize) -> usize {
    (1..=n).fold(0, |summa, tal| summa + tal)
}

fn main() {
    let sluttal: usize = 100_000_000;
    println!("Summa: {}", talsumma(sluttal));
}


//********************************//



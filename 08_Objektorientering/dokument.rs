/*  ex01a_modul  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    pub year: u16,
    pub month: u8,  // 1-12
    pub day: u8,  // 1-31
}

// Fil: main.rs
mod datum;

fn main() {
    let idag = datum::Datum {
	year: 2022, month: 3, day: 11
    };
    println!("{idag:?}");
}
// Datum { year: 2022, month: 3, day: 11 }

/*  ex02a_konstruktor  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    year: u16,
    month: u8,  // 1-12
    day: u8,  // 1-31
}

impl Datum {
    pub fn new(year: u16, month: u8, day: u8) -> Option<Datum> {
	if month > 12 || day > 31 ||
	    month == 0 || day == 0 {
	    return None;
	}
	let d = Datum { year, month, day };
	Some(d)
    }
}

// Fil: main.rs
mod datum;

fn main() {
    let idag = datum::Datum::new(2022, 3, 11).unwrap();
    println!("{:?}", idag);
}
// Datum { year: 2022, month: 3, day: 11 }

/*  ex03a_metod  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    year: u16,
    month: u8,  // 1-12
    day: u8,  // 1-31
}

impl Datum {
    pub fn new(year: u16, month: u8, day: u8) -> Option<Datum> {
	if month > 12 || day > 31 ||
	    month == 0 || day == 0 {
	    return None;
	}
	let d = Datum { year, month, day };
	Some(d)
    }

    pub fn skottår(&self) -> bool {
	self.year % 4 == 0 &&
	    (self.year % 100 != 0 ||
	     self.year % 400 != 0)
    }
}



// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 10).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("Skottår 2022: {}", idag.skottår());
    println!("Skottår 2024: {}", sedan.skottår());
}
// Skottår 2022: false
// Skottår 2024: true

/*  ex04a_const  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    year: u16,
    month: u8,  // 1-12
    day: u8,  // 1-31
}

impl Datum {
    const DAGAR: [u8; 13] = [0, 31,28,31,30,31,30,31,31,30,31,30,31];

    pub fn månadslängd(&self) -> u8 {
	if self.month == 2 && self.skottår() {
	    29
	} else {
	    Datum::DAGAR[self.month as usize]
	}
    }

    pub fn new(year: u16, month: u8, day: u8) -> Option<Datum> {
	if month > 12 || day > 31 ||
	    month == 0 || day == 0 {
	    return None;
	}
	let d = Datum { year, month, day };
	if day > d.månadslängd() { return None; }
	Some(d)
    }

    pub fn skottår(&self) -> bool {
	self.year % 4 == 0 &&
	    (self.year % 100 != 0 ||
	     self.year % 400 != 0)
    }
}


// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 10).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("Dagar: {}", idag.månadslängd());
    println!("Dagar: {}", sedan.månadslängd());
}
// Dagar: 31
// Dagar: 29

/*  ex05a_display  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    year: u16,
    month: u8,  // 1-12
    day: u8,  // 1-31
}

impl std::fmt::Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
	   -> std::fmt::Result {
	write!(f, "{} {} {}", self.day,
	       Datum::MÅNAD[self.month as usize], self.year)
    }
}

impl Datum {
    const MÅNAD: [&'static str; 13] = [ "",
	"jan", "feb", "mar", "apr", "maj", "jun",
	"jul", "aug", "sep", "okt", "nov", "dec"
    ];

    pub fn datum(&self) -> String {
	format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    const DAGAR: [u8; 13] = [0, 31,28,31,30,31,30,31,31,30,31,30,31];

    pub fn månadslängd(&self) -> u8 {
	if self.month == 2 && self.skottår() {
	    29
	} else {
	    Datum::DAGAR[self.month as usize]
	}
    }

    pub fn new(year: u16, month: u8, day: u8) -> Option<Datum> {
	if month > 12 || day > 31 ||
	    month == 0 || day == 0 {
	    return None;
	}
	let d = Datum { year, month, day };
	if day > d.månadslängd() { return None; }
	Some(d)
    }

    pub fn skottår(&self) -> bool {
	self.year % 4 == 0 &&
	    (self.year % 100 != 0 ||
	     self.year % 400 != 0)
    }
}


// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let idag = Datum::new(2022, 3, 11).unwrap();
    let sedan = Datum::new(2024, 2, 15).unwrap();
    println!("idag: {}", idag);
    println!("sedan: {}", sedan.datum());
}
// idag: 11 mar 2022
// sedan: 2024-02-15

/*  ex06a_operator  */

// Fil: datum.rs
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Datum {
    year: u16,
    month: u8,  // 1-12
    day: u8,  // 1-31
}

impl std::fmt::Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
	   -> std::fmt::Result {
	write!(f, "{} {} {}", self.day,
	       Datum::MÅNAD[self.month as usize], self.year)
    }
}
impl Datum {

    pub fn dagar_framåt(&mut self, n: u16) {
	let mut d = self.day as u16 + n;
	while d > self.månadslängd() as u16 {
	    d -= self.månadslängd() as u16;
	    if self.month == 12 {
		self.month = 1;
		self.year += 1;
	    } else {
		self.month += 1;
	    }
	}
	self.day = d as u8;
    }

    const MÅNAD: [&'static str; 13] = [ "",
	"jan", "feb", "mar", "apr", "maj", "jun",
	"jul", "aug", "sep", "okt", "nov", "dec"
    ];

    pub fn datum(&self) -> String {
	format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    const DAGAR: [u8; 13] = [0, 31,28,31,30,31,30,31,31,30,31,30,31];

    pub fn månadslängd(&self) -> u8 {
	if self.month == 2 && self.skottår() {
	    29
	} else {
	    Datum::DAGAR[self.month as usize]
	}
    }

    pub fn new(year: u16, month: u8, day: u8) -> Option<Datum> {
	if month > 12 || day > 31 ||
	    month == 0 || day == 0 {
	    return None;
	}
	let d = Datum { year, month, day };
	if day > d.månadslängd() { return None; }
	Some(d)
    }

    pub fn skottår(&self) -> bool {
	self.year % 4 == 0 &&
	    (self.year % 100 != 0 ||
	     self.year % 400 != 0)
    }
}

use std::ops::Add;

impl Add<u16> for Datum {
    type Output = Datum;
    fn add(self, rhs: u16) -> Self {
	let mut d = self.clone();
	d.dagar_framåt(rhs);
	d
    }
}

impl Iterator for Datum {
    type Item = Datum;
    fn next(&mut self) -> Option<Datum> {
	self.dagar_framåt(1);
	Some(self.clone())
    }
}

// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let mut dag = Datum::new(2022, 3, 11).unwrap();
    dag.dagar_framåt(1);
    println!("imorgon: {}", dag);
    println!("sedan: {}", dag + 730);
}
// imorgon: 12 mar 2022
// sedan: 11 mar 2024

/*  ex07f_datumfel  */

#[derive(Debug, Clone)]
pub struct DatumFel {
    pub message: String,
}

impl std::fmt::Display for DatumFel {
    fn fmt(&self, f: &mut std::fmt::Formatter)
	   -> Result<(), std::fmt::Error> {
	write!(f, "{}", self.message)
    }
}

impl std::error::Error for DatumFel { }


/* Övning 801: datumordning

1. Verifiera att icke-ascii-identifierare fungerar.
   (Bl.a. krävs Rust 1.53 eller senare.)
   Om inte, ändra namn på lämpliga ställen
   så att det fungerar.

2. Implementera metoden dag så att den returnerar
   self.day. Och motsvarande för månad och år.

3. Implementera Ord och PartialOrd för Datum.

4. Ändra returtyp för new till Result<Datum, DatumFel>
*/


// Fil: main.rs
mod datum;
use datum::Datum;

fn main() {
    let data = [(2022, 3, 11), (2000, 1, 1), (1999, 12, 31),
		(2024, 3, 11), (1970, 1, 1), (2022, 12, 24)];
    let mut v: Vec<_> = data.iter().map(|&(y, m, d)|
	Datum::new(y as u16, m as u8, d as u8).unwrap()
    ).collect();
    println!("{} {} {}", v[0].år(), v[0].månad(), v[0].dag());
    println!("{} < {}: {}", v[0], v[1].datum(), v[0] < v[1]);
    v.sort();
    for d in v {
	print!("{} ", d);
    }
    println!();
}
// 2022 3 11
// 11 mar 2022 < 2000-01-01: false
// 1 jan 1970 31 dec 1999 1 jan 2000 11 mar 2022 24 dec 2022 11 mar 2024

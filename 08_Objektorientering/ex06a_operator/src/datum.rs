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


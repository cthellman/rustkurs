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

    pub fn år(&self) -> u16 { self.year }

    pub fn månad(&self) -> u8 { self.month }

    pub fn dag(&self) -> u8 { self.day }   

    pub fn new(year: u16, month: u8, day: u8) -> Result<Datum, DatumFel> {
	if month > 12 || month == 0 {
	    return Err(DatumFel {
		message: "felaktig månad".to_string(),
	    });
	}
	let d = Datum { year, month, day };
	if day == 0 ||  day > d.månadslängd() {
	    return Err(DatumFel {
		message: "felaktig månad".to_string(),
	    });
	}	    
	Ok(d)
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

use std::cmp::Ordering;

impl Ord for Datum {
    fn cmp(&self, other: &Self) -> Ordering {
	let s = (self.year, self.month, self.day);
	let o = (other.year, other.month, other.day);
	s.cmp(&o)
    }
}

impl PartialOrd for Datum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Iterator for Datum {
    type Item = Datum;
    fn next(&mut self) -> Option<Datum> {
	self.dagar_framåt(1);
	Some(*self)
    }
}

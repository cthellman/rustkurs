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



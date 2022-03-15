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
	       Datum::MÅNAD[self.month as usize], self.year)	// rust will not auto convert 
		   													// self.month to usize
    }
}

impl Datum {
    const MÅNAD: [&'static str; 13] = [ "",				// 'static - reference lifetime, 														// in this case forever. Ensures that
														// the data the reference points to 
														// exists 
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



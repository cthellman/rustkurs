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


// Fil: datum.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Datum {
    pub year: u16,
    pub month: u8,  // 1-12, one byte
    pub day: u8,  // 1-31, one byte
}


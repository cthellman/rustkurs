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






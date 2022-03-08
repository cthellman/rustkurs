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



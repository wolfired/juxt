use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Adler32(u32);

impl Display for Adler32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

impl PartialEq<u32> for Adler32 {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl FromStr for Adler32 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_bytes(s.as_bytes()))
    }
}

impl Adler32 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Adler32(imp::calc(bytes))
    }
}

mod imp {
    const BASE: u32 = 65521;

    pub fn calc(bytes: &[u8]) -> u32 {
        let mut lo = 1 & 0xffff;
        let mut hi = (1 >> 16) & 0xffff;

        for v in bytes.iter() {
            lo = (lo + *v as u32) % BASE;
            hi = (hi + lo) % BASE;
        }

        (hi << 16) + lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Adler32::from_str("juxt_adler32").unwrap(), 0x20640498);
        assert_eq!(Adler32::from_str("juxt_adler32").unwrap().to_string(), "20640498");
        assert_eq!(Adler32::from_str("juxt_adler32").unwrap(), Adler32::from_str("juxt_adler32").unwrap().clone());
        assert_eq!(format!("{:?}", Adler32::from_str("juxt_adler32").unwrap()), "Adler32(543425688)");
    }
}

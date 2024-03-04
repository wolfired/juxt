use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct Base64(Vec<u8>);

impl From<&[u8]> for Base64 {
    fn from(value: &[u8]) -> Self {
        Self(imp::to_base64(value))
    }
}

impl FromStr for Base64 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.as_bytes()))
    }
}

impl Display for Base64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|c| {
            write!(f, "{}", imp::BASE64_ALPHABET[*c as usize] as char).unwrap();
        });
        write!(f, "")
    }
}

impl Base64 {
    pub fn to_string_safe(&self) -> String {
        String::from_iter(self.0.iter().map(|c| imp::BASE64_ALPHABET_SAFE[*c as usize] as char))
    }
}

mod imp {
    #[rustfmt::skip]
    pub const BASE64_ALPHABET: [u8; 65] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
        b'=',
    ];

    #[rustfmt::skip]
    pub const BASE64_ALPHABET_SAFE: [u8; 65] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'-', b'_',
        b'=',
    ];

    pub fn to_base64(bytes: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity((bytes.len() + 2) / 3 * 4);
        for block in bytes.chunks(3) {
            match block.len() {
                1 => {
                    result.push(block[0] >> 2);
                    result.push((block[0] & 0b00000011) << 4);
                    result.push(64);
                    result.push(64);
                }
                2 => {
                    result.push(block[0] >> 2);
                    result.push(((block[0] & 0b00000011) << 4) | (block[1] >> 4));
                    result.push((block[1] & 0b00001111) << 2);
                    result.push(64);
                }
                _ => {
                    result.push(block[0] >> 2);
                    result.push(((block[0] & 0b00000011) << 4) | (block[1] >> 4));
                    result.push(((block[1] & 0b00001111) << 2) | (block[2] >> 6));
                    result.push(block[2] & 0b00111111);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Base64::from_str("f").unwrap().to_string(), "Zg==");
        assert_eq!(Base64::from_str("+/").unwrap().to_string_safe(), "Ky8=");
        assert_eq!(Base64::from_str("foob").unwrap().to_string(), "Zm9vYg==");
        assert_eq!(format!("{:02x?}", Base64::from("fo".as_bytes())), "Base64([19, 26, 3c, 40])");
    }
}

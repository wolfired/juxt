use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Md5([u8; 16]);

impl Md5 {
    ///
    /// append/padded/extended in place
    ///
    pub fn from_vec(bytes: &mut Vec<u8>) -> Self {
        Md5(imp::calc_in_place(bytes))
    }
}

impl FromStr for Md5 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_vec(&mut s.as_bytes().into()))
    }
}

impl Display for Md5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.0.iter() {
            write!(f, "{:02x}", n).unwrap();
        }
        write!(f, "")
    }
}

mod imp {
    use std::ptr::read_unaligned;

    const A: u32 = 0x67452301;
    const B: u32 = 0xefcdab89;
    const C: u32 = 0x98badcfe;
    const D: u32 = 0x10325476;

    ///
    /// # Generator
    /// 
    /// ```rust
    ///    for i in 0..64 {
    ///        let j;
    ///        if 16 > i {
    ///            j = i;
    ///        } else if 32 > i {
    ///            j = (1 + 5 * i) % 16;
    ///        } else if 48 > i {
    ///            j = (5 + 3 * i) % 16;
    ///        } else {
    ///            j = (7 * i) % 16;
    ///        }
    ///        if 16 == i || 32 == i || 48 == i {
    ///            println!();
    ///        }
    ///        print!("{:#04x}, ", j);
    ///    }
    /// ```
    /// 
    #[rustfmt::skip]
    const KI:[usize; 64] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x01, 0x06, 0x0b, 0x00, 0x05, 0x0a, 0x0f, 0x04, 0x09, 0x0e, 0x03, 0x08, 0x0d, 0x02, 0x07, 0x0c,
        0x05, 0x08, 0x0b, 0x0e, 0x01, 0x04, 0x07, 0x0a, 0x0d, 0x00, 0x03, 0x06, 0x09, 0x0c, 0x0f, 0x02,
        0x00, 0x07, 0x0e, 0x05, 0x0c, 0x03, 0x0a, 0x01, 0x08, 0x0f, 0x06, 0x0d, 0x04, 0x0b, 0x02, 0x09,
    ];

    #[rustfmt::skip]
    const TI:[u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    #[rustfmt::skip]
    const SI:[u32; 64] = [
        0x07, 0x0c, 0x11, 0x16, 0x07, 0x0c, 0x11, 0x16, 0x07, 0x0c, 0x11, 0x16, 0x07, 0x0c, 0x11, 0x16,
        0x05, 0x09, 0x0e, 0x14, 0x05, 0x09, 0x0e, 0x14, 0x05, 0x09, 0x0e, 0x14, 0x05, 0x09, 0x0e, 0x14,
        0x04, 0x0b, 0x10, 0x17, 0x04, 0x0b, 0x10, 0x17, 0x04, 0x0b, 0x10, 0x17, 0x04, 0x0b, 0x10, 0x17,
        0x06, 0x0a, 0x0f, 0x15, 0x06, 0x0a, 0x0f, 0x15, 0x06, 0x0a, 0x0f, 0x15, 0x06, 0x0a, 0x0f, 0x15,
    ];

    #[inline]
    #[rustfmt::skip]
    fn f(b: u32, c: u32, d: u32) -> u32 { b & c | !b & d }

    #[inline]
    #[rustfmt::skip]
    fn g(b: u32, c: u32, d: u32) -> u32 { b & d | c & !d }

    #[inline]
    #[rustfmt::skip]
    fn h(b: u32, c: u32, d: u32) -> u32 { b ^ c ^ d }

    #[inline]
    #[rustfmt::skip]
    fn i(b: u32, c: u32, d: u32) -> u32 { c ^ (b | !d) }

    #[inline]
    #[rustfmt::skip]
    fn ff(a: u32, b: u32, c: u32, d: u32, xi: u32, ti: u32, si: u32) -> u32 {
        // b + (a + f(b, c, d) + xi + ti).rotate_left(si)
        a.wrapping_add(f(b, c, d)).wrapping_add(xi).wrapping_add(ti).rotate_left(si).wrapping_add(b)
    }

    #[inline]
    #[rustfmt::skip]
    fn gg(a: u32, b: u32, c: u32, d: u32, xi: u32, ti: u32, si: u32) -> u32 {
        // b + (a + g(b, c, d) + xi + ti).rotate_left(si)
        a.wrapping_add(g(b, c, d)).wrapping_add(xi).wrapping_add(ti).rotate_left(si).wrapping_add(b)
    }

    #[inline]
    #[rustfmt::skip]
    fn hh(a: u32, b: u32, c: u32, d: u32, xi: u32, ti: u32, si: u32) -> u32 {
        // b + (a + h(b, c, d) + xi + ti).rotate_left(si)
        a.wrapping_add(h(b, c, d)).wrapping_add(xi).wrapping_add(ti).rotate_left(si).wrapping_add(b)
    }

    #[inline]
    #[rustfmt::skip]
    fn ii(a: u32, b: u32, c: u32, d: u32, xi: u32, ti: u32, si: u32) -> u32 {
        // b + (a + i(b, c, d) + xi + ti).rotate_left(si)
        a.wrapping_add(i(b, c, d)).wrapping_add(xi).wrapping_add(ti).rotate_left(si).wrapping_add(b)
    }

    pub fn calc_in_place(bytes: &mut Vec<u8>) -> [u8; 16] {
        let bytes_count = bytes.len();
        let bits_count = bytes_count as u128 * u8::BITS as u128;

        bytes.push(0x80);
        while 448 != bytes.len() * 8 % 512 {
            bytes.push(0x00);
        }

        bytes.append(&mut Into::<Vec<_>>::into(&bits_count.to_le_bytes()[0..8]));

        let mut aa = A;
        let mut bb = B;
        let mut cc = C;
        let mut dd = D;

        for x in bytes.chunks(64) {
            let mut a = aa;
            let mut b = bb;
            let mut c = cc;
            let mut d = dd;

            for i in 0..64 {
                let xi = unsafe { read_unaligned((x as *const [u8] as *mut u32).add(KI[i])).to_le() };
                if 16 > i {
                    (a, b, c, d) = (d, ff(a, b, c, d, xi, TI[i], SI[i]), b, c);
                } else if 32 > i {
                    (a, b, c, d) = (d, gg(a, b, c, d, xi, TI[i], SI[i]), b, c);
                } else if 48 > i {
                    (a, b, c, d) = (d, hh(a, b, c, d, xi, TI[i], SI[i]), b, c);
                } else {
                    (a, b, c, d) = (d, ii(a, b, c, d, xi, TI[i], SI[i]), b, c);
                }
            }

            aa = aa.wrapping_add(a);
            bb = bb.wrapping_add(b);
            cc = cc.wrapping_add(c);
            dd = dd.wrapping_add(d);
        }

        bytes.truncate(bytes_count);

        let mut all = aa.to_le_bytes().into_iter().chain(bb.to_le_bytes().into_iter()).chain(cc.to_le_bytes().into_iter()).chain(dd.to_le_bytes().into_iter());

        std::array::from_fn(|_| all.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_test() {
        assert_eq!(Md5::from_str("just_md5").unwrap().to_string(), "7ada8af6959dc1ec238617351e7b45ea");
        assert_eq!(Md5::from_str("just_md5").unwrap().clone().to_string(), "7ada8af6959dc1ec238617351e7b45ea");
        assert_eq!(Md5::from_str("just_md5").unwrap(), Md5::from_str("just_md5").unwrap());
        assert_eq!(format!("{:02x?}", Md5::from_str("just_md5").unwrap()), "Md5([7a, da, 8a, f6, 95, 9d, c1, ec, 23, 86, 17, 35, 1e, 7b, 45, ea])");
    }
}

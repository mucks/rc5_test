use std::{
    fmt::{Binary, Display, Formatter, LowerHex},
    num::ParseIntError,
};

use crate::hex::{decode_hex, encode_hex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U80 {
    bits: [bool; 80],
}

impl Binary for U80 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bit_str())
    }
}

impl Display for U80 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u128())
    }
}
impl LowerHex for U80 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_str())
    }
}

impl std::ops::BitXor for U80 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        Self { bits }
    }
}

impl std::ops::BitOr for U80 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[i] || rhs.bits[i];
        }
        Self { bits }
    }
}

impl std::ops::Add for U80 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut bits = [false; Self::BITS];
        let mut carry = false;
        for i in (0..Self::BITS).rev() {
            let a = self.bits[i];
            let b = rhs.bits[i];

            if a && b {
                if carry {
                    carry = true;
                    bits[i] = true;
                } else {
                    carry = true;
                    bits[i] = false;
                }
            } else if !a && !b {
                if carry {
                    carry = false;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            } else {
                if carry {
                    carry = true;
                    bits[i] = false;
                } else {
                    carry = false;
                    bits[i] = true;
                }
            }
        }

        Self { bits }
    }
}

impl std::ops::Sub for U80 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut bits = [false; Self::BITS];
        let mut carry = false;
        for i in (0..Self::BITS).rev() {
            let a = self.bits[i];
            let b = rhs.bits[i];

            // 1 - 1
            if a && b {
                if carry {
                    carry = false;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            // 0 - 0
            } else if !a && !b {
                if carry {
                    carry = true;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            // 0 - 1
            } else if !a && b {
                if carry {
                    carry = true;
                    bits[i] = false;
                } else {
                    carry = true;
                    bits[i] = true;
                }
            // 1 - 0
            } else {
                if carry {
                    carry = false;
                    bits[i] = false;
                } else {
                    carry = false;
                    bits[i] = true;
                }
            }
        }

        Self { bits }
    }
}

impl U80 {
    pub const BITS: usize = 80;

    pub const MIN: U80 = U80 {
        bits: [false; Self::BITS],
    };
    pub const MAX: U80 = U80 {
        bits: [true; Self::BITS],
    };

    fn n(n: u128) -> Self {
        Self::from_u128(n)
    }

    pub fn to_u128(&self) -> u128 {
        let mut sum = 0;
        for i in 0..Self::BITS {
            let bit = self.bits[Self::BITS - i - 1];

            if bit {
                sum += 2_u128.pow(i as u32);
            }
        }
        sum
    }

    fn to_bit_str(&self) -> String {
        let mut s = String::new();
        for i in 0..Self::BITS {
            let bit = self.bits[Self::BITS - i - 1];
            s = format!("{}{}", s, if bit { "1" } else { "0" })
        }
        s
    }

    pub fn from_bytes(bytes: &[u8], hex: bool) -> Self {
        let mut bit_string = String::new();
        for byte in bytes {
            let s = format!("{:b}", byte);
            let diff = 8 - s.len();
            let leading_zeroes: String = (0..diff).map(|_| '0').collect();
            let bits = format!("{}{}", leading_zeroes, s);
            if hex {
                bit_string = format!("{}{}", bit_string, bits);
            } else {
                bit_string = format!("{}{}", bits, bit_string);
            }
        }

        let diff = 80 - bit_string.len();
        let leading_zeroes: String = (0..diff).map(|_| '0').collect();
        bit_string = format!("{}{}", leading_zeroes, bit_string);

        let bit_chars: Vec<char> = bit_string.chars().collect();
        let mut bits = [false; Self::BITS];

        for i in 0..Self::BITS {
            bits[i] = bit_chars.get(i) == Some(&'1');
        }

        Self { bits }
    }

    //TODO: test this
    pub fn to_bytes(&self, hex: bool) -> Vec<u8> {
        let bit_str = self.to_bit_str();
        let mut bytes: Vec<u8> = vec![];

        for i in (0..80).step_by(8) {
            let s = bit_str[i..i + 8].to_string();

            // reverse string to assign bits correctly
            let mut bit_chars: Vec<char> = s.chars().collect();
            bit_chars.reverse();

            let mut byte = [false; 8];
            //convert to booleans
            for i in 0..8 {
                byte[i] = bit_chars.get(i) == Some(&'1');
            }

            //convert boolean to u8 using 2^i
            let mut sum: u8 = 0;
            for i in 0..8 {
                let bit = byte[8 - i - 1];
                if bit {
                    sum += 2_u8.pow(i as u32);
                }
            }
            bytes.push(sum);
        }

        if hex {
            bytes.reverse();
        }
        bytes
    }

    // From Hex String
    fn from_hex_str(s: &str) -> Result<Self, ParseIntError> {
        let mut s = decode_hex(s)?;
        Ok(Self::from_bytes(&mut s.as_slice(), true))
    }

    fn to_hex_str(&self) -> String {
        encode_hex(&self.to_bytes(true))
    }

    pub fn from_u128(u: u128) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = (u & (1 << i)) != 0;
        }
        bits.reverse();
        U80 { bits }
    }

    pub fn rotate_left(self, rhs: u32) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[(i + rhs as usize) % Self::BITS];
        }
        Self { bits }
    }

    pub fn rotate_right(self, rhs: u32) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[(i + Self::BITS - rhs as usize) % Self::BITS];
        }
        Self { bits }
    }

    pub fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotate_right() {
        let u = U80::from_u128(2);
        let u = u.rotate_right(1);
        assert_eq!(u.to_u128(), 1);
    }

    #[test]
    fn rotate_left() {
        let u = U80::from_u128(2);
        let u = u.rotate_left(1);
        assert_eq!(u.to_u128(), 4);
    }

    #[test]
    fn rotate_left_wrap() {
        let u = U80::from_u128(2);
        let u = u.rotate_left(79);
        assert_eq!(u.to_u128(), 1);
    }

    #[test]
    fn from_bytes() {
        let a = 11_u8.to_le_bytes();
        let b = 1_u8.to_le_bytes();

        let c = 1111_u32.to_le_bytes();

        let u = U80::from_bytes(&mut a.as_slice(), false);
        assert_eq!(u.to_u128(), 11);
        let u = U80::from_bytes(&mut b.as_slice(), false);
        assert_eq!(u.to_u128(), 1);
        println!("\nc\n");
        let u = U80::from_bytes(&mut c.as_slice(), false);
        assert_eq!(u.to_u128(), 1111);
    }

    #[test]
    fn to_bytes() {
        let a: Vec<u8> = vec![250, 209, 184, 0, 0, 0, 0, 0, 0, 0];

        let u = U80::from_u128(12112378).to_bytes(false);
        assert_eq!(u, a);
    }

    #[test]
    fn from_hex() {
        let s = "40000000000000000000";
        let u = U80::from_hex_str(s).unwrap();
        assert_eq!(u.to_u128(), 0x40000000000000000000);

        let s2 = "40000080000000000000";
        let u = U80::from_hex_str(s2).unwrap();
        assert_eq!(u.to_u128(), 0x40000080000000000000);

        let s3 = "1E854F94";
        let u = U80::from_hex_str(s3).unwrap();
        assert_eq!(u.to_u128(), 0x1E854F94);
    }

    #[test]
    fn to_hex() {
        let key = "02030405060708090a0b";
        let a: u128 = 9500362842338723695115;
        let u = U80::from_u128(a);

        assert_eq!(u.to_hex_str(), key);
    }

    #[test]
    fn add() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(3);

        assert_eq!(a + b, U80::from_u128(6));
    }

    #[test]
    fn sub() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(2);

        assert_eq!(a - b, U80::from_u128(1));
    }

    #[test]
    fn wrapping_add() {
        let a = U80::from_u128(12);
        let b = U80::from_u128(1208925819614629174706172);

        assert_eq!((a + b).to_u128(), 8);
    }

    #[test]
    fn wrapping_sub() {
        let a = U80::from_u128(1);
        let b = U80::from_u128(2);

        assert_eq!((a - b).to_u128(), 1208925819614629174706175);
    }

    #[test]
    fn or() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(4);

        assert_eq!((a | b).to_u128(), 7);
    }
    #[test]
    fn xor() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(5);

        assert_eq!(a ^ b, U80::from_u128(6));
    }
}

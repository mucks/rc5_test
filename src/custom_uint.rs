use std::{
    fmt::{Binary, Display, Formatter, LowerHex},
    num::ParseIntError,
};

use crate::hex::{decode_hex, encode_hex};

pub type U8 = CustomUint<8>;
pub type U16 = CustomUint<16>;
pub type U32 = CustomUint<32>;
pub type U64 = CustomUint<64>;
pub type U128 = CustomUint<128>;

pub type U80 = CustomUint<80>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomUint<const N: usize> {
    bits: [bool; N],
}

impl<const N: usize> CustomUint<N> {
    pub const MIN: Self = Self { bits: [false; N] };
    pub const MAX: Self = Self { bits: [true; N] };

    pub fn to_u128(&self) -> u128 {
        let mut sum = 0;
        for i in 0..N {
            let bit = self.bits[N - i - 1];

            if bit {
                sum += 2_u128.pow(i as u32);
            }
        }
        sum
    }

    pub fn to_u32(&self) -> u32 {
        self.to_u128() as u32
    }

    fn to_bit_str(&self) -> String {
        let mut s = String::new();
        for i in 0..N {
            let bit = self.bits[N - i - 1];
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

        let diff = N - bit_string.len();
        let leading_zeroes: String = (0..diff).map(|_| '0').collect();
        bit_string = format!("{}{}", leading_zeroes, bit_string);

        let bit_chars: Vec<char> = bit_string.chars().collect();
        let mut bits = [false; N];

        for i in 0..N {
            bits[i] = bit_chars.get(i) == Some(&'1');
        }

        Self { bits }
    }

    //TODO: test this
    pub fn to_bytes(&self, hex: bool) -> Vec<u8> {
        let bit_str = self.to_bit_str();
        let mut bytes: Vec<u8> = vec![];

        for i in (0..N).step_by(8) {
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
    pub fn from_hex_str(s: &str) -> Result<Self, ParseIntError> {
        let s = decode_hex(s)?;
        Ok(Self::from_bytes(&mut s.as_slice(), true))
    }

    pub fn to_hex_str(&self) -> String {
        encode_hex(&self.to_bytes(true))
    }

    pub fn from_u128(u: u128) -> Self {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = (u & (1 << i)) != 0;
        }
        bits.reverse();
        Self { bits }
    }

    pub fn rotate_left(self, rhs: u32) -> Self {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = self.bits[(i + rhs as usize) % N];
        }
        Self { bits }
    }

    // works fine!
    pub fn rotate_right(self, rhs: u32) -> Self {
        let mut bits = [false; N];

        for i in 0..N {
            let bit_index = ((i + N).wrapping_sub(rhs as usize)) % N;
            bits[i] = self.bits[bit_index];
        }

        let s = Self { bits };
        // #[cfg(test)]
        // println!("{} >> {} = {}", self, rhs, s.to_u128());
        s
    }

    pub fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl<const N: usize> Binary for CustomUint<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_bit_str())
    }
}

impl<const N: usize> Display for CustomUint<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u128())
    }
}
impl<const N: usize> LowerHex for CustomUint<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex_str())
    }
}

// works fine!
impl<const N: usize> std::ops::BitXor for CustomUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = self.bits[i] ^ rhs.bits[i];
        }

        let s = Self { bits };

        // #[cfg(test)]
        // println!("{} ^ {} = {}", self, rhs, s);

        s
    }
}

impl<const N: usize> std::ops::BitOr for CustomUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; N];
        for i in 0..N {
            bits[i] = self.bits[i] || rhs.bits[i];
        }
        Self { bits }
    }
}

impl<const N: usize> std::ops::Add for CustomUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut bits = [false; N];
        let mut carry = false;
        for i in (0..N).rev() {
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

//TODO: fix subtraction
impl<const N: usize> std::ops::Sub for CustomUint<N> {
    type Output = Self;
    fn sub(self, mut rhs: Self) -> Self::Output {
        let s = format!("{} - {}", self, rhs);
        for i in (0..N).rev() {
            rhs.bits[i] = !rhs.bits[i];
        }

        let sum = self + rhs + Self::from_u128(1);

        #[cfg(test)]
        println!("{} = {}", s, sum);

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_conversion() {
        let v: u32 = 500;
        let u: u8 = v as u8;

        let t = v % 256;

        assert_eq!(u, t as u8);
    }

    #[test]
    fn u32_conversion() {
        let v64: u64 = 902166487400020018;
        //let v128: u128 = 902166487400020018;

        let u = U80::from_bytes(&v64.to_le_bytes(), false);

        println!("{} == {}", u.to_u32(), v64 as u32);

        assert_eq!(u.to_u32(), v64 as u32);
    }

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
        let u = U80::from_u128(2);
        let u = u.rotate_left(80);
        assert_eq!(u.to_u128(), 2);
        let u = U80::from_u128(2);
        let u = u.rotate_left(81);
        assert_eq!(u.to_u128(), 4);
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
        let a = U80::from_u128(2);
        let b = U80::from_u128(1208925819614629174706175);
        assert_eq!((a + b).to_u128(), 1);
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

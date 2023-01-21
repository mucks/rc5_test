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
            let bit_index = ((i + N) - rhs as usize) % N;
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

//TODO: fix subtraction
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

impl<const N: usize> std::ops::Sub for CustomUint<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut bits = [false; N];
        let mut borrow_mode = false;
        let mut next_one_index = 0;
        for i in (0..N).rev() {
            let mut a = self.bits[i];
            let b = rhs.bits[i];

            if i == next_one_index {
                borrow_mode = false;
                a = !a;
            }

            if borrow_mode {
                // 1 - 1
                if a && b {
                    bits[i] = true;
                // 0 - 0
                } else if !a && !b {
                    bits[i] = true;
                // 0 - 1
                } else if !a && b {
                    bits[i] = false;
                // 1 - 0
                } else {
                    bits[i] = false;
                }

                continue;
            }

            // 1 - 1
            if a && b {
                bits[i] = false;
            // 0 - 0
            } else if !a && !b {
                bits[i] = false
            // 0 - 1
            } else if !a && b {
                bits[i] = true;

                for j in (0..i).rev() {
                    if self.bits[j] {
                        borrow_mode = true;
                        next_one_index = j;
                        break;
                    }
                }
            // 1 - 0
            } else {
                bits[i] = true;
            }
        }

        let s = Self { bits };
        #[cfg(test)]
        println!("{} - {} = {}", self, rhs, s);
        s
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn u8_math() {
        // rotate right
        let s = 238_u8.rotate_right(186);
        assert_eq!(s, 187);
        let s = 181_u8.rotate_right(33);
        assert_eq!(s, 218);
        let s = 236_u8.rotate_right(251);
        assert_eq!(s, 157);

        // xor
        assert_eq!(218_u8 ^ 33, 251);
        assert_eq!(157_u8 ^ 251, 102);
        assert_eq!(190_u8 ^ 102, 216);

        // subtraction
        assert_eq!(42_u8.wrapping_sub(181), 181);
    }
}

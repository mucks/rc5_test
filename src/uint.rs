use std::{
    fmt::{Binary, Display, LowerHex},
    ops::{Add, BitOr, BitXor, Sub},
};

use crate::{custom_uint::CustomUInt, from_bytes::FromBytes};
use std::fmt::Debug;

pub trait UInt:
    Add<Output = Self>
    + Sub<Output = Self>
    + BitXor<Output = Self>
    + BitOr<Output = Self>
    + PartialEq
    + Copy
    + Debug
    + Binary
    + Display
    + LowerHex
{
    fn zero() -> Self;
    fn n(u: u32) -> Self;
    fn from_u8(u: u8) -> Self;
    fn wadd(self, rhs: Self) -> Self;
    fn wsub(self, rhs: Self) -> Self;
    fn rotl(self, rhs: u32) -> Self;
    fn rotr(self, rhs: u32) -> Self;
    fn into_u32(self) -> u32;
    fn from_bytes(a: &mut &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn range() -> usize;
    // The length of a word in bits, typically 16, 32 or 64. Encryption is done in 2-word blocks.
    fn w() -> usize;
    /*
    The first magic constant, defined as  Odd((e-2)*2^w),
    where Odd is the nearest odd integer to the given input,
    e is the base of the natural logarithm, and w is defined above.
    */
    fn pw() -> Self;
    /*
    The second magic constant, defined as Odd((\phi - 1) * 2^w),
    where Odd is the nearest odd integer to the given input, where
    \phi  is the golden ratio, and w is defined above.
    */
    fn qw() -> Self;
}

impl<const N: usize> UInt for CustomUInt<N> {
    fn zero() -> Self {
        Self::from_u128(0)
    }

    fn n(u: u32) -> Self {
        Self::from_u128(u as u128)
    }

    fn from_u8(u: u8) -> Self {
        Self::from_u128(u as u128)
    }

    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }

    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }

    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }

    fn into_u32(self) -> u32 {
        self.to_u32()
    }

    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_bytes(&a, false)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes(false)
    }

    fn range() -> usize {
        N / 8
    }

    fn w() -> usize {
        N
    }

    fn pw() -> Self {
        let pw = match N {
            8 => 0xB7,
            16 => 0xB7E1,
            32 => 0xB7E15163,
            64 => 0xB7E151628AED2A6B,
            128 => 0xB7E151628AED2A6ABF7158809CF4F3C7,
            24 => 0xB7E151,
            80 => 0xB7E151628AED2A6ABF71,
            _ => todo!(),
        };
        Self::from_u128(pw)
    }

    fn qw() -> Self {
        let qw = match N {
            8 => 0x9E,
            16 => 0x9E37,
            32 => 0x9E3779B9,
            64 => 0x9E3779B97F4A7C15,
            128 => 0x9E3779B97F4A7C15F39CC0605CEDC835,
            24 => 0x9E3779,
            80 => 0x9E3779B97F4A7C15F39D,
            //256 => 0x9E3779B97F4A7C15F39CC0605CEDC8341082276BF3A27251F86C6A11D0C18E95,
            _ => todo!(),
        };
        Self::from_u128(qw)
    }
}

// The code below is only to support rust internal types for convenience
// not strictly necessary for the code to work because CustomUInt supports U8,U16,U32,U64,U128

impl UInt for u8 {
    fn zero() -> Self {
        0
    }
    fn n(u: u32) -> Self {
        u as u8
    }
    fn from_u8(u: u8) -> Self {
        u as u8
    }
    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }
    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_le_bytes(FromBytes::from_le_bytes(a))
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn range() -> usize {
        1
    }
    fn pw() -> Self {
        0xB7
    }
    fn qw() -> Self {
        0x9E
    }
    fn w() -> usize {
        8
    }
}

impl UInt for u16 {
    fn zero() -> Self {
        0
    }
    fn n(u: u32) -> Self {
        u as u16
    }
    fn from_u8(u: u8) -> Self {
        u as u16
    }
    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }
    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_le_bytes(FromBytes::from_le_bytes(a))
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn range() -> usize {
        2
    }
    fn pw() -> Self {
        0xB7E1
    }
    fn qw() -> Self {
        0x9E37
    }
    fn w() -> usize {
        16
    }
}

impl UInt for u32 {
    fn zero() -> Self {
        0
    }
    fn n(u: u32) -> Self {
        u as u32
    }
    fn from_u8(u: u8) -> Self {
        u as u32
    }
    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }
    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_le_bytes(FromBytes::from_le_bytes(a))
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn range() -> usize {
        4
    }
    fn pw() -> Self {
        0xB7E15163
    }
    fn qw() -> Self {
        0x9E3779B9
    }
    fn w() -> usize {
        32
    }
}

impl UInt for u64 {
    fn zero() -> Self {
        0
    }
    fn n(u: u32) -> Self {
        u as u64
    }
    fn from_u8(u: u8) -> Self {
        u as u64
    }
    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }
    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_le_bytes(FromBytes::from_le_bytes(a))
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
    fn range() -> usize {
        8
    }
    fn pw() -> Self {
        0xB7E151628AED2A6B
    }
    fn qw() -> Self {
        0x9E3779B97F4A7C15
    }
    fn w() -> usize {
        64
    }
}

impl UInt for u128 {
    fn zero() -> Self {
        0
    }
    fn n(u: u32) -> Self {
        u as u128
    }
    fn from_u8(u: u8) -> Self {
        u as u128
    }
    fn wadd(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
    fn wsub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
    fn rotl(self, rhs: u32) -> Self {
        self.rotate_left(rhs)
    }
    fn rotr(self, rhs: u32) -> Self {
        self.rotate_right(rhs)
    }
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn from_bytes(a: &mut &[u8]) -> Self {
        Self::from_le_bytes(FromBytes::from_le_bytes(a))
    }
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn range() -> usize {
        u128::BITS as usize / 8
    }
    fn pw() -> Self {
        0xB7E151628AED2A6ABF7158809CF4F3C7
    }
    fn qw() -> Self {
        0x9E3779B97F4A7C15F39CC0605CEDC835
    }
    fn w() -> usize {
        u128::BITS as usize
    }
}

use std::{
    fmt::{Binary, Display, LowerHex},
    ops::{Add, BitOr, BitXor, Sub},
};

use crate::{from_bytes::FromBytes, u80::U80};
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

impl UInt for U80 {
    fn n(u: u32) -> Self {
        U80::from_u128(u as u128)
    }

    fn zero() -> Self {
        U80::from_u128(0)
    }

    fn from_u8(u: u8) -> Self {
        U80::from_u128(u as u128)
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
        self.to_u128() as u32
    }

    fn from_bytes(a: &mut &[u8]) -> Self {
        U80::from_bytes(a, true)
    }

    fn to_bytes(&self) -> Vec<u8> {
        U80::to_bytes(&self, true)
    }

    fn range() -> usize {
        U80::BITS / 8
    }

    fn w() -> usize {
        U80::BITS
    }

    fn pw() -> Self {
        U80::from_u128(0xB7E151628AED2A6ABF71)
    }

    fn qw() -> Self {
        U80::from_u128(0x9E3779B97F4A7C15F39D)
    }
}

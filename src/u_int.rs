use std::ops::{Add, BitOr, BitXor, Shl, Shr, Sub};

use crate::from_bytes::FromBytes;
use std::fmt::Debug;

pub trait UInt:
    Add<Output = Self>
    + Sub<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitXor<Output = Self>
    + BitOr<Output = Self>
    + PartialEq
    + Copy
    + Debug
{
    fn zero() -> Self;
    fn from_u32(u: u32) -> Self;
    fn from_u8(u: u8) -> Self;
    fn from_u128(u: u128) -> Self;
    fn from_i32(i: i32) -> Self;
    fn wadd(self, rhs: Self) -> Self;
    fn wsub(self, rhs: Self) -> Self;
    fn rotl(self, rhs: u32) -> Self;
    fn rotr(self, rhs: u32) -> Self;
    fn into_u32(self) -> u32;
    fn from_bytes(a: &mut &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn range() -> usize;
    fn pw() -> Self;
    fn qw() -> Self;
    fn block_size() -> usize;
}

impl UInt for u8 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as u8
    }
    fn from_u8(u: u8) -> Self {
        u as u8
    }
    fn from_i32(i: i32) -> Self {
        i as u8
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

    fn from_u128(u: u128) -> Self {
        u as u8
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
    fn block_size() -> usize {
        8
    }
}

impl UInt for u16 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as u16
    }
    fn from_u8(u: u8) -> Self {
        u as u16
    }
    fn from_i32(i: i32) -> Self {
        i as u16
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
    fn from_u128(u: u128) -> Self {
        u as u16
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
    fn block_size() -> usize {
        16
    }
}

impl UInt for u32 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as u32
    }
    fn from_u8(u: u8) -> Self {
        u as u32
    }
    fn from_i32(i: i32) -> Self {
        i as u32
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

    fn from_u128(u: u128) -> Self {
        u as u32
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
    fn block_size() -> usize {
        32
    }
}

impl UInt for u64 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as u64
    }
    fn from_u8(u: u8) -> Self {
        u as u64
    }
    fn from_i32(i: i32) -> Self {
        i as u64
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
    fn from_u128(u: u128) -> Self {
        u as u64
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
    fn block_size() -> usize {
        64
    }
}

impl UInt for u128 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as u128
    }
    fn from_u8(u: u8) -> Self {
        u as u128
    }
    fn from_i32(i: i32) -> Self {
        i as u128
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

    fn from_u128(u: u128) -> Self {
        u as u128
    }

    fn range() -> usize {
        16
    }
    fn pw() -> Self {
        0xB7E151628AED2A6ABF7158809CF4F3C7
    }
    fn qw() -> Self {
        0x9E3779B97F4A7C15F39CC0605CEDC835
    }
    fn block_size() -> usize {
        128
    }
}

use std::ops::{Add, BitOr, BitXor, Shl, Shr, Sub};

use crate::from_bytes::FromBytes;
use std::fmt::Debug;

pub trait Int:
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
}

impl Int for i8 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as i8
    }
    fn from_u8(u: u8) -> Self {
        u as i8
    }
    fn from_i32(i: i32) -> Self {
        i as i8
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
        u as i8
    }
}

impl Int for i16 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as i16
    }
    fn from_u8(u: u8) -> Self {
        u as i16
    }
    fn from_i32(i: i32) -> Self {
        i as i16
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
        u as i16
    }
}

impl Int for i32 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as i32
    }
    fn from_u8(u: u8) -> Self {
        u as i32
    }
    fn from_i32(i: i32) -> Self {
        i
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
        u as i32
    }
}

impl Int for i64 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as i64
    }
    fn from_u8(u: u8) -> Self {
        u as i64
    }
    fn from_i32(i: i32) -> Self {
        i as i64
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
        u as i64
    }
}

impl Int for i128 {
    fn zero() -> Self {
        0
    }
    fn from_u32(u: u32) -> Self {
        u as i128
    }
    fn from_u8(u: u8) -> Self {
        u as i128
    }
    fn from_i32(i: i32) -> Self {
        i as i128
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
        u as i128
    }
}

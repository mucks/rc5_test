#[derive(Debug, Clone, Copy)]
pub enum BlockSize {
    BlockSize16 = 16,
    BlockSize32 = 32,
    BlockSize64 = 64,
    BlockSize128 = 128,
}

impl Default for BlockSize {
    fn default() -> Self {
        Self::BlockSize32
    }
}

impl BlockSize {
    pub fn range(&self) -> usize {
        match self {
            BlockSize::BlockSize16 => 2,
            BlockSize::BlockSize32 => 4,
            BlockSize::BlockSize64 => 8,
            BlockSize::BlockSize128 => 16,
        }
    }
    //Pw - The first magic constant, defined as Odd((e-2) * 2^w) where Odd is the nearest odd integer to the given input
    pub fn pw(&self) -> u128 {
        match self {
            BlockSize::BlockSize16 => 0xB7E1,
            BlockSize::BlockSize32 => 0xB7E15163,
            BlockSize::BlockSize64 => 0xB7E151628AED2A6B,
            BlockSize::BlockSize128 => 0xB7E151628AED2A6ABF7158809CF4F3C7,
        }
    }
    //Qw - The second magic constant, defined as Odd((golden_ratio -1) * 2^w) where Odd is the nearest odd integer to the given input
    pub fn qw(&self) -> u128 {
        match self {
            BlockSize::BlockSize16 => 0x9E37,
            BlockSize::BlockSize32 => 0x9E3779B9,
            BlockSize::BlockSize64 => 0x9E3779B97F4A7C15,
            BlockSize::BlockSize128 => 0x9E3779B97F4A7C15F39CC0605CEDC835,
        }
    }

    // The following code is an attempt to calculate the magic constants at runtime.

    // pub fn pw(&self) -> u64 {
    //     use std::f64::consts::E;

    //     let d = (E - 2.) * 2_u64.pow(*self as u32) as f64;

    //     println!("d: {}", d);
    //     odd(d)
    // }

    // pub fn qw(&self) -> u64 {
    //     let golden_ratio = (1. + 5_f64.sqrt()) / 2.;

    //     let d = (golden_ratio - 1.) * 2_u64.pow(*self as u32) as f64;
    //     odd(d)
    // }
}

// get nearest odd integer given a float
// fn odd(d: f64) -> u64 {
//     (((d + 1.) / 2.) * 2. - 1.).round() as u64
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn block_size_pw_test() {
//         assert_eq!(BlockSize::BlockSize16.pw(), 0xB7E1);
//         assert_eq!(BlockSize::BlockSize32.pw(), 0xB7E15163);
//         //TODO: fix this 'attempt to multiply with overflow'
//         //assert_eq!(BlockSize::BlockSize64.pw(), 0xB7E151628AED2A6B);
//     }

//     #[test]
//     fn block_size_qw_test() {
//         assert_eq!(BlockSize::BlockSize16.qw(), 0x9E37);
//         assert_eq!(BlockSize::BlockSize32.qw(), 0x9E3779B9);
//         //assert_eq!(BlockSize::BlockSize64.qw(), 0x9E3779B97F4A7C15);
//     }
// }

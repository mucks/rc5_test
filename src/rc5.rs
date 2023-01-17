/*
RC5 implementation in Rust
algorithm source:
    https://en.wikipedia.org/wiki/RC5
*/

use std::convert::TryInto;

// Should be 32-bit = 4 bytes
type WORD = u32;

pub struct Rc5 {
    word_size_in_bits: u32,
    rounds: u32,
    key_length_in_bytes: u32,
    s: Vec<i32>,
}

impl Default for Rc5 {
    fn default() -> Self {
        Self {
            word_size_in_bits: 32,
            rounds: 12,
            key_length_in_bytes: 16,
            s: vec![],
        }
    }
}

impl Rc5 {
    pub fn new(word_size_in_bits: u32, rounds: u32, key_length_in_bytes: u32) -> Rc5 {
        Rc5 {
            word_size_in_bits,
            rounds,
            key_length_in_bytes,
            s: vec![],
        }
    }

    pub fn encode(&self, plaintext: Vec<u8>, ciphertext: &mut Vec<u8>) {
        let slice_a: [u8; 4] = plaintext[0..4].try_into().unwrap();
        let plaintext_a = i32::from_le_bytes(slice_a);
        let slice_b: [u8; 4] = plaintext[4..8].try_into().unwrap();
        let plaintext_b = i32::from_le_bytes(slice_b);

        let mut a = self.s[0] + plaintext_a;
        let mut b = self.s[1] + plaintext_b;

        println!("S[0]: {}", self.s[0]);
        println!("pt[0]: {}", plaintext_a);
        println!("S[1]: {}", self.s[1]);
        println!("pt[1]: {}", plaintext_b);
        println!("A: {}", a);
        println!("B: {}", b);

        for i in 1..self.rounds + 1 {
            a = (a ^ b)
                .rotate_left(b as u32)
                .wrapping_add(self.s[(2 * i) as usize]);

            println!("A{}: {}", i, a);

            b = (b ^ a)
                .rotate_left(a as u32)
                .wrapping_add(self.s[(2 * i + 1) as usize]);

            println!("B{}: {}", i, b);
        }
        println!("A: {}", a);
        println!("B: {}", b);

        ciphertext.extend(a.to_le_bytes());
        ciphertext.extend(b.to_le_bytes());
    }

    fn generate_L(&self, key: &Vec<u8>) -> Vec<i32> {
        let w = self.word_size_in_bits;
        let b = self.key_length_in_bytes as usize;
        let c = (8_f32 * b as f32 / w as f32).ceil().max(1.) as usize;
        // word size in bytes
        let u = (self.word_size_in_bits / 8) as usize;

        let mut l: Vec<i32> = vec![0; c];
        l[c - 1] = 0;

        for i in (0..b).rev() {
            l[i / u] = (l[i / u] << 8) + key[i] as i32;
        }
        l
    }

    fn generate_S(&self) -> Vec<i32> {
        let t = (2 * (self.rounds + 1)) as usize;
        let mut s: Vec<i32> = vec![0; t];
        let pw = calculate_magic_constant_pw(self.word_size_in_bits);
        let qw = calculate_magic_constant_qw(self.word_size_in_bits);

        s[0] = pw as i32;
        for i in 1..t {
            s[i] = s[i - 1].wrapping_add(qw as i32);
        }
        s
    }

    pub fn setup(&mut self, key: Vec<u8>) {
        let w = self.word_size_in_bits;
        let b = self.key_length_in_bytes as usize;
        // length of key in words
        let c = (8_f64 * b as f64 / w as f64).ceil().max(1.) as usize;

        // A temporary working array used during key scheduling. initialized to the key in words.
        let mut l = self.generate_L(&key);
        println!("L: {:?}", l);
        let mut s = self.generate_S();
        println!("S: {:?}", s);

        let mut i = 0;
        let mut j = 0;
        let mut a: i32 = 0;
        let mut b: i32 = 0;

        let t = (2 * (self.rounds + 1)) as usize;
        let len = 3 * t.max(c);

        for _k in 0..len {
            let ab: i32 = a.wrapping_add(b);
            s[i] = s[i].wrapping_add(ab).rotate_left(3);
            a = s[i];

            let ab: i32 = a.wrapping_add(b);
            l[j] = l[j].wrapping_add(ab).rotate_left(ab as u32);
            b = l[j];

            i = (i + 1) % t;
            j = (j + 1) % c;
        }
        self.s = s;
    }
}

//Pw - The first magic constant, defined as Odd((e-2) * 2^w) where Odd is the nearest odd integer to the given input
fn calculate_magic_constant_pw(w: u32) -> u32 {
    use std::f64::consts::E;

    let d = (E - 2.) * (2_u64.pow(w) as f64);
    odd(d)
}

//Qw - The second magic constant, defined as Odd((golden_ratio -1) * 2^w) where Odd is the nearest odd integer to the given input
fn calculate_magic_constant_qw(w: u32) -> u32 {
    let golden_ratio = (1. + 5_f64.sqrt()) / 2.;

    let d = (golden_ratio - 1.) * (2_u64.pow(w) as f64);
    odd(d)
}

// get nearest odd integer given a float
fn odd(d: f64) -> u32 {
    (((d + 1.) / 2.) * 2. - 1.).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_magic_constant_pw_test() {
        assert_eq!(calculate_magic_constant_pw(16), 0xB7E1);
        assert_eq!(calculate_magic_constant_pw(32), 0xB7E15163);
        //TODO: fix this 'attempt to multiply with overflow'
        //assert_eq!(calculate_magic_constant_pw(64), 0xB7E151628AED2A6B);
    }

    #[test]
    fn calculate_magic_constant_qw_test() {
        assert_eq!(calculate_magic_constant_qw(16), 0x9E37);
        assert_eq!(calculate_magic_constant_qw(32), 0x9E3779B9);
    }
}

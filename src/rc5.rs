/*
RC5 implementation in Rust
algorithm source:
    https://en.wikipedia.org/wiki/RC5
*/
use crate::error::Result;
use crate::int::Int;
use crate::{block_size::BlockSize, key_size::KeySize};

pub struct Rc5<T> {
    block_size: BlockSize,
    key_size: KeySize,
    rounds: u8,
    s: Vec<T>,
}

impl<T> Default for Rc5<T> {
    fn default() -> Self {
        Self {
            rounds: 12,
            s: vec![],
            block_size: BlockSize::default(),
            key_size: KeySize::default(),
        }
    }
}

impl<T> Rc5<T>
where
    T: Int,
{
    pub fn new(block_size: BlockSize, rounds: u8, key_size: usize) -> Result<Rc5<T>> {
        Ok(Self {
            block_size,
            rounds,
            key_size: KeySize::new(key_size as u32)?,
            s: vec![],
        })
    }

    fn parse_bytes(&self, plaintext: Vec<u8>) -> (T, T) {
        let range = self.block_size.range();
        let mut slice_a: &[u8] = &plaintext[0..range];
        let plaintext_a = T::from_bytes(&mut slice_a);
        let mut slice_b: &[u8] = &plaintext[range..range * 2];
        let plaintext_b = T::from_bytes(&mut slice_b);
        (plaintext_a, plaintext_b)
    }

    pub fn encode(&self, plaintext: Vec<u8>, ciphertext: &mut Vec<u8>) {
        let (plaintext_a, plaintext_b) = self.parse_bytes(plaintext);
        let mut a = self.s[0].wadd(plaintext_a);
        let mut b = self.s[1].wadd(plaintext_b);

        for i in 1..self.rounds + 1 {
            a = (a ^ b).rotl(b.into_u32()).wadd(self.s[(2 * i) as usize]);

            b = (b ^ a)
                .rotl(a.into_u32())
                .wadd(self.s[(2 * i + 1) as usize]);
        }
        ciphertext.extend(a.to_bytes());
        ciphertext.extend(b.to_bytes());
    }

    pub fn decode(&self, ciphertext: Vec<u8>, plaintext: &mut Vec<u8>) {
        let (ciphertext_a, ciphertext_b) = self.parse_bytes(ciphertext);
        let mut a = ciphertext_a;
        let mut b = ciphertext_b;

        for i in (1..self.rounds + 1).rev() {
            b = ((b.wsub(self.s[(2 * i + 1) as usize])).rotr(a.into_u32())) ^ a;
            a = ((a.wsub(self.s[(2 * i) as usize])).rotr(b.into_u32())) ^ b;
        }

        a = a.wsub(self.s[0]);
        b = b.wsub(self.s[1]);

        plaintext.extend(a.to_bytes());
        plaintext.extend(b.to_bytes());
    }

    fn generate_L(&self, key: &Vec<u8>) -> Vec<T>
    where
        T: Int,
    {
        let w = self.block_size as usize;
        let b = self.key_size.0 as usize;
        let c = (8_f32 * b as f32 / w as f32).ceil().max(1.) as usize;
        // word size in bytes
        let u = (self.block_size as u32 / 8) as usize;

        let mut l: Vec<T> = vec![T::zero(); c];
        l[c - 1] = T::zero();

        for i in (0..b).rev() {
            l[i / u] = (l[i / u] << T::from_u32(8)) + T::from_u8(key[i]);
        }

        l
    }

    fn generate_S(&self) -> Vec<T>
    where
        T: Int,
    {
        let t = (2 * (self.rounds + 1)) as usize;
        let mut s: Vec<T> = vec![T::zero(); t];
        let pw = T::from_u128(self.block_size.pw());
        let qw = T::from_u128(self.block_size.qw());

        s[0] = pw;
        for i in 1..t {
            s[i] = s[i - 1].wadd(qw);
        }
        s
    }

    pub fn setup(&mut self, key: Vec<u8>)
    where
        T: Int,
    {
        let w = self.block_size as usize;
        let b = self.key_size.0 as usize;
        // length of key in words
        let c = (8_f32 * b as f32 / w as f32).ceil().max(1.) as usize;

        // A temporary working array used during key scheduling. initialized to the key in words.
        let mut l: Vec<T> = self.generate_L(&key);
        let mut s: Vec<T> = self.generate_S();

        let mut i = 0;
        let mut j = 0;
        let mut a: T = T::zero();
        let mut b: T = T::zero();

        let t = (2 * (self.rounds + 1)) as usize;
        let len = 3 * t.max(c);

        for _k in 0..len {
            let ab: T = a.wadd(b);
            s[i] = s[i].wadd(ab).rotl(3);
            a = s[i];

            let ab: T = a.wadd(b);
            l[j] = l[j].wadd(ab).rotl(ab.into_u32());
            b = l[j];

            i = (i + 1) % t;
            j = (j + 1) % c;
        }
        self.s = s;
    }
}

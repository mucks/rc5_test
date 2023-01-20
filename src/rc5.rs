/*
RC5 implementation in Rust
algorithm source:
    https://en.wikipedia.org/wiki/RC5
*/
use crate::error::Result;
use crate::key_size::KeySize;
use crate::u_int::UInt;

pub struct Rc5<T> {
    key_size: KeySize,
    // The number of rounds to use when encrypting data.
    rounds: u8,
    s: Vec<T>,
}

impl<T> Default for Rc5<T> {
    fn default() -> Self {
        Self {
            rounds: 12,
            s: vec![],
            key_size: KeySize::default(),
        }
    }
}

impl<T> Rc5<T>
where
    T: UInt,
{
    pub fn new(rounds: u8, key_size: usize) -> Result<Rc5<T>> {
        Ok(Self {
            rounds,
            key_size: KeySize::new(key_size as u32)?,
            s: vec![],
        })
    }

    fn parse_bytes(&self, plaintext: Vec<u8>) -> (T, T) {
        let range = T::range();
        let mut slice_a: &[u8] = &plaintext[0..range];
        println!("slice_a: {:?}", slice_a);
        let plaintext_a = T::from_bytes(&mut slice_a);
        let mut slice_b: &[u8] = &plaintext[range..range * 2];
        println!("slice_b: {:?}", slice_b);
        let plaintext_b = T::from_bytes(&mut slice_b);
        (plaintext_a, plaintext_b)
    }

    pub fn encode(&self, plaintext: Vec<u8>, ciphertext: &mut Vec<u8>) {
        let (plaintext_a, plaintext_b) = self.parse_bytes(plaintext);

        println!("PA: {}", plaintext_a);
        println!("PB: {}", plaintext_b);

        let mut a = self.s[0].wadd(plaintext_a);
        let mut b = self.s[1].wadd(plaintext_b);

        println!("A: {:x}", a);
        println!("B: {:x}", b);

        for i in 1..(self.rounds + 1) as usize {
            a = (a ^ b).rotl(b.into_u32()).wadd(self.s[2 * i]);
            b = (b ^ a).rotl(a.into_u32()).wadd(self.s[2 * i + 1]);
            println!("A: {:x}", a);
            println!("B: {:x}", b);
        }

        ciphertext.extend(a.to_bytes());
        ciphertext.extend(b.to_bytes());
    }

    pub fn decode(&self, ciphertext: Vec<u8>, plaintext: &mut Vec<u8>) {
        let (ciphertext_a, ciphertext_b) = self.parse_bytes(ciphertext);
        let mut a = ciphertext_a;
        let mut b = ciphertext_b;

        for i in (1..(self.rounds + 1) as usize).rev() {
            b = ((b.wsub(self.s[2 * i + 1])).rotr(a.into_u32())) ^ a;
            a = ((a.wsub(self.s[2 * i])).rotr(b.into_u32())) ^ b;
        }

        a = a.wsub(self.s[0]);
        b = b.wsub(self.s[1]);

        plaintext.extend(a.to_bytes());
        plaintext.extend(b.to_bytes());
    }

    //The length of a word in bytes.
    fn u(&self) -> usize {
        T::w() / 8
    }

    // The length of the key in bytes.
    fn b(&self) -> usize {
        self.key_size.0 as usize
    }

    // The length of the key in words (or 1, if b = 0).
    fn c(&self) -> usize {
        // length of key in words
        (8_f32 * self.b() as f32 / T::w() as f32).ceil().max(1.) as usize
    }

    // size of table S in blocks
    fn t(&self) -> usize {
        (2 * (self.rounds + 1)) as usize
    }

    fn print_L(l: &Vec<T>) {
        for i in 0..l.len() {
            println!("L[{}] = {:x}", i, l[i]);
        }
    }

    fn print_S(s: &Vec<T>) {
        for i in 0..s.len() {
            println!("S[{}] = {:x}", i, s[i]);
        }
    }

    // L is initially a c-length list of 0-valued w-length words
    // A temporary working array used during key scheduling. initialized to the key in words.
    fn generate_L(&self, key: &Vec<u8>) -> Vec<T> {
        let mut l: Vec<T> = vec![T::zero(); self.c()];
        l[self.c() - 1] = T::zero();

        // U80: L-values are reversed!
        for i in (0..self.b()).rev() {
            let iu = i / self.u();

            let r = l[iu].rotl(8);

            let k = T::from_u8(key[i]);
            let f = r + k;

            l[iu] = f;
        }

        Self::print_L(&l);

        l
    }
    //Initialize key-independent pseudorandom S array
    //S is initially a t=2(r+1) length list of undefined w-length words
    fn generate_S(&self) -> Vec<T> {
        let mut s: Vec<T> = vec![T::zero(); self.t()];

        s[0] = T::pw();
        for i in 1..self.t() {
            s[i] = s[i - 1].wadd(T::qw());
            // u8 is 8 bits, so we need to add 1 to the last word
            if T::w() == 8 {
                s[i] = s[i].wadd(T::from_u8(1))
            }
        }

        Self::print_S(&s);

        s
    }

    pub fn setup(&mut self, key: Vec<u8>) {
        let mut l: Vec<T> = self.generate_L(&key);
        let mut s: Vec<T> = self.generate_S();

        let mut i = 0;
        let mut j = 0;
        let mut a: T = T::zero();
        let mut b: T = T::zero();

        let len = 3 * self.t().max(self.c());

        // The main key scheduling loop
        for _k in 0..len {
            let ab: T = a.wadd(b);
            s[i] = s[i].wadd(ab).rotl(3);
            a = s[i];

            println!("S[{}] = {:x}", i, s[i]);

            let ab: T = a.wadd(b);
            // L + AB
            let lj = l[j];
            let lpab = lj.wadd(ab);
            l[j] = lpab.rotl(ab.into_u32());
            b = l[j];

            if format!("{:x}", l[j]) == "aacaff91b90252434b8d" {
                println!("{} + {} = {}", lj, ab, lpab);
            }

            //This L is the wrong value!
            println!("L[{}] = {:x}", j, l[j]);

            i = (i + 1) % self.t();
            j = (j + 1) % self.c();
        }
        self.s = s;
    }
}

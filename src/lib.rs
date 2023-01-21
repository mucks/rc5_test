mod custom_uint;
mod error;
mod from_bytes;
mod hex;
mod key_size;
mod rc5;
mod uint;

pub use rc5::Rc5;

pub use custom_uint::{U128, U16, U24, U256, U32, U64, U8, U80};
pub use uint::UInt;

pub fn encode<T>(rounds: u8, key_size: usize, key: Vec<u8>, plaintext: Vec<u8>) -> Vec<u8>
where
    T: UInt,
{
    let mut rc5: Rc5<T> = Rc5::new(rounds, key_size).unwrap();
    rc5.setup(key);
    let mut ciphertext = Vec::new();
    rc5.encode(plaintext, &mut ciphertext);
    ciphertext
}

pub fn decode<T>(rounds: u8, key_size: usize, key: Vec<u8>, ciphertext: Vec<u8>) -> Vec<u8>
where
    T: UInt,
{
    let mut rc5: Rc5<T> = Rc5::new(rounds, key_size).unwrap();
    rc5.setup(key);
    let mut plaintext = Vec::new();
    rc5.decode(ciphertext, &mut plaintext);
    plaintext
}

#[cfg(test)]
mod tests {
    use crate::custom_uint::{U128, U16, U24, U32, U64, U8, U80};
    use crate::error::Result;
    use crate::hex::{decode_hex, encode_hex};

    use crate::encode;
    use crate::{decode, U256};

    use super::*;

    // Given Test Cases

    #[test]
    fn encode_a() {
        let key = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let pt = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let ct = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
        let res = encode::<u32>(12, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_b() {
        let key = vec![
            0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81,
            0xFF, 0x48,
        ];
        let pt = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
        let ct = vec![0x11, 0xE4, 0x3B, 0x86, 0xD2, 0x31, 0xEA, 0x64];
        let res = encode::<u32>(12, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_a() {
        let key = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let pt = vec![0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
        let ct = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
        let res = decode::<u32>(12, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_b() {
        let key = vec![
            0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81,
            0xFF, 0x48,
        ];
        let pt = vec![0x63, 0x8B, 0x3A, 0x5E, 0xF7, 0x2B, 0x66, 0x3F];
        let ct = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
        let res = decode::<u32>(12, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    // Custom Test Cases

    fn parse_key_ct_pt(key: &str, pt: &str, ct: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        Ok((decode_hex(key)?, decode_hex(pt)?, decode_hex(ct)?))
    }

    fn rc5_8_12_4() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "00010203";
        let pt = "0001";
        let ct = "212A";
        parse_key_ct_pt(key, pt, ct)
    }

    fn rc5_16_16_8() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "0001020304050607";
        let pt = "00010203";
        let ct = "23A8D72E";
        parse_key_ct_pt(key, pt, ct)
    }
    fn rc5_32_20_16() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B0C0D0E0F";
        let pt = "0001020304050607";
        let ct = "2A0EDC0E9431FF73";
        parse_key_ct_pt(key, pt, ct)
    }
    fn rc5_64_24_24() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B0C0D0E0F1011121314151617";
        let pt = "000102030405060708090A0B0C0D0E0F";
        let ct = "A46772820EDBCE0235ABEA32AE7178DA";
        parse_key_ct_pt(key, pt, ct)
    }

    fn rc5_128_28_32() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F";
        let pt = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F";
        let ct = "ECA5910921A4F4CFDD7AD7AD20A1FCBA068EC7A7CD752D68FE914B7FE180B440";
        parse_key_ct_pt(key, pt, ct)
    }

    fn rc5_24_4_0() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "";
        let pt = "000102030405";
        let ct = "89CBDCC9525A";
        parse_key_ct_pt(key, pt, ct)
    }

    fn rc5_80_4_12() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B";
        let pt = "000102030405060708090A0B0C0D0E0F10111213";
        let ct = "9CB59ECBA4EA84568A4278B0E132D5FC9D5819D6";
        parse_key_ct_pt(key, pt, ct)
    }

    fn rc5_256_28_32() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F";
        let pt = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F";
        let ct = "5D5759FD8D73772F91219919933A7B63BEB98B695AED436982DB90387181C428CAFF3C7C99D59464B79C0F3CC6EAD369634D37683962139B29B08001FB3D27CD";
        parse_key_ct_pt(key, pt, ct)
    }

    // Tests with standard uints both with internal rust uint and my own custom uint

    #[test]
    fn encode_rc5_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = encode::<u8>(12, 4, key, pt);
        println!("{} == {:?}", encode_hex(&res), encode_hex(&ct));
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_custom_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = encode::<U8>(12, 4, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = decode::<u8>(12, 4, key, ct);
        assert!(&pt[..] == &res[..]);
    }
    #[test]
    fn decode_rc5_custom_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = decode::<U8>(12, 4, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_16_16_8() {
        let (key, pt, ct) = rc5_16_16_8().unwrap();
        let res = encode::<u16>(16, 8, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_custom_16_16_8() {
        let (key, pt, ct) = rc5_16_16_8().unwrap();
        let res = encode::<U16>(16, 8, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_16_16_8() {
        let (key, pt, ct) = rc5_16_16_8().unwrap();
        let res = decode::<u16>(16, 8, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_custom_16_16_8() {
        let (key, pt, ct) = rc5_16_16_8().unwrap();
        let res = decode::<U16>(16, 8, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = encode::<u32>(20, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }
    #[test]
    fn encode_rc5_custom_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = encode::<U32>(20, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = decode::<u32>(20, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_custom_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = decode::<U32>(20, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = encode::<u64>(24, 24, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_custom_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = encode::<U64>(24, 24, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = decode::<u64>(24, 24, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_custom_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = decode::<U64>(24, 24, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = encode::<u128>(28, 32, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_custom_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = encode::<U128>(28, 32, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = decode::<u128>(28, 32, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_custom_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = decode::<U128>(28, 32, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    // Irregular UINTS

    #[test]
    #[ignore]
    fn encode_rc5_80_4_12() {
        let (key, pt, ct) = rc5_80_4_12().unwrap();
        let res = encode::<U80>(4, 12, key, pt);

        println!("{} == {}", encode_hex(&res), encode_hex(&ct));

        assert!(&res[..] == &ct[..]);
    }

    #[test]
    #[ignore]
    fn decode_rc5_80_4_12() {
        let (key, pt, ct) = rc5_80_4_12().unwrap();
        let res = decode::<U80>(4, 12, key, ct);

        println!("{} == {}", encode_hex(&res), encode_hex(&pt));

        assert!(&res[..] == &pt[..]);
    }

    #[test]
    #[ignore]
    fn encode_rc5_24_4_0() {
        let (key, pt, ct) = rc5_24_4_0().unwrap();
        let res = encode::<U24>(4, 0, key, pt);

        println!("{} == {}", encode_hex(&res), encode_hex(&ct));

        assert!(&res[..] == &ct[..]);
    }

    #[test]
    #[ignore]
    fn decode_rc5_24_4_0() {
        let (key, pt, ct) = rc5_24_4_0().unwrap();
        let res = decode::<U24>(4, 0, key, ct);

        println!("{} == {}", encode_hex(&res), encode_hex(&pt));

        assert!(&res[..] == &pt[..]);
    }

    #[test]
    #[ignore]
    fn encode_rc5_256_28_32() {
        let (key, pt, ct) = rc5_256_28_32().unwrap();
        let res = encode::<U256>(28, 32, key, pt);

        println!("{} == {}", encode_hex(&res), encode_hex(&ct));

        assert!(&res[..] == &ct[..]);
    }
}

use crate::{rc5::Rc5, u_int::UInt};

/*
 * This function should return a cipher text for a given key and plaintext
 *
 */
fn encode<T>(rounds: u8, key_size: usize, key: Vec<u8>, plaintext: Vec<u8>) -> Vec<u8>
where
    T: UInt,
{
    let mut rc5: Rc5<T> = Rc5::new(rounds, key_size).unwrap();
    rc5.setup(key);
    let mut ciphertext = Vec::new();
    rc5.encode(plaintext, &mut ciphertext);
    ciphertext
}

/*
 * This function should return a plaintext for a given key and ciphertext
 *
 */
fn decode<T>(rounds: u8, key_size: usize, key: Vec<u8>, ciphertext: Vec<u8>) -> Vec<u8>
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

    use crate::custom_uint::{U128, U16, U32, U64, U8};
    use crate::error::Result;
    use crate::hex::{decode_hex, encode_hex};
    use crate::u80::U80;

    use super::*;

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

    fn rc5_80_4_12() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let key = "000102030405060708090A0B";
        let pt = "000102030405060708090A0B0C0D0E0F10111213";
        let ct = "9CB59ECBA4EA84568A4278B0E132D5FC9D5819D6";
        parse_key_ct_pt(key, pt, ct)
    }

    #[test]
    fn encode_rc5_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = encode::<U8>(12, 4, key, pt);
        println!("{} == {:?}", encode_hex(&res), encode_hex(&ct));
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = decode::<u8>(12, 4, key, ct);
        assert!(&pt[..] == &res[..]);
    }
    #[test]
    fn decode_custom_rc5_8_12_4() {
        let (key, pt, ct) = rc5_8_12_4().unwrap();
        let res = decode::<U8>(12, 4, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_16_16_8() {
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
    fn decode_custom_rc5_16_16_8() {
        let (key, pt, ct) = rc5_16_16_8().unwrap();
        let res = decode::<U16>(16, 8, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = encode::<U32>(20, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc_32_20_16() {
        let (key, pt, ct) = rc5_32_20_16().unwrap();
        let res = decode::<u32>(20, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = encode::<U64>(24, 24, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc_64_24_24() {
        let (key, pt, ct) = rc5_64_24_24().unwrap();
        let res = decode::<u64>(24, 24, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn encode_rc_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = encode::<U128>(28, 32, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_rc_128_28_32() {
        let (key, pt, ct) = rc5_128_28_32().unwrap();
        let res = decode::<u128>(28, 32, key, ct);
        assert!(&pt[..] == &res[..]);
    }
    #[test]
    fn encode_rc5_80_4_12() {
        let (key, pt, ct) = rc5_80_4_12().unwrap();
        let res = encode::<U80>(4, 12, key, pt);

        println!("{} == {}", encode_hex(&res), encode_hex(&ct));

        assert!(&res[..] == &ct[..]);
    }

    #[test]
    fn decode_rc5_80_4_12() {
        let (key, pt, ct) = rc5_80_4_12().unwrap();
        let res = decode::<U80>(4, 12, key, ct);

        println!("{} == {}", encode_hex(&res), encode_hex(&pt));

        assert!(&res[..] == &pt[..]);
    }
}

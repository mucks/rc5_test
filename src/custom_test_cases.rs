use crate::{block_size::BlockSize, int::Int, rc5::Rc5};

/*
 * This function should return a cipher text for a given key and plaintext
 *
 */
fn encode<T>(
    block_size: BlockSize,
    rounds: u8,
    key_size: usize,
    key: Vec<u8>,
    plaintext: Vec<u8>,
) -> Vec<u8>
where
    T: Int,
{
    let mut rc5: Rc5<T> = Rc5::new(block_size, rounds, key_size).unwrap();
    rc5.setup(key);
    let mut ciphertext = Vec::new();
    rc5.encode(plaintext, &mut ciphertext);
    ciphertext
}

/*
 * This function should return a plaintext for a given key and ciphertext
 *
 */
fn decode<T>(
    block_size: BlockSize,
    rounds: u8,
    key_size: usize,
    key: Vec<u8>,
    ciphertext: Vec<u8>,
) -> Vec<u8>
where
    T: Int,
{
    let mut rc5: Rc5<T> = Rc5::new(block_size, rounds, key_size).unwrap();
    rc5.setup(key);
    let mut plaintext = Vec::new();
    rc5.decode(ciphertext, &mut plaintext);
    plaintext
}

#[cfg(test)]
mod tests {
    use crate::hex::{decode_hex, encode_hex};

    use super::*;

    #[test]
    fn encode_a_from_hex_str() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let pt = decode_hex("0011223344556677").unwrap();
        let ct = decode_hex("2DDC149BCF088B9E").unwrap();
        let res = encode::<i32>(BlockSize::BlockSize32, 12, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_8_12_4() {
        let key = decode_hex("00010203").unwrap();
        let pt = decode_hex("0001").unwrap();
        let ct = decode_hex("212A").unwrap();
        let res = encode::<i8>(BlockSize::BlockSize8, 12, 4, key, pt);
        println!("{} == {}", encode_hex(&ct), encode_hex(&res));
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc5_16_16_8() {
        let key = decode_hex("0001020304050607").unwrap();
        let pt = decode_hex("00010203").unwrap();
        let ct = decode_hex("23A8D72E").unwrap();
        let res = encode::<i16>(BlockSize::BlockSize16, 16, 8, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc_32_20_16() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let pt = decode_hex("0001020304050607").unwrap();
        let ct = decode_hex("2A0EDC0E9431FF73").unwrap();
        let res = encode::<i32>(BlockSize::BlockSize32, 20, 16, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc_64_24_24() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F1011121314151617").unwrap();
        let pt = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let ct = decode_hex("A46772820EDBCE0235ABEA32AE7178DA").unwrap();
        let res = encode::<i64>(BlockSize::BlockSize64, 24, 24, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_rc_128_28_32() {
        let key =
            decode_hex("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F").unwrap();
        let pt =
            decode_hex("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F").unwrap();
        let ct =
            decode_hex("ECA5910921A4F4CFDD7AD7AD20A1FCBA068EC7A7CD752D68FE914B7FE180B440").unwrap();
        let res = encode::<i128>(BlockSize::BlockSize128, 28, 32, key, pt);
        assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_a_from_hex_str() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let pt = decode_hex("0011223344556677").unwrap();
        let ct = decode_hex("2DDC149BCF088B9E").unwrap();
        let res = decode::<i32>(BlockSize::BlockSize32, 12, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_8_12_4() {
        let key = decode_hex("00010203").unwrap();
        let pt = decode_hex("0001").unwrap();
        let ct = decode_hex("212A").unwrap();
        let res = decode::<i8>(BlockSize::BlockSize8, 12, 4, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc5_16_16_8() {
        let key = decode_hex("0001020304050607").unwrap();
        let pt = decode_hex("00010203").unwrap();
        let ct = decode_hex("23A8D72E").unwrap();
        let res = decode::<i16>(BlockSize::BlockSize16, 16, 8, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc_32_20_16() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let pt = decode_hex("0001020304050607").unwrap();
        let ct = decode_hex("2A0EDC0E9431FF73").unwrap();
        let res = decode::<i32>(BlockSize::BlockSize32, 20, 16, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc_64_24_24() {
        let key = decode_hex("000102030405060708090A0B0C0D0E0F1011121314151617").unwrap();
        let pt = decode_hex("000102030405060708090A0B0C0D0E0F").unwrap();
        let ct = decode_hex("A46772820EDBCE0235ABEA32AE7178DA").unwrap();
        let res = decode::<i64>(BlockSize::BlockSize64, 24, 24, key, ct);
        assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_rc_128_28_32() {
        let key =
            decode_hex("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F").unwrap();
        let pt =
            decode_hex("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F").unwrap();
        let ct =
            decode_hex("ECA5910921A4F4CFDD7AD7AD20A1FCBA068EC7A7CD752D68FE914B7FE180B440").unwrap();
        let res = decode::<i128>(BlockSize::BlockSize128, 28, 32, key, ct);
        assert!(&pt[..] == &res[..]);
    }
}

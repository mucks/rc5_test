use crate::custom_uint::CustomUint;

pub type U80 = CustomUint<80>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_conversion() {
        let v: u32 = 500;
        let u: u8 = v as u8;

        let t = v % 256;

        assert_eq!(u, t as u8);
    }

    #[test]
    fn u32_conversion() {
        let v64: u64 = 902166487400020018;
        //let v128: u128 = 902166487400020018;

        let u = U80::from_bytes(&v64.to_le_bytes(), false);

        println!("{} == {}", u.to_u32(), v64 as u32);

        assert_eq!(u.to_u32(), v64 as u32);
    }

    #[test]
    fn rotate_right() {
        let u = U80::from_u128(2);
        let u = u.rotate_right(1);
        assert_eq!(u.to_u128(), 1);
    }

    #[test]
    fn rotate_left() {
        let u = U80::from_u128(2);
        let u = u.rotate_left(1);
        assert_eq!(u.to_u128(), 4);
    }

    #[test]
    fn rotate_left_wrap() {
        let u = U80::from_u128(2);
        let u = u.rotate_left(79);
        assert_eq!(u.to_u128(), 1);
        let u = U80::from_u128(2);
        let u = u.rotate_left(80);
        assert_eq!(u.to_u128(), 2);
        let u = U80::from_u128(2);
        let u = u.rotate_left(81);
        assert_eq!(u.to_u128(), 4);
    }

    #[test]
    fn from_bytes() {
        let a = 11_u8.to_le_bytes();
        let b = 1_u8.to_le_bytes();

        let c = 1111_u32.to_le_bytes();

        let u = U80::from_bytes(&mut a.as_slice(), false);
        assert_eq!(u.to_u128(), 11);
        let u = U80::from_bytes(&mut b.as_slice(), false);
        assert_eq!(u.to_u128(), 1);
        println!("\nc\n");
        let u = U80::from_bytes(&mut c.as_slice(), false);
        assert_eq!(u.to_u128(), 1111);
    }

    #[test]
    fn to_bytes() {
        let a: Vec<u8> = vec![250, 209, 184, 0, 0, 0, 0, 0, 0, 0];

        let u = U80::from_u128(12112378).to_bytes(false);
        assert_eq!(u, a);
    }

    #[test]
    fn from_hex() {
        let s = "40000000000000000000";
        let u = U80::from_hex_str(s).unwrap();
        assert_eq!(u.to_u128(), 0x40000000000000000000);

        let s2 = "40000080000000000000";
        let u = U80::from_hex_str(s2).unwrap();
        assert_eq!(u.to_u128(), 0x40000080000000000000);

        let s3 = "1E854F94";
        let u = U80::from_hex_str(s3).unwrap();
        assert_eq!(u.to_u128(), 0x1E854F94);
    }

    #[test]
    fn to_hex() {
        let key = "02030405060708090a0b";
        let a: u128 = 9500362842338723695115;
        let u = U80::from_u128(a);

        assert_eq!(u.to_hex_str(), key);
    }

    #[test]
    fn add() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(3);

        assert_eq!(a + b, U80::from_u128(6));
    }

    #[test]
    fn sub() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(2);

        assert_eq!(a - b, U80::from_u128(1));
    }

    #[test]
    fn wrapping_add() {
        let a = U80::from_u128(12);
        let b = U80::from_u128(1208925819614629174706172);
        assert_eq!((a + b).to_u128(), 8);
        let a = U80::from_u128(2);
        let b = U80::from_u128(1208925819614629174706175);
        assert_eq!((a + b).to_u128(), 1);
    }

    #[test]
    fn wrapping_sub() {
        let a = U80::from_u128(1);
        let b = U80::from_u128(2);

        assert_eq!((a - b).to_u128(), 1208925819614629174706175);
    }

    #[test]
    fn or() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(4);

        assert_eq!((a | b).to_u128(), 7);
    }
    #[test]
    fn xor() {
        let a = U80::from_u128(3);
        let b = U80::from_u128(5);

        assert_eq!(a ^ b, U80::from_u128(6));
    }
}

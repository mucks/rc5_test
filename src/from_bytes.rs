pub trait FromBytes {
    fn from_le_bytes(a: &mut &[u8]) -> Self;
}

impl<const N: usize> FromBytes for [u8; N] {
    fn from_le_bytes(a: &mut &[u8]) -> [u8; N] {
        let (int_bytes, rest) = a.split_at(N);

        let mut me = [0u8; N];
        me.copy_from_slice(int_bytes);

        *a = rest;
        me
    }
}

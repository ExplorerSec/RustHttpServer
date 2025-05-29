#[derive(Copy,Clone)]
pub enum CodecCrypto{
    Std,
}

pub fn en_crypto(codec: CodecCrypto, arr: [u8; 3]) -> [u8; 4] {
    match &codec {
        CodecCrypto::Std => en_crypto_std(arr),
    }
}

pub fn de_crypto(codec: CodecCrypto, arr: [u8; 4]) -> [u8; 3] {
    match &codec {
        CodecCrypto::Std => de_crypto_std(arr),
    }
}

fn en_crypto_std(arr: [u8; 3]) -> [u8; 4] {
    [
        (arr[0] >> 2),
        ((arr[0] & 0b11) << 4) | (arr[1] >> 4),
        (((arr[1] << 4) >> 2) | (arr[2] >> 6)),
        (arr[2] << 2) >> 2,
    ]
}
fn de_crypto_std(arr: [u8; 4]) -> [u8; 3] {
    [
        (arr[0] << 2) | ((arr[1]) >> 4),
        (arr[1] << 4) | (arr[2] >> 2),
        (arr[2] << 6) | arr[3],
    ]
}

#[cfg(test)]
mod test{
    use super::en_crypto_std;
    use super::de_crypto_std;
    #[test]
    fn test_std(){
        let s = "ABCDE示例文本".as_bytes();
        for i in [0,3,6]{
            let prime = s[i..i+3].try_into().unwrap();
            let en_txt = en_crypto_std(prime);
            for v in en_txt{
                assert!(v<64);
            }
            assert_eq!(de_crypto_std(en_txt),prime);
        }
    }

}

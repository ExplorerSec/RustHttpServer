use std::collections::HashMap;

mod codec_crypto;
use codec_crypto::{CodecCrypto,en_crypto,de_crypto};

pub struct Base64Codec {
    padding: char,
    table: Vec<u8>, // len == 64
    d_table: HashMap<char, u8>,
    codec:CodecCrypto
}

impl Base64Codec {
    pub fn new(s: &str, ch: char, codec:CodecCrypto) -> Base64Codec {
        let mut dtb = HashMap::new();
        let chars = s.chars();
        for (idx, c) in chars.enumerate() {
            dtb.insert(c, idx as u8);
        }
        assert_eq!(dtb.len(), 64);
        dtb.insert(ch, 0);
        Base64Codec {
            padding: ch,
            table: s.as_bytes().to_vec(),
            d_table: dtb,
            codec,
        }
    }
    pub fn default() -> Base64Codec {
        let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        Self::new(s, '=',CodecCrypto::Std)
    }
    pub fn web_default() -> Base64Codec {
        let s = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        Self::new(s, '=',CodecCrypto::Std)
    }
    pub fn encode(&self, mut vec:Vec<u8>) -> String {
        let mut num_padding = 0;
        while vec.len() % 3 != 0 {
            vec.push(0);
            num_padding += 1;
        }
        let len = vec.len() / 3;
        let mut v_out: Vec<u8> = Vec::with_capacity(len * 4);
        for i in 0..len {
            let arr = en_crypto(self.codec,vec[i*3..=i*3+2].try_into().unwrap());
            for val in arr{
                v_out.push(self.table[val as usize]);
            }
        }
        let mut s_out = String::from_utf8(v_out).unwrap();
        for _ in 0..num_padding {
            s_out.pop();
        }
        for _ in 0..num_padding {
            s_out.push(self.padding);
        }

        s_out
    }
    pub fn encode_str(&self, s: &str) -> String {
        let vec: Vec<u8> = s.as_bytes().to_vec();
        self.encode(vec)
    }
    pub fn decode(&self, s: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        let mut vec_out: Vec<u8> = Vec::new();
        let mut num_padding = 0;
        let chars = s.chars();
        for c in chars {
            if c == self.padding {
                num_padding += 1;
            }
            vec.push(self.d_table[&c]);
            if vec.len() == 4 {
                let arr = de_crypto(self.codec, vec[0..4].try_into().unwrap());
                for val in arr{
                    vec_out.push(val);
                }
                vec.clear();
            }
        }
        if vec.is_empty(){
            for _ in 0..(self.padding.len_utf8()*num_padding){
                vec_out.pop();
            }
            return vec_out;
        }
        Vec::from("[Error] Invalid Text")
    }

    pub fn decode_str(&self, s: &str) -> String{
        let vec = self.decode(s);
        if let Ok(s) = String::from_utf8(vec){
            s
        }else {
            String::from("[Error] Invalid String, Try fdf mode!")
        }
    }
}

#[cfg(test)]
mod test {
    use super::Base64Codec;
    #[test]
    fn test1_default() {
        let ct = Base64Codec::default();
        assert_eq!(
            ct.table,
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes()
        );
        assert_eq!(ct.padding, '=');
    }
    #[test]
    fn test2_en_utf8() {
        let ct = Base64Codec::default();
        assert_eq!(
            ct.encode_str("This is a Base64 Test"),
            "VGhpcyBpcyBhIEJhc2U2NCBUZXN0"
        );
        assert_eq!(
            ct.encode_str("这是一个进行编码的测试"),
            "6L+Z5piv5LiA5Liq6L+b6KGM57yW56CB55qE5rWL6K+V"
        );
    }
    #[test]
    fn test3_en_utf8_padding() {
        let ct = Base64Codec::default();
        assert_eq!(ct.encode_str("12345"), "MTIzNDU=");
        assert_eq!(ct.encode_str("padding"), "cGFkZGluZw==");
    }
    #[test]
    fn test4_de_utf8() {
        let ct = Base64Codec::default();
        assert_eq!(
            ct.decode_str("6L+b6KGM6Kej56CB5rWL6K+V").as_bytes(),
            "进行解码测试".as_bytes()
        );
        assert_eq!(
            ct.decode_str("WlhDVkJOTUxLSkhHRkRTQVFXRVJUWVVJT1A7").as_bytes(),
            "ZXCVBNMLKJHGFDSAQWERTYUIOP;".as_bytes()
        );
    }
    #[test]
    fn test4_de_utf8_padding() {
        let ct = Base64Codec::default();
        assert_eq!(ct.decode_str("QUJDRA==").as_bytes(), "ABCD".as_bytes());
    }
    #[test]
    fn test5_en_de() {
        let ct = Base64Codec::default();
        let s = "Base64 是一种基于 64 个可打印字符来表示二进制数据的表示方法，由于 2^6=64，所以每 6 个比特为一个单元，对应某个可打印字符。";
        assert_eq!(ct.decode_str(&ct.encode_str(s)), s);
    }
}

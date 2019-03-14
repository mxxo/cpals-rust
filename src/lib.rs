extern crate base64;
extern crate hex;

//use base64::{encode, decode};
//use hex::{encode, decode};

/// hex to base64 encoding, panics on hex read error
fn hex_2_b64(hex_str: &str) -> String {
    // panic on error
    base64::encode(&hex::decode(hex_str).unwrap())
}

fn fixed_xor(hex_str1: &str, hex_str2: &str) -> String {
    // try out lossy conversion fn
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Cryptopals challenge 1-1
    /// "Always operate on raw bytes, never encoded strings"
    fn test_hex_2_b64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_2_b64(&hex), b64);
    }

    #[test]
    /// Cryptopals challenge 1-2
    fn test_fixed_xor() {
        let h1 = "1c0111001f010100061a024b53535009181c";
        let h2 = "686974207468652062756c6c277320657965";

        assert_eq!(fixed_xor(&h1, &h2), "746865206b696420646f6e277420706c6179");
    }
}
